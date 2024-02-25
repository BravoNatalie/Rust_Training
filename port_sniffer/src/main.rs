use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;

// Max IP Port.
const MAX: u16 = 65535;

// Address fallback.
const IP_FALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

// CLI Arguments.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    #[bpaf(long, short, argument("Address"), fallback(IP_FALLBACK))]
    /// The address that you want to sniff.  Must be a valid ipv4 address.  Falls back to 127.0.0.1
    pub address: IpAddr,

    #[bpaf(
        long("start"),
        short('s'),
        guard(start_port_guard, "Must be greater than 0"),
        fallback(1u16)
    )]
    /// The start port for the sniffer. (must be greater than 0)
    pub start_port: u16,

    #[bpaf(
        long("end"),
        short('e'),
        guard(end_port_guard, "Must be less than or equal to 65535"),
        fallback(MAX)
    )]
    /// The end port for the sniffer. (must be less than or equal to 65535)
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}


async fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, start_port)).await {
        // If the connection is successful, print out a . and then pass the port through the channel.
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(start_port).unwrap();
        }
        // If the connection is unsuccessful, do nothing. Means port is not open.
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let opts = arguments().run();
    let (tx, rx) = channel();

    // Iterate through all of the ports (based on user input) so that we can spawn a single task for each.
    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();

        task::spawn(async move { scan(tx, i, opts.address).await });
    }


    let mut out = vec![];
    // Drop the tx clones.
    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}