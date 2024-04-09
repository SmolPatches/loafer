use std::fmt;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
// ref: https://doc.rust-lang.org/book/ch20-01-single-threaded.html
#[derive(Debug,Ord,PartialOrd,PartialEq,Eq)]
pub struct Status(u16); // implement http status for error checking(using the type system)
#[derive(Clone,Debug)]
pub enum HTTPError {
    InvalidStatus {code: u16},
}
impl fmt::Display for HTTPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self{
            HTTPError::InvalidStatus{code} => write!(f,"{code} is not a valid http code")
        }
    }
}
// Different error messages according to AppError.code
impl Status {
    pub fn from(val:u16) -> Result<Status,HTTPError> {
        // return error if
        let http_range = 100..599;
        match http_range.contains(&val) {
            true => {
                // process status Code
                todo!()
            },
            false => {
                // error invalid version
                // return error
                return Err(HTTPError::InvalidStatus{code:val})
            }
        }
    }
}
pub enum Request{
    GET {
        status: Status,
        // other data that comes with get
    },
    /*
    POST,
    PUT,
    */
    INVALID(HTTPError)
    //...
}
pub fn start_server() -> ! {
    unimplemented!();
}
pub fn verify_http<T: AsRef<[u8]>>(data:T) {
    let mut reader = BufReader::new(data.as_ref());
    let mut line: String = String::with_capacity(1024);
    if let Ok(a) = reader.read_line(&mut line) {
       // lexical analysis???
    }
    unimplemented!()
}

fn handle_conn(mut stream: TcpStream) {
}
