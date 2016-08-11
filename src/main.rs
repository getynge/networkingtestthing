use std::net;
use std::io;
use std::io::Write;

//The "big spoon" receives data from the little spoon
fn big_spoon(conn:net::UdpSocket) {
    println!("Waiting for data to be sent...");
    let mut msg:[u8;255] = [0;255];
    conn.recv(&mut msg).unwrap();
    let message = String::from_utf8_lossy(&msg);
    println!("Received: {}", message);
}

//The "little spoon" sends data to the big spoon
fn little_spoon(conn:net::UdpSocket){
    println!("Please type in the message you would like to send (max len 255)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    conn.send(input.as_bytes()).expect("Error sending data to the big spoon");
    println!("Data sent");
}

fn main() {
    println!("Welcome to network test");
    println!("Please type in an address to connect to below");
    println!("If you would like to be the big spoon, just type 0");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("Beginning to establish tcp connection at address {}...", input.trim());

    if let Ok(conn) = net::UdpSocket::bind(input.as_str()) {
        println!("Bound to socket successfully");
        little_spoon(conn);
    }else{

        input = String::new();
        println!("None of the available ports were announcing");
        print!("Would you like to attempt to announce a connection instead? (y/n) ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            println!("Attempting to announce on ports 60000 65535");
            let ip = net::Ipv4Addr::new(0, 0, 0, 0);
            'traverse:for i in 60000..65535 {
                let addr = net::SocketAddrV4::new(ip, i);
                if let Ok(conn)  = net::UdpSocket::bind(addr) {
                    println!("Successfully begun announcing on port {}", i);
                    big_spoon(conn);
                    break'traverse;
                }
            }
        }else{
            println!("exiting...");
        }
    }
}
