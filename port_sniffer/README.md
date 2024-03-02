## Port Sniffer

A simple cli command that looks for open ports.

### :joystick: How to Use

#### Requirements

To run the application you'll need:

- Rust and Cargo
- - Clone the repository:
  - `$ git clone https://github.com/BravoNatalie/Rust_Training.git`

Now go to project folder and run the app:

```bash
$ cd port_sniffer
$ cargo run -- -h # to get the available options
$ cargo run -- -a 127.0.0.1 -s 80 -e 5000 # example
```
