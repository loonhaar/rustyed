use std::{fs, io, path::Path};

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
		if let Some(filename) = input.strip_prefix("w ") {
			return self.write_cmd(filename);
		}

		let cmd_char = input.chars().last().unwrap_or(' ');
		let range = &input[..input.len().saturating_sub(1)];

		let mut parts = range.split(',');
		let start_str = parts.next().unwrap_or("");
		let stop_str = parts.next().unwrap_or("");

		let start: usize = start_str.parse().unwrap_or(self.current_address);
		let stop: usize = stop_str.parse().unwrap_or(start);

		match cmd_char {
			'q' => self.quit_cmd(),
			'a' => self.append_cmd(),
			'l' => self.list_cmd(start, stop),
			'd' => self.delete_cmd(start, stop),
			_ => self.unnknown_cmd(),
		}
	}

	fn quit_cmd(&mut self) -> bool {
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

	fn append_cmd(&mut self) -> bool {
		self.mode = Mode::Edit;
		false
	}

	fn list_cmd(&mut self, start: usize, stop: usize) -> bool {
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

	fn delete_cmd(&mut self, start: usize, stop: usize) -> bool {
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

	fn write_cmd(&mut self, filename: &str) -> bool {
		let grimey_name = filename
			.chars()
			.any(|c| !c.is_alphanumeric() && !"_-. ".contains(c));

		if grimey_name || filename.is_empty() || filename.len() > 255 {
			println!("?");
			return false;
		}

		let path = Path::new(filename);

		if path.components().count() != 1 {
			println!("?");
			return false;
		}

		let _ = fs::File::options()
			.write(true)
			.create_new(true)
			.open(path)
			.map(|_| {
				if fs::write(path, self.buffer.join("\n")).is_err() {
					println!("?");
				}
			})
			.unwrap_or_else(|_| {
				println!("?");
			});

		self.modified_buffer = false;
		false
	}

	fn unnknown_cmd(&mut self) -> bool {
		println!("?");
		false
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
