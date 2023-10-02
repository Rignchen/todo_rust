/*
Made by Nils H
todolist in rust
11.09.23 -> 15.09.23
*/

use std::{io::{self, BufRead,Write}, fs::{self,File}};
use todo_rust::{*};

fn main() {
	//Define global variables
	let mut inp: &str;
	let mut inpu: String;
	let mut running: bool = true;
	let mut last_inp: String = "h".to_string();
	let (mut todos,mut archive): (Vec<String>, Vec<String>) = load("task");

	start(&todos);

	// main loop
	while running {
		inpu = input("input:").to_lowercase();
		inp = inpu.trim();
		if command(inp, &mut running, &mut todos, &mut archive) {last_inp = inp.to_string()} // commands -> store in "last_command"
		else {																									 // commands -> not store in "last_command"
			match inp {
				"c"|"cls"|"clear" => clear(),
				""|"last" => {command(last_inp.as_str(), &mut running, &mut todos, &mut archive);},
				"q"|"quick_start"|"quickstart"|"quick start"|"restart"|"reset" => start(&todos),
				_ => {
					match inp.parse::<usize>() {
						Ok(parsed_value) => {
							if parsed_value < todos.len() {archive.push(todos.remove(parsed_value))}
							else {println!("No task with id {} found",parsed_value)}
						}
						Err(_) => {
							println!("Unknow command \"{}\"",inp);
						}
					}
				}
			}
		}
	}
}
fn command(command_input: &str,mut _run: &mut bool, todos: &mut Vec<String>, archive: &mut Vec<String>) -> bool { // commands -> can be store in "last_command"
	match command_input {
		"h"|"help" => println!("\n\nLast: repeat last input\nAny number: Archive the associated task\n\n---------\n\nThese commands can be input as 1 letter:\n\nHelp: Show this message\nClear: Clesr the shell\nExit: Close the app\nQuick_start: Clear the shell and show the first 5 task\nNew: Add a new task\nUnarchive: Take a task back from the archive\nDelet: Definitly remove an archived task\nArchived: Show every archived task\nTodos: Show every unfinished task\nLoad: Load task from an external file\nSave: Save task in an external file\n\n"),
		"n"|"new" => {
			let temp = input("What's task do you want to add?");
			if temp.trim() != "" {todos.push(temp.trim().to_string())};
			show_values(&todos, 5);
		},
		"d"|"delet"|"erase" => {
			println!("");
			show_values(&archive, archive.len());
			match input("Wich archoved task do you wanna delet?").trim().parse::<usize>() {
				Ok(parsed_value) => {
					if parsed_value < archive.len() {archive.remove(parsed_value);}
					else {println!("No task with id {} found",parsed_value)}
				}
				Err(err) => {
					println!("An error occured: {:?}",err);
				}
			}
		},
		"u"|"unarchive"|"retrieve" => {
			println!("");
			show_values(&archive, archive.len());
			match input("Wich archoved task do you wanna unarchive?").trim().parse::<usize>() {
				Ok(parsed_value) => {
					if parsed_value < archive.len() {todos.push(archive.remove(parsed_value));}
					else {println!("No task with id {} found",parsed_value)}
				}
				Err(err) => {println!("An error occured: {:?}",err);}
			}
		},
		"s"|"save"|"backup" => {
			let temp: String = input("In wich file do you wana save the data?");
			let mut temp: &str = temp.trim();
			if temp == "" {temp = "task";}
			save(temp,todos,archive);
			println!("Backup made");
		},
		"l"|"load" => {
			if input("Are you sure you want to load a file? All unsaved modification will be lost [y/n]").to_lowercase()=="y" {
				for file_name in listdir("data/") {print!("{}", file_name + "    ");}
				println!();
				let temp = input("From wich file do you wana import the data?");
				clear();
				let mut temp: &str = temp.trim();
				if temp == "" {temp = "task";}
				(*todos, *archive) = load(temp);
				show_values(&todos, 5);
			}
		},
		"a"|"archived"|"archive" => {show_values(archive, archive.len());},
		"t"|"all"|"todos"|"todo"|"show" => {show_values(todos, todos.len());},
		"e"|"esc"|"exit" => *_run = false,
		_ => return false,
	}
	true
}
fn start(todos: &[String]) {
	clear();
	if !show_values(&todos, 5) {clear()};
}
fn show_values(list: &[String], mut amount: usize) -> bool {
	if amount > list.len() {amount = list.len()}
	if amount > 0 {
		for i in 0..amount {
			println!("{}. {:?}",i, list[i])
		}
		return true;
	}
	println!("The list is empty");
	false
}
fn save(file_name: &str, todos: &[String], archive: &[String]) {
	let open = File::create("data/".to_string() + file_name + ".md");

	match open {
		Ok(mut file) => {
			for s in todos {
				let _ = file.write_all(b"- [ ] ");
				let _ = file.write_all(s.as_bytes());
				let _ = file.write_all(b"\n");
			}
			for s in archive {
				let _ = file.write_all(b"- [X] ");
				let _ = file.write_all(s.as_bytes());
				let _ = file.write_all(b"\n");
			}
		},
		Err(err) => {
			println!("Error while opening file: {:?}",err)
		},
	}
}
fn load(file_name: &str) -> (Vec<String>,Vec<String>) {
	let (mut todos, mut archive): (Vec<String>, Vec<String>) = (vec![], vec![]);
	match fs::File::open("data/".to_string() + file_name + ".md") {
		Ok(file) => {
			let reader = io::BufReader::new(file);
			
			// Lecture des lignes du fichier dans un vecteur de chaînes de caractères
			for line in reader.lines() {
				match line {
					Ok(text) => {
						if text[..5] == *"- [ ]" {todos.push(text[5..].trim().to_string())}
						else if text[..5] == *"- [X]" {archive.push(text[5..].trim().to_string())}
						else {println!("WARNING! unable to sort line \"{}\" through either todos or archive",text)};
					},
					Err(err) => {println!("An error occured: {:?}",err);}
				}
			}
		},
		Err(err) => {
			println!("Error while opening file: {:?}",err)
		},
	} 
	(todos,archive)
}

