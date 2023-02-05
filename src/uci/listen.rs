use crate::uci::{perft, position, ucinewgame};

pub fn listen() {
    println!("id name Honse");
    println!("id author EPD");
    println!("uciok");

    let mut board = cozy_chess::Board::startpos();

    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }

        let mut stream = input.split_ascii_whitespace();

        match stream.next().unwrap_or("") {
            "ucinewgame" => ucinewgame::ucinewgame(&mut board),
            "position" => position::position(&mut stream, &mut board),
            "perft" => perft::perft(&mut stream, &mut board),
            "quit" => break,
            _ => {}
        }
    }
}
