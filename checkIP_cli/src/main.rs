use std::net::{IpAddr, UdpSocket};
use std::process::Command;

/// Retrieves the system's hostname by executing the `hostname` command.
///
/// # Returns
///
/// * `Some(String)` - The hostname if successfully retrieved and converted to UTF-8
/// * `None` - If the command fails to execute or returns a non-success status
///
/// # Panics
///
/// Panics if the `hostname` command cannot be executed at all.
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

/// Determines the local IP address by creating a UDP socket and connecting to an external address.
///
/// This function doesn't actually send any data; it uses the socket connection to determine
/// which local IP address would be used to reach the external address (8.8.8.8).
///
/// # Returns
///
/// * `Some(IpAddr)` - The local IP address that would be used for external connections
/// * `None` - If the socket cannot be created or connected
fn get_local_ip() -> Option<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let local_addr = socket.local_addr().ok()?;
    Some(local_addr.ip())
}

/// Retrieves the public IP address by querying an external API service.
///
/// Makes an HTTP GET request to https://api.ipify.org which returns the public IP address
/// as seen from the internet.
///
/// # Returns
///
/// * `Some(IpAddr)` - The public IP address if successfully retrieved and parsed
/// * `None` - If the HTTP request fails or the response cannot be parsed as an IP address
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
