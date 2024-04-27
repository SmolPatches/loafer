use std::{ time::Duration, net::{TcpListener,TcpStream}, io::{BufRead, BufReader, Read, Write} };
use crate::{ipc,webserver};
pub fn start_server(addr:&str) {
    println!("SServer\tAddr:{}",addr);
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Answering stream!!!");
        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut conn = ipc::api::Conn::new().expect("Failed to create connection");
    let path = conn.get_path();
    // http stuff
    //let mut br = BufReader::new(&stream);
    let mut buf = String::new();
    println!("Still reading");
    stream.set_read_timeout(Some(Duration::from_millis(500)));
    stream.read_to_string(&mut buf).expect("Couldnt read http data to string");
    println!("DONE reading, \n{}",&buf);
    let msg = webserver::parser::convert_body(&buf);
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    // send it to ipc
    match msg.unwrap() {
        webserver::parser::Body::Seek(x) => {
            println!("Got seek");
            let load =ipc::api::Payload::seek_command(x,0).val;
            conn.get_handle().write(load.as_bytes());
        },
        webserver::parser::Body::SetFullscreen(x) => {
            println!("Got fs");
            let load =ipc::api::Payload::set_fullscreen(x,0).val;
            conn.get_handle().write(load.as_bytes());
        },
        webserver::parser::Body::SetPause(x) => {
            println!("Got fs");
            let load =ipc::api::Payload::set_pause(x,0).val;
            conn.get_handle().write(load.as_bytes());
        },
        _=> todo!()
        //Body::SetFullscreen(x),
        //Body::SetPause(x),
        //Body::GetFullscreen,
        //Body::GetPause,
    };
}
