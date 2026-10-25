// factd: the "Panic as a Service" network service from docs/panic.txt.
//
// It listens on localhost port 3000 for TCP connections and gives each client its own
// thread. A client sends a number on each line and gets its factorial back.
//
// Some inputs make the client's thread panic:
//     a number above 20  ->  "attempt to multiply with overflow" (debug builds)
//     text that isn't a number  ->  unwrap() on an Err
// With the default `panic = "unwind"`, only that client's thread stops and its connection
// closes; other clients keep being served. With `panic = "abort"` in Cargo.toml, the first
// panic stops the whole server.
//
// Run it:  cargo run --bin factd
// Connect from another terminal, e.g. with netcat:  nc localhost 3000
//
// The transcript stops just after the call to bind(), so the code after that is my own
// version of what the video describes, not a copy of the video's slides.

// Bring the networking, I/O and threading names we use into scope.
use std::io::{BufRead, BufReader, Write}; // reading lines and writing text over a stream
use std::net::{TcpListener, TcpStream}; // TCP server socket and one client connection
use std::thread; // spawning threads

// The service has two functions: main, and handle_client below.
// main returns a Result: Ok(()) on success, or an I/O error, which makes error handling easier.
fn main() -> std::io::Result<()> {
    // Listen for TCP connections on localhost port 3000.
    // bind() returns a Result. The `?` means: if it's an error (say the port is already in
    // use), return that error from main right away; otherwise hand back the listener.
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    // Let the user know the server is up.
    println!("factd listening on 127.0.0.1:3000");

    // incoming() waits for a client to connect, gives us that connection, and repeats forever.
    for stream in listener.incoming() {
        // Each connection attempt is a Result; `?` stops the server if accepting fails.
        let stream = stream?;
        // Start a new thread for this client. `move` hands ownership of the stream to the
        // thread, and the code between the || and the end of the block runs on that thread.
        thread::spawn(move || {
            // Run the client's session and report any I/O error instead of ignoring it.
            if let Err(e) = handle_client(stream) {
                eprintln!("client error: {}", e); // eprintln! prints to stderr
            }
        });
        // main doesn't wait for the thread: it loops back and waits for the next client.
    }
    Ok(()) // never reached, because incoming() never runs out
}

// Serves one client: read a number per line, write back its factorial.
fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let peer = stream.peer_addr()?; // the client's address, for log messages
    println!("{} connected", peer); // log the new connection

    let mut writer = stream.try_clone()?; // a second handle on the same connection, for writing
    let reader = BufReader::new(stream); // buffers the input so we can read it line by line

    // lines() yields each line the client sends, until they disconnect.
    for line in reader.lines() {
        let line = line?; // stop on a read error
        // trim() strips spaces and the line ending; parse() turns the text into a u64.
        // unwrap() panics if it isn't a number, which ends this thread.
        let n: u64 = line.trim().parse().unwrap();
        // Calculate and send the answer. factorial panics if n is bigger than 20.
        writeln!(writer, "{}", factorial(n))?;
    }

    println!("{} disconnected", peer); // only reached if the client leaves without a panic
    Ok(()) // success
}

// Iterative factorial, the same as the exercise solution in src/bin/panic.rs.
fn factorial(n: u64) -> u64 {
    let mut x = 1; // running product
    for i in 2..=n {
        // multiply by 2, 3, ..., n
        x *= i; // panics on overflow in a debug build
    }
    x // return the product
}
