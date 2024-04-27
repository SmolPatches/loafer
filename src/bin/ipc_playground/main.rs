use loafer_lib::ipc::api::*;
use std::io::{BufReader,Write,BufRead};
/* Add this code to examples
 * for this to work mpv must be listening to input-ipc-server @ loafer.sock
 * mpv $VIDEO --input-ipc-server=/tmp/loafer.sock
 */
fn main() {

    let mut conn = Conn::new().expect("Failed to create connection");
    let path = conn.get_path();
    let mut buf: String = String::new();
    println!("Path: {}",&path);
    println!("Connecting to Handle");
    conn.get_handle().write(
        {
            let load = Payload::seek_command(400,101);
            println!("sent:{load}");
            Box::new(load).val.as_bytes()
        }
    ).expect("Failed to write");
    let mut reader = BufReader::new(conn.get_handle());
    reader.read_line(&mut buf).expect("Couldn't read til new line"); // msgs end with new line
    conn.get_handle().write(
        {
            let load = Payload::set_fullscreen(true,11);
            println!("sent:{load}");
            Box::new(load).val.as_bytes()
        }
    ).expect("Failed to write");
    println!("Read {buf}");
}
