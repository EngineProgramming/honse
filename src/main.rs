use std::io;
mod chess;
mod search;
mod uci;

fn main() -> io::Result<()> {
    if std::env::args().nth(1).as_deref() == Some("bench") {
        crate::uci::bench::bench();
        return Ok(());
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.as_str().trim() {
        "uci" => crate::uci::listen::listen(),
        _ => println!("Unknown protocol"),
    };

    Ok(())
}
