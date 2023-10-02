use std::{io, fs, process::Command};

pub fn listdir(path: &str) -> Vec<String> {
	match fs::read_dir(path) {
		Ok(entries) => {
			let file_names: Vec<String> = entries
				.filter_map(|entry| {
					entry.ok().and_then(|e| {
						e.file_name().into_string().ok()
					})
				})
				.collect();
			file_names
		}
		Err(err) => {
			eprintln!("Erreur lors de la lecture du répertoire : {:?}", err);
			vec![]
		}
	}
}
pub fn input(text: &str) -> String {
	let mut input: String = Default::default();
	println!("{}",text);
	let _ = io::stdin().read_line(&mut input);
	input[..input.len()-1].to_string()
}
pub fn clear() {
	match Command::new("clear").status() {
		Ok(_) => {},
		Err(_) => {
			match Command::new("cl").status() {
				Ok(_) => {},
				Err(_) => {
					for _ in 0..100 {
                        println!()
                    }
				},
			}
		},
	}
}
