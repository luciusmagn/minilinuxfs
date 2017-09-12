#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate pbr;

use std::path::Path;
use std::fs::create_dir;
use std::process::{exit, Command};

use pbr::ProgressBar;

mod cfg;

fn main()
{
	let cfg = if let Ok(cfg) = cfg::Config::read()
	{
		cfg.unpack()
	}
	else
	{
		eprintln!("failed to parse config");
		exit(-2);
	};

	if !Path::new(&cfg.src_dir).exists()
	{
		create_dir(&cfg.src_dir).expect("couldn't create git source directory");
	}

	let mut pb = ProgressBar::new(cfg.entries.len() as u64);
	
	println!("cloning repositories:");

	for entry in &cfg.entries
	{
		pb.message(&entry.name);

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
}
