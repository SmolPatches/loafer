use std::ascii;
use std::collections::HashMap;
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
struct RawBody(String);
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
pub fn parse_body<T: AsRef<[u8]>>(data:T) -> std::io::Result<String>{
    let reader = BufReader::new(data.as_ref());
    let mut found_body = false;
    let mut lines = reader.lines().skip_while(|line|->bool {
        // fix this code
        if line.as_ref().expect("HUH?").trim().is_empty() && !found_body{  // once we find an empty line we got to the body
            return false;
        } else { found_body=true; return true; }
    });
    lines.next(); // consume empty line
    let body = lines.next().expect("Failed to read body")?;
    // do body parsing with url
    Ok(body)
}
pub fn parse_body_str(data:&str) -> std::io::Result<String>{
    let s = data.split_ascii_whitespace().skip_while(|l| !l.is_empty());
    Ok(s.collect())
}
#[derive(Debug)]
pub enum Cycles {
    Audio,
    Subs
}
#[derive(Debug)]
pub enum Body {
    Seek(isize),
    SetFullscreen(bool),
    SetPause(bool),
    GetFullscreen,
    GetPause,
    Cycle(Cycles)
}
pub fn convert_body(s:&str) -> std::io::Result<Body> {
    let TheError:Error = Error::from(ErrorKind::InvalidData);
    println!("s is:{s}");
    if s.contains("cycle=audio") {
        println!("Got set cycle to audio");
        return Ok(Body::Cycle(Cycles::Audio));
    } else if s.contains("cycle=sub") {
        println!("Got set cycle to sub");
        return Ok(Body::Cycle(Cycles::Subs));
    }
    let s:String = s.split_ascii_whitespace().skip_while(|l| !l.contains("cmd=")).collect();
    println!("Conv Body1: {}",s);
    let mut matcher = s.split("&");
    // command=get&param=fullscreen
    // command=get&param=pause
    // command=fullscreen&param=1
    // command=pause&param=1
    // command=seek&param=30
    let cmd = matcher.next().unwrap().split("=").nth(1).unwrap(); // do error checking
    let param = matcher.next().unwrap().split("=").nth(1).unwrap(); // do error checking
    println!("Conv Body: {}\t{:?}",cmd,param);
    match (cmd,param)  {
        ("fullscreen",x) => {
            let parse: bool = x.parse().unwrap();
            Ok(Body::SetFullscreen(parse))
        },
        ("pause",x) => {
            let parse: bool = x.parse().unwrap();
            Ok(Body::SetPause(parse))
        },
        ("seek",x) => {
            let parse: isize = x.parse().unwrap();
            Ok(Body::Seek(parse))
        },
        ("get",x) => {
            match x {
               "fullscreen" => Ok(Body::GetFullscreen),
               "pause" => Ok(Body::GetPause),
                _ => Err(TheError),
            }
        },
        _=> Err(TheError),
    }
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
    #[test]
    fn test_parse_body() {
       let form_req =  "POST /api HTTP/1.1\r
Content-Type: application/x-www-form-urlencoded\r
\r
command=skip&time=30
";
        assert!(parse_body(form_req).is_ok());
    }
    #[test]
    fn test_parse_request(){
       let form_req =  "POST /api HTTP/1.1\r
Content-Type: application/x-www-form-urlencoded\r
\r
command=seek&param=30
";
        let headers = parse_header(form_req).unwrap();
        let mut parse_as_url:bool = false;
        // add parsing to check if it is a url header
        //for header in headers.headers.into_iter() {
        //    if header == ContentTypes::URLENCODED {
        //        parse_as_url = true;
        //    }
        //}
        let data = convert_body(&parse_body(form_req).unwrap()).unwrap();
        println!("Conversion yieled {:?}",data);
    }
}
