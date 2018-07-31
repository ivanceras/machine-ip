extern crate machine_ip;

fn main() {
    println!(
        "{}",
        machine_ip::get()
            .expect("Could not find local IP address.")
            .to_string()
    );
}
