use std::io;

fn main() -> io::Result<()> {
	let result = App::default().run();
	result
}

#[derive(Debug, Default)]
struct App {
	buffer: Vec<String>,
	current_address: usize,
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
		let cmd_char = input.chars().last().unwrap_or(' ');
		let range = &input[..input.len().saturating_sub(1)];

		let mut parts = range.split(',');
		let start_str = parts.next().unwrap_or("");
		let stop_str = parts.next().unwrap_or("");

		let start: usize = start_str.parse().unwrap_or(self.current_address);
		let stop: usize = stop_str.parse().unwrap_or(start);

		match cmd_char {
			'q' => {
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
			'a' => {
				self.mode = Mode::Edit;
				false
			}
			'l' => {
				if start > 0 && stop <= self.buffer.len() && start <= stop {
					for i in (start - 1)..stop {
						println!("{}$", self.buffer[i]);
					}
					self.current_address = stop;
				} else {
					println!("?");
				}

				false
			}
			'd' => {
				if start > 0 && stop <= self.buffer.len() && start <= stop {
					self.buffer.drain((start - 1)..stop);
					self.current_address = if self.buffer.is_empty() {
						0
					} else {
						start.min(self.buffer.len())
					};
					self.modified_buffer = true;
				} else {
					println!("?");
					return false;
				}

				false
			}
			'w' => {

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
			self.current_address = self.buffer.len();
			self.modified_buffer = true;
		}
	}
}
