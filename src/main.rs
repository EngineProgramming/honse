use std::io;
mod uci;

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.as_str().trim() {
        "uci" => crate::uci::listen::listen(),
        _ => println!("Unknown protocol"),
    };

    Ok(())
}
