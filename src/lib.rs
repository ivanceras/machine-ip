use std::process::Command;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn get() -> Option<IpAddr> {
    let output = Command::new("hostname")
        .args(&["-I"])
        .output()
        .expect("failed to execute `hostname`");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let ips: Vec<&str> = stdout.trim().split(" ").collect::<Vec<&str>>();
    let first = ips.first();
    match first{
        Some(first) =>  {
            if !first.is_empty(){
                if let Ok(addr) = first.parse::<Ipv4Addr>() {
                    return Some(IpAddr::V4(addr))
                }
                else if let Ok(addr) = first.parse::<Ipv6Addr>() {
                    return Some(IpAddr::V6(addr))
                }
                else{
                    None
                }
            }else{
                None
            }
        }
        None => None
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let res = get();
        println!("res: {:?}", res);
        assert!(res.is_some());
    }
}
