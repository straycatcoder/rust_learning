use std::net::{IpAddr, UdpSocket};
use std::process::Command;

fn get_hostname() -> Option<String> {
    let output = Command::new("hostname")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let hostname = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Some(hostname)
    } else {
        None
    }
}

fn get_local_ip() -> Option<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let local_addr = socket.local_addr().ok()?;
    Some(local_addr.ip())
}

fn get_public_ip() -> Option<IpAddr> {
    let response = reqwest::blocking::get("https://api.ipify.org").ok()?;
    let ip_str = response.text().ok()?;
    ip_str.parse().ok()
}   

fn main() {
    let hostname: Option<String> = get_hostname();
    match hostname {
        Some(name) => println!("Hostname: {}", name),
        None => println!("Could not retrieve hostname"),    
    } 

    let local_ip: Option<IpAddr> = get_local_ip();
    match local_ip {
        Some(ip) => println!("Local IP Address: {}", ip),
        None => println!("Could not retrieve local IP address"),
    }
    
    let public_ip: Option<IpAddr> = get_public_ip();
    match public_ip {
        Some(ip) => println!("Public IP Address: {}", ip),
        None => println!("Could not retrieve public IP address"),
    }
}
