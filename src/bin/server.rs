use std::{net::{TcpListener, TcpStream}, io::{self, Write, BufRead}};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use chrono::{Local};
use std::collections::HashMap;

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// enum JsonView {
//     Requesttype(String),
//     Key(String),
//     Hash(String),
// }
// #[derive(Serialize, Deserialize)]
// struct Requesting {
//     value: JsonView
// }

#[derive(Serialize, Deserialize)]
pub struct JsonView {
    request_type: String,
    key: String,
    hash: Option<String>,
}

// храни свежие запросы на запись store
// храни свежие запросы на вывод load 
// и передавай эти штуки в лог вместе с тисипилитсенер

// fn logon(request_type: &String, key: &String, hash: &Option<String>, stream: &TcpStream) {
//     println!("{:?} [{:?}] Connection established.  Storage size: {:?}", stream.peer_addr().unwrap(), Local::now(), 10101010);
//     println!("{:?} [{:?}] Received request to write new value {:?} by key {:?}. Storage size: {:?}", stream.peer_addr().unwrap(), Local::now(), 10000, key, 1000);
//     println!("{:?} [{:?}] Received request to get value by key {:?}. Storage size: {:?}", stream.peer_addr().unwrap(), Local::now(), key, 10101010);
// }

fn connect_log(stream: &TcpStream, stsize: &usize) {
    println!("{:?} [{:?}] Connection established.  Storage size: {:?}", stream.peer_addr().unwrap(), Local::now(), stsize);
}

fn store_log(stream: &TcpStream, hash: &String, key: &String, stsize: &usize) {
    println!("{:?} [{:?}] Received request to write new value {:?} by key {:?}. Storage size: {:?}", stream.peer_addr().unwrap(), Local::now(), hash, key, stsize);
}

fn load_log(stream: &TcpStream, key: &String, stsize: &usize) {
    println!("{:?} [{:?}] Received request to get value by key {:?}. Storage size: {:?}", stream.peer_addr().unwrap(), Local::now(), key, stsize);
}

fn handle_connection(mut connection: TcpStream, 
                    storage: &mut HashMap<String, String>, 
                    fresh_hash_for_store: &mut String,
                    fresh_key_for_store: &mut String, 
                    fresh_key_for_load: &mut String) -> io::Result<()> {
    
    // convert this string to type like a json ctructure
    // parse this string to JsonView structure
    // continue work with them

    let mut buffer: Vec<u8> = vec![];
    let bytes_read = std::io::BufReader::new(&connection).read_until(b'}', &mut buffer).unwrap();
    let parse_string = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
    let v: JsonView =  serde_json::from_str(parse_string)?;

    if v.request_type == "store" {
        let stsize = storage.len();
        store_log(&connection, &v.hash.clone().unwrap(), &v.key, &stsize);

        storage.insert(v.key.clone(), v.hash.clone().unwrap());

        *fresh_hash_for_store = v.hash.clone().unwrap();
        *fresh_key_for_store = v.key.clone();

        let answer_for_client = r#"{
            "response_status": "success"
          }"#;
        connection.write_all(answer_for_client.as_bytes()).unwrap();

    } else {
        let stsize = storage.len();
        load_log(&connection, &v.key, &stsize);
        if storage.contains_key(& v.key) == true {
            // Get hash from storage
            let hasf_for_key_in_load = storage.get(&v.key).unwrap();
            let clear_hash = hasf_for_key_in_load.clone();

            // Create the finish json for load  
            let mut map_for_json_load = Map::new();
            map_for_json_load.insert("response_status".to_string(), Value::String("success".to_string()));
            map_for_json_load.insert("requested_key".to_string(), Value::String(v.key.clone()));
            map_for_json_load.insert("requested_hash".to_string(), Value::String(clear_hash));
            
            let obj_for_json_load = Value::Object(map_for_json_load);

            // Send json to client
            let string_json_load = serde_json::to_string(&obj_for_json_load).unwrap();
            connection.write_all(string_json_load.as_bytes()).unwrap();
        } else {
            let answer_for_client = r#"{
                "response_status": "key not found",
              }"#;
            connection.write_all(answer_for_client.as_bytes()).unwrap();
        }
        *fresh_key_for_load = v.key.clone();
    }
    //println!("{}, {}", v.request_type, v.key);
    // match v.value {
    //     JsonView::Requesttype(val) => println!("{}", val),
    //     JsonView::Key(k) => println!("{}", k),
    //     JsonView::Hash(h) => println!("{}", h),
    // }
    //println!("Please call {:?}", v.value);
    //println!("Response {:?}", parse_string);
    //println!("{}", bytes_read);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut storage: HashMap<String, String> = HashMap::new();
    let mut fresh_hash_for_store: String = String::from("Please, upload something to the storage");
    let mut fresh_key_for_store: String = String::from("Please, upload something to the storage");
    let mut fresh_key_for_load: String = String::from("");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {

        let mut stream = stream.unwrap();

        println!("{}", "New connection");
        // Firstly: we should send to client the message about succcessful connection
        //  1. make json message
        //  2. convert to string
        //  3. send some bytes of string

        let greeting = r#"{ "student_name": "Your Name", }"#;
        stream.write_all(greeting.as_bytes()).unwrap();

        let stsize = storage.len();
        connect_log(&stream, &stsize);

        // Secondly: we should do request from our client and send some message to him
        handle_connection(stream, &mut storage, &mut fresh_hash_for_store, &mut fresh_key_for_store, &mut fresh_key_for_load)?;
    }
    Ok(())
}