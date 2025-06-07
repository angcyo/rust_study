///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/22
///

pub async fn get_url_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;
    //println!("body = {body:?}");
    Ok(body)
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

    /// 测试TCP服务端
    #[test]
    fn test_tcp_server() {
        let id = 8080;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), id);
        match std::net::TcpListener::bind(addr) {
            Ok(listener) => {
                println!("Server listening on {}", listener.local_addr().unwrap());
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut stream) => {
                            println!("New connection:{}", stream.peer_addr().unwrap());
                            let mut bufer = vec![];
                            match stream.read_to_end(&mut bufer) {
                                Ok(n) => {
                                    println!(
                                        "Read {} bytes->{}",
                                        n,
                                        rc_basis::bytes::bytes_to_string(&bufer)
                                    );
                                }
                                Err(e) => {
                                    eprintln!("[Server]Read Error: {}", e);
                                }
                            }
                            stream.flush().unwrap();
                            stream.shutdown(std::net::Shutdown::Both).unwrap_or(());
                            break;
                        }
                        Err(e) => {
                            eprintln!("[Server]Incoming Error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("[Server]Bind Error: {}", e);
            }
        }
        println!("[Server]...end");
    }

    /// 测试TCP客户端
    #[test]
    fn test_tcp_client() {
        let id = 8080;
        //SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), id));
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), id);
        match std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_secs(1)) {
            Ok(mut stream) => {
                stream.write_all("Hello Rust".as_bytes()).unwrap();
                stream.flush().unwrap();
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                println!("[Client]Sent Hello Rust");
            }
            Err(e) => {
                eprintln!("[Client]Connect Error: {}", e);
            }
        }
        println!("[Client]...end");
    }
}
