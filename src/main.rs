use std::net;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn server_mode(conn:net::UdpSocket) {
    println!("Waiting for data to be sent...");
    let mut msg:[u8;255] = [0;255];
    conn.recv(&mut msg).unwrap();
    let message = String::from_utf8_lossy(&msg);
    println!("Received: {}", message);
}

fn client_mode(target: net::SocketAddr){
    let socket = net::UdpSocket::bind("127.0.0.1:12345").unwrap();
    let mut input = String::new();

    println!("Please type your message (limit 255 characters)");
    io::stdin().read_line(&mut input).unwrap();

    socket.send_to(input.as_bytes(), target).expect("Failed to send");
}

fn main() {
    println!("Welcome to network test");
    println!("Please type in an address to connect to below");
    println!("If you would like to be the big spoon, just type 0");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input == "0" {
        input = String::new();
        print!("Would you like to attempt to announce a connection instead? (y/n) ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            println!("Attempting to announce on ports 60000 65535");
            let ip = net::Ipv4Addr::new(0, 0, 0, 0);
            'traverse: for i in 60000..65535 {
                let addr = net::SocketAddrV4::new(ip, i);
                if let Ok(conn) = net::UdpSocket::bind(addr) {
                    println!("Successfully begun announcing on port {}", i);
                    server_mode(conn);
                    break 'traverse;
                }
            }
        } else {
            println!("exiting...");
        }
    }else{
        client_mode(net::SocketAddr::from_str(input.trim()).unwrap())
    }
}
