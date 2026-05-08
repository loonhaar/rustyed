use std::io::{self, Write};

fn main() -> io::Result<()> {
	let result = App::default().run();
	result
}

#[derive(Debug, Default)]
struct App {
	buffer: Vec<String>,
	caret_address: usize,
	modified_buffer: bool,
	mode: Mode,
}

#[derive(Debug, Default)]
enum Mode {
	#[default]
	Cmd,
	Edit,
}

impl App {
	fn run(mut self) -> io::Result<()> {
		let mut line = String::new();

		loop {
			line.clear();

			if io::stdin().read_line(&mut line)? == 0 {
				break;
			}

			let input = line.trim();

			match self.mode {
				Mode::Cmd => {
					if self.handle_cmd(input) {
						break;
					}
				}
				_ => {}
			}
		}

		Ok(())
	}

	fn handle_cmd(&mut self, input: &str) -> bool {
		match input {
			"q" => true,
			_ => {
				println!("?");
				false
			}
		}
	}

	fn handle_edit() {}
}
