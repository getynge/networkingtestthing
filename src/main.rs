use std::net;
use std::io;
use std::io::Write;

fn big_spoon(conn:net::UdpSocket) {
    println!("Successfully begun announcing");
    println!("Waiting for data to be sent...");
    let mut msg:[u8;255] = [0;255];
    conn.recv(&mut msg).unwrap();
    let message = String::from_utf8_lossy(&msg);
    println!("Received: {}", message);
}

fn little_spoon(conn:net::UdpSocket){
    println!("Please type in the message you would like to send (max len 255)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    conn.send(input.as_bytes()).expect("Error sending data to the big spoon");
    println!("Data sent");
}

fn main() {
    println!("Welcome to network test");
    println!("Please type in a port to connect to below");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("Beginning to establish tcp connection at address {}...", input.trim());

    if let Ok(conn) = net::UdpSocket::bind(input.as_str()) {
        println!("Bound to socket successfully");
        little_spoon(conn);
    }else{
        input = String::new();
        println!("Error: you either did not type a valid IP or the socket failed to bind");
        print!("Would you like to attempt to announce a connection instead? (y/n) ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            big_spoon(net::UdpSocket::bind("127.0.0.1:16501").expect("Could not bind localhost"));
        }else{
            println!("exiting...");
        }
    }
}
