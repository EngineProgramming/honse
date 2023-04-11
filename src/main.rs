use std::io;
mod chess;
mod search;
mod uci;

fn main() -> io::Result<()> {
    std::env::args().nth(1).map_or_else(
        || (),
        |arg| match arg.as_str() {
            "bench" => crate::uci::bench::bench(),
            _ => (),
        },
    );

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.as_str().trim() {
        "uci" => crate::uci::listen::listen(),
        _ => println!("Unknown protocol"),
    };

    Ok(())
}
