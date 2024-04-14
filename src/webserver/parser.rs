use std::io::{Error,ErrorKind};
use std::io::{BufRead, BufReader};
// parser doesnt aim to be complete
// just enough so it can read data from browser requests
// and reply
#[derive(Debug)]
enum Method {
    POST,
}
//struct URL<'a>(&'a str);// maybe make do some processing to make a real string type
#[derive(Debug)]
enum ContentTypes {
   URLENCODED,
}
#[derive(Debug)]
enum Connections{
    KeepAlive,
}
#[derive(Debug)]
enum Headers {
    ContentType(ContentTypes),
    Connection(Connections),
    ContentLength(u8),
    Host(String),
}
#[derive(Debug)]
struct Header{
    method: Method,
    location: String,
    version: String,
    headers: Vec<Headers>,
}
/// Given any readable datatypes, read them and get their method, location and additional headers
pub fn parse_header<T: AsRef<[u8]>>(data:T) -> std::io::Result<Header>{
    let mut reader = BufReader::new(data.as_ref());
    let mut line: String = String::with_capacity(1024);
    reader.read_line(&mut line)?;
    let mut line = line.split_ascii_whitespace();
    if let (Some(method), Some(location), Some(version)) = (line.next(),line.next(),line.next()) {
        let method = match method {
            "POST" => Method::POST,
            &_ => return Err(Error::new(ErrorKind::InvalidData,"not an implemented method"))
        };
        let (location,version) = (location.to_string(),version.to_string());
        let mut headers: Vec<Headers> = Vec::new();
        for lines in reader.lines() {
            let loop_line: String = lines?;
            if loop_line.trim().is_empty() { // handle end of headers
                break;
            }
            let mut it = loop_line.trim().split_ascii_whitespace();
            let verb = it.find(|&ctx| {
                ctx.ends_with(":")
            }).ok_or(Error::new(ErrorKind::InvalidData,"Missing semicolon:verb"))?; // return error if missing
            let optional_data = it.next();
            match (verb,optional_data.as_deref()) {
                ("Content-Type:",Some("application/x-www-form-urlencoded")) => {
                    headers.push(Headers::ContentType(ContentTypes::URLENCODED))
                }
                ("Content-Type",Some(_)) => { // other content types
                    todo!("Capture and parse other content types")
                },
                ("Host:",Some(host)) => {
                   headers.push(Headers::Host(host.to_owned()))
                }
                (&_,x) => todo!("Add additional header parsing"),
            }
        }
        return Ok(Header {
            method,
            location,
            version,
            headers
        })
    }
    Err(Error::from(ErrorKind::InvalidData))
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_parse_header() {
       let form_req =  "POST /api HTTP/1.1\r
Content-Type: application/x-www-form-urlencoded\r
\r
command=skip&time=30
";
        assert!(parse_header(form_req).is_ok());
        println!("{:?}",parse_header(form_req).unwrap());
        let get_req =  "GET /api HTTP/1.1\r";
        assert!(parse_header(get_req).is_err());
        assert_eq!(parse_header(get_req).err().unwrap().kind(),ErrorKind::InvalidData);
    }
}
