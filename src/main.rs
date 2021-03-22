mod io;

fn main() -> std::io::Result<()> {
    let mut reader = io::IO::new()?;

    while let Ok(line) = reader.next() {
        match line {
            io::Input::Line(s) => println!("{}", s),
            io::Input::Empty => continue,
            io::Input::EOF => break,
        }
    }

    Ok(())
}
