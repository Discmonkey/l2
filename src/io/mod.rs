use std::io::Result;
use linefeed;
use linefeed::ReadResult;

pub struct IO {
    io: linefeed::Interface<linefeed::DefaultTerminal>
}

pub enum Input {
    EOF,
    Empty,
    Line(String),
}

impl IO {
    pub fn new() -> Result<Self> {
        let io = linefeed::Interface::new("l2")?;

        io.set_prompt("l2>>> ")?;

        Ok(IO{ io })
    }

    pub fn next(&mut self) -> Result<Input> {
        match self.io.read_line()? {
            ReadResult::Eof => Ok(Input::EOF),

            ReadResult::Input(line) => {
                let trimmed = line.trim();

                if trimmed.len() > 0 {
                    self.io.add_history(trimmed.to_string());

                    Ok(Input::Line(trimmed.to_string()))
                } else {
                    Ok(Input::Empty)
                }
            }

            ReadResult::Signal(_) => Ok(Input::EOF),
        }
    }
}