use std::{net::{TcpListener, TcpStream}, io::{Read, self, Write}};

fn main () -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    // let mut request = r#"{
    //     "request_type": "store",
    //     "key": "some_key_3",
    //     "hash": "0b672dd94fd3da6a8d404b66ee3f0c83"
    //   }"#;

    let mut buffer = [0u8; 256];

    let bytes_read = stream.read(&mut buffer)?;
    // println!("{:?}", buffer);

    let parsed_string = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
    
    // println!("{}", bytes_read);
    println!("Server response: {:?}", parsed_string);
    
    let mut request = r#"{
       "request_type": "load",
       "key": "some_key"
     }"#;

    stream.write_all(request.as_bytes());

    let mut buffer = [0u8; 256];

    let bytes_read = stream.read(&mut buffer)?;
    // println!("{:?}", buffer);

    let parsed_string = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
    
    // println!("{}", bytes_read);
    println!("Server response: {:?}", parsed_string);

    Ok(())
}
