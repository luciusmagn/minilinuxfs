use std::process::exit;
use std::io::Read;
use std::fs::File;

use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config
{
	pub input: Option<String>,
	pub output: Option<String>,
	pub src_dir: Option<String>,
	pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry
{
	pub url: String,
	pub name: String,
	pub cmds: Vec<String>,
	pub paths: Vec<(String, String)>,
	pub install: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct UnpackedConfig
{
	pub input: String,
	pub output: String,
	pub src_dir: String,
	pub entries: Vec<Entry>,
}

#[allow(dead_code)]
impl Config
{
	pub fn read() -> Result<Config, toml::de::Error>
	{
		let mut me = match File::open("cfg.toml")
		{
			Ok(f) => f,
			Err(_) => {eprintln!("failed to open cfg.toml"); exit(-1)}
		};
		let mut contents = String::new();
		if me.read_to_string(&mut contents).is_err()
			{eprintln!("failed to read cfg.toml"); exit(-1)}
		toml::from_str(contents.as_ref())
	}

	pub fn unpack(self) -> UnpackedConfig
	{
		UnpackedConfig
		{
			input:   if let Some(ref s) = self.input {s.clone()}
				else
				{
					"inputfs".to_string()
				},
			output:  if let Some(ref s) = self.output {s.clone()}
				else
				{
					"fs".to_string()
				},
			src_dir: if let Some(ref s) = self.src_dir {s.clone()}
				else
				{
					"git".to_string()
				},
			entries: self.entries.clone()
		}
	}
}
