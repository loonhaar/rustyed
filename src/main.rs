use std::io;

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
				Mode::Edit => self.handle_edit(&input),
			}
		}

		Ok(())
	}

	fn handle_cmd(&mut self, input: &str) -> bool {
		match input {
			"q" => {
				if self.modified_buffer {
					println!("?");
					// Next line is for the force quit functionality. If the buffer is not saved
					// user must quit twice.
					self.modified_buffer = false;
					false
				} else {
					true
				}
			}
			"a" => {
				self.mode = Mode::Edit;
				false
			}
			"l" => {
				if self.buffer.len() == 0 {
					println!("?");
				} else {
					for line in &self.buffer {
						println!("{}", line);
					}
				}

				false
			}
			_ => {
				println!("?");
				false
			}
		}
	}

	fn handle_edit(&mut self, input: &str) {
		if input == "." {
			self.mode = Mode::Cmd;
		} else {
			self.buffer.push(input.to_string());
			self.caret_address = self.buffer.len();
			self.modified_buffer = true;
		}
	}
}
