// ref: https://doc.rust-lang.org/book/ch20-01-single-threaded.html
use askama::Template;
#[derive(Template)]
#[template(path="index.html")]
struct IndexTmpl<'a>{// html code that will have server addr
    addr:&'a str,
}
use std::{fs::OpenOptions, io::{BufRead, BufReader, Read, Write}, net::{TcpListener, TcpStream}, os::unix::process, process::{exit, Command}, time::Duration};
mod ipc;
mod webserver;
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
    let mut vbuf: Vec<u8> = Vec::new();
    println!("Still reading");
    stream.set_read_timeout(Some(Duration::from_millis(500)));
    stream.read_to_string(&mut buf);
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
fn main() {
    // get ip address then use it to build html template
    if cfg!(target_os="windows") {
        panic!("Using trash os")
    };
    let get_ip = || -> String {
        let args = "route show proto dhcp".split_ascii_whitespace().into_iter();
        let lines = String::from_utf8(Command::new("ip").args(args).output().unwrap().stdout).unwrap();
        let ip_resp = lines.lines().next().unwrap().split_whitespace().nth(6).unwrap();
        String::from(ip_resp)
    };
    let webserver_ip = get_ip();
    let addr = format!("{}:8080",webserver_ip);
    let idx_render = IndexTmpl { addr:&addr };
    println!("Server Address: {}",webserver_ip);
    // do html templating
    let render_hmtl = idx_render.render().expect("Couldn't render");
    let mut f = OpenOptions::new().write(true).create(true).open("renders/index.html").expect("Coulnd't create file to write render to");
    f.write_all(render_hmtl.as_bytes()).expect("Failed to write bytes");
    println!("Server starting");
    start_server(&addr);
}
/* Add this code to examples
 * for this to work mpv must be listening to input-ipc-server @ loafer.sock
 * mpv $VIDEO --input-ipc-server=/tmp/loafer.sock
    let mut conn = ipc::Conn::new().expect("Failed to create connection");
    let path = conn.get_path();
    let mut buf: String = String::new();
    println!("Path: {}",&path);
    println!("Connecting to Handle");
    conn.get_handle().write(
        {
            let load = ipc::Payload::seek_command(400,101);
            println!("sent:{load}");
            Box::new(load).val.as_bytes()
        }
    ).expect("Failed to write");
    let mut reader = BufReader::new(conn.get_handle());
    reader.read_line(&mut buf).expect("Couldn't read til new line"); // msgs end with new line
    conn.get_handle().write(
        {
            let load = ipc::Payload::set_fullscreen(true,11);
            println!("sent:{load}");
            Box::new(load).val.as_bytes()
        }
    ).expect("Failed to write");
    println!("Read {buf}");
  */
