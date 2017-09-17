#[macro_use]
extern crate serde_derive;
extern crate ansi_term;
extern crate toml;
extern crate pbr;

use std::path::Path;
use std::process::{exit, Command};
use std::fs::{create_dir, remove_dir_all, copy};
use std::env::{args, set_current_dir, current_dir};

use ansi_term::Colour::Green;
use pbr::ProgressBar;

mod cfg;

fn main()
{
	let orig_cwd = current_dir().expect("couldn't get current working directory");
	let args: Vec<_> = args().collect();
	let cfg = match cfg::Config::read()
	{
		Ok(cfg) => cfg.unpack(),
		Err(e) =>
		{
			eprintln!("failed to parse config: {}", e);
			exit(-2)	
		},
	};
	
	if args.len() > 1 && args[1] == "clean"
	{
		println!("cleaning up");
		let _ = remove_dir_all(Path::new(&cfg.src_dir));
		let _ = remove_dir_all(Path::new(&cfg.output));
		exit(0);
	}

	if !Path::new(&cfg.src_dir).exists()
	{
		create_dir(&cfg.src_dir).expect("couldn't create git source directory");
	}

	let mut pb = ProgressBar::new(cfg.entries.len() as u64);
	
	println!("{}",
		Green.paint("cloning repositories:")
	);

	for entry in &cfg.entries
	{
		let _ = remove_dir_all(
			Path::new(&cfg.src_dir)
			.join(&entry.name)
			.into_os_string()
		);
		let cmd = Command::new("git")
			.arg("clone")
			.arg("-q")
			.arg(&entry.url)
			.arg(Path::new(&cfg.src_dir).join(&entry.name).into_os_string())
			.status()
			.expect("failed to execute process: git");

		if !cmd.success()
		{
			pb.finish_println(&format!("error during cloning {}, aborting...", &entry.name));
			exit(-3);
		}

		pb.inc();
	}

	println!("{}",
		Green.paint("building:")
	);

	let absolute_src = orig_cwd.join(Path::new(&cfg.src_dir));
	let absolute_dest = orig_cwd.join(Path::new(&cfg.output));

	for entry in &cfg.entries
	{
		println!("{}", &entry.name);

		set_current_dir(&absolute_src.join(Path::new(&entry.name)))
			.expect("couldn't change cwd");
		for c in &entry.cmds
		{
			println!("exec: {} {:?}", c.clone().split_whitespace().next().unwrap(), c.clone().split_whitespace().skip(1).collect::<Vec<_>>());
			
			let cmd = Command::new(&c.clone().split_whitespace().next().unwrap())
				.args(c.clone().split_whitespace().skip(1))
				.status()
				.expect(&format!("failed to execute process: {}", &c));
			if !cmd.success()
			{
				eprintln!("error during cloning {}, aborting...", &entry.name);
				exit(-4);
			}
		}
		set_current_dir(&orig_cwd)
			.expect("couldn't change cwd");
	}

	println!("{}",
		Green.paint("installing:")
	);

	Command::new("cp")
		.arg("-a")
		.arg(&cfg.input)
		.arg(&absolute_dest)
		.status()
		.expect("couldn't copy input directory");

	for entry in &cfg.entries
	{
		println!("{}", Green.paint(entry.name.clone()));

		for &(ref src, ref dest) in &entry.paths
		{
			copy(
				&absolute_src.join(Path::new(&entry.name).join(Path::new(src))),
				&absolute_dest.join(Path::new(dest).join(Path::new(src).file_name().unwrap()))
			).expect(format!("could not copy {} to {}", src, dest).as_ref());
		}

		if let Some(ref install) = entry.install
		{
			set_current_dir(&absolute_src.join(Path::new(&entry.name)))
				.expect("couldn't change cwd");
			for c in install
			{
				let cr = c.replace("<<target>>", absolute_dest.to_str().unwrap());
				let cmd = Command::new(&cr.clone().split_whitespace().next().unwrap())
					.args(cr.clone().split_whitespace().skip(1))
					.status()
					.expect(&format!("failed to execute process: {}",
						&c.replace("<<target>>", absolute_dest.to_str().unwrap()))
					);
				if !cmd.success()
				{
					eprintln!("error during installing {}, aborting...", &entry.name);
					exit(-5);
				}
			}
			set_current_dir(&orig_cwd)
				.expect("couldn't change cwd");
		}
	}

	println!("done");
}
