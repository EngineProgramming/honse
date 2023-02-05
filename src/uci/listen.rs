pub fn listen() {
    println!("id name Honse");
    println!("id author EPD");
    println!("uciok");

    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }

        let mut stream = input.split_ascii_whitespace();

        if stream.next().unwrap_or("") == "quit" {
            break;
        }
    }
}
