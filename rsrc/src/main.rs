#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate pbr;

use std::path::Path;
use std::process::{exit, Command};
use std::fs::{create_dir, remove_dir_all, copy};
use std::env::{args, set_current_dir, current_dir};

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
	
	println!("cloning repositories:");

	for entry in &cfg.entries
	{
		let cmd = Command::new("git")
			.arg("clone")
			.arg("-q")
			.arg(&entry.url)
			.arg(Path::new(&cfg.src_dir).join(&entry.name).into_os_string())
			.status()
			.expect("failed to execute process");

		if !cmd.success()
		{
			pb.finish_println(&format!("error during cloning {}, aborting...", &entry.name));
			exit(-3);
		}

		pb.inc();
	}

	println!("building:");

	let absolute_src = orig_cwd.join(Path::new(&cfg.src_dir));
	let absolute_dest = orig_cwd.join(Path::new(&cfg.output));

	for entry in &cfg.entries
	{
		println!("{}", &entry.name);

		for c in &entry.cmds
		{
			set_current_dir(&absolute_src.join(Path::new(&entry.name)))
				.expect("couldn't change cwd");
			let cmd = Command::new(&c)
				.status()
				.expect("failed to execute process");
			if !cmd.success()
			{
				println!("error during cloning {}, aborting...", &entry.name);
				exit(-4);
			}
			set_current_dir(&orig_cwd)
				.expect("couldn't change cwd");
		}
	}

	println!("installing:");

	copy(&Path::new(&cfg.input), &Path::new(&cfg.output))
		.expect("couldn't copy input directory");

	for entry in &cfg.entries
	{
		println!("{}", &entry.name);

		for &(ref src, ref dest) in &entry.paths
		{
			copy(
				&absolute_src.join(Path::new(&entry.name).join(Path::new(src))),
				&absolute_dest.join(Path::new(dest))
			).expect(format!("could not copy {} to {}", src, dest).as_ref());
		}

		if let Some(ref install) = entry.install
		{
			set_current_dir(&absolute_src.join(Path::new(&entry.name)))
				.expect("couldn't change cwd");
			for c in install
			{
				let cmd = Command::new(&c.replace("{{target}}", &absolute_dest.to_str().unwrap()))
					.status()
					.expect("failed to execute process");
				if !cmd.success()
				{
					println!("error during installing {}, aborting...", &entry.name);
					exit(-4);
				}
			}
			set_current_dir(&orig_cwd)
				.expect("couldn't change cwd");
		}
	}

	println!("done");
}
