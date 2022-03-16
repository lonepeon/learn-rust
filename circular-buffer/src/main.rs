use circular_buffer::CircularBuffer;

fn main() {
    let matches = clap::Command::new("circular-buffer")
        .arg(
            clap::Arg::new("size")
                .short('s')
                .long("size")
                .takes_value(true)
                .help("buffer size"),
        )
        .get_matches();

    let size = matches.value_of("size").unwrap_or("10");
    let size = size
        .parse::<usize>()
        .expect("invalid size flag value: expecting a positive integer");

    let mut buf = CircularBuffer::new(size);

    let mut line = String::new();
    loop {
        let rst = std::io::stdin().read_line(&mut line);
        match rst {
            Ok(0) => break,
            Err(e) => eprintln!("error: {}", e),
            Ok(_) => {}
        }

        buf.push(line.trim().to_string());
        line.clear();
    }

    for line in buf.to_vector() {
        println!("line: {}", line);
    }
}
