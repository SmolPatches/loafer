use std::os::unix::net::{SocketAddr, UnixListener, UnixStream};
use serde_json::json;
use std::fmt;
//pub struct Payload(String);
pub struct Payload{
    pub val: String
}
impl Payload {
    pub fn seek_command(time:isize,request_id:usize) -> Payload{
        Payload{ val:{
        let mut v=json!({
            "command": [
                "seek",
                time
            ],
            "request_id":request_id
        }).to_string();
        v.push_str("\n"); // Tells ipc that this is the end of the msg
        v
        }}
    }
    pub fn get_pause(state:bool,request_id:usize) -> Payload {
        Payload{ val:{
        let mut v=json!({
            "command": [
                "get_property",
                "pause",
                state
            ],
            "request_id":request_id
        }).to_string();
        v.push_str("\n"); // Tells ipc that this is the end of the msg
        v
        }}
    }
    pub fn get_fullscreen(state:bool,request_id:usize) -> Payload {
        Payload{ val:{
        let mut v=json!({
            "command": [
                "get_property",
                "fullscreen",
                state
            ],
            "request_id":request_id
        }).to_string();
        v.push_str("\n"); // Tells ipc that this is the end of the msg
        v
        }}
    }
    pub fn set_pause(state:bool,request_id:usize) -> Payload {
        Payload{ val:{
        let mut v=json!({
            "command": [
                "set_property",
                "pause",
                state
            ],
            "request_id":request_id
        }).to_string();
        v.push_str("\n"); // Tells ipc that this is the end of the msg
        v
        }}
    }
    pub fn set_fullscreen(state:bool,request_id:usize) -> Payload {
        Payload{ val:{
        let mut v=json!({
            "command": [
                "set_property",
                "fullscreen",
                state
            ],
            "request_id":request_id
        }).to_string();
        v.push_str("\n"); // Tells ipc that this is the end of the msg
        v
        }}
    }
}
impl fmt::Display for Payload { // display formatter
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.val)
    }
}
// Note: mpv creates server & we are the clients
pub struct Conn<'a>{
    path: &'a str,
    sock_handle: UnixStream,
}
impl<'a> Conn<'a> {
    // connect to new unix server @ /tmp/loafer.sock
    pub fn new() -> std::io::Result<Conn<'a>>  {
        let mut stream = UnixStream::connect("/tmp/loafer.sock")?;
        Ok(Conn {
            sock_handle: stream,
            path: "/tmp/loafer.sock",
        })
    }
    /// build a new connection via a socket or ipc
    /// from a path
    /// should have arguements such to specify path of unixstream / if to use random uuid for file path
    pub fn from() {
        unimplemented!()
    }
    pub fn get_path(&self) -> Box<&str> {
        Box::new(&self.path)
    }
    pub fn get_handle(&mut self) -> Box<&UnixStream> {
        Box::new(&self.sock_handle)
    }
}
// forward skip
//{ "command": ["seek", "30"] }
//Result: {"data":null,"request_id":0,"error":"success"}

// backward skip
//{ "command": ["seek", "-30"] }
// Result: {"data":null,"request_id":0,"error":"success"}
//{ "command": ["seek", "-30"], "request_id": 101 }
// Result: {"data":null,"request_id":101,"error":"success"}
// https://mpv.io/manual/master/#list-of-input-commands
// Pause
// { "command": ["set_property", "pause", true] }
// Unpause
// { "command": ["set_property", "pause", false] }
// fullscreen
// { "command": ["set_property", "pause", false] }
// Result: {"request_id":0,"error":"success"}
// set_property fail
// {"request_id":0,"error":"invalid parameter"}
// Loadfile command for playing next file
// EVENTS
// {"event":"end-file","reason":"quit","playlist_entry_id":1}
