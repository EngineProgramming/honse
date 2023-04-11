use crate::{
    search::search::SearchInfo,
    uci::{go, options, perft, position, split, ucinewgame},
};
use cozy_chess::Board;

pub fn listen() {
    println!("id name Honse");
    println!("id author EPD");
    options::print_options();
    println!("uciok");

    let mut board = Board::default();
    let mut search_info = SearchInfo::new();

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
            "split" => split::split(&mut stream, &mut board),
            "go" => go::go(&mut stream, &mut search_info, &board),
            "isready" => println!("readyok"),
            "quit" => break,
            _ => {}
        }
    }
}
