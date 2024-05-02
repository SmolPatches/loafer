// ref: https://doc.rust-lang.org/book/ch20-01-single-threaded.html
use askama::Template;
use std::{collections::VecDeque, fs::{self, read_dir, OpenOptions, ReadDir}, io::Write, mem, os::unix::ffi::OsStrExt, path::{self, Path, PathBuf}, process::{exit, Command}};
use loafer_lib::webserver::api::start_server;
#[derive(Template)]
#[template(path="index.html")]
struct IndexTmpl<'a>{// html code that will have server addr
    addr:&'a str,
}
/// get last watched for a given path
/// play remaining videos in list
// run in another thread
// do this is someone runs "king binge"
fn play_list(p:&Path) {
    let accepted_types = ["mkv","mp4"];
    // list of videos as path(might need path again)
    let mut videos:VecDeque<_>= read_dir(p)
        .unwrap()
        .into_iter()
        // get list of accepted video types to play
        .filter(|entry_opt| entry_opt.as_ref().is_ok_and(|entry| accepted_types.contains(&entry.path().extension().unwrap().to_str().unwrap())))
        // return get rid of result types
        .map(|entry|
            entry.unwrap().path()
        )//)
        .collect();
    // get last watched_video
    // either from watched.txt or front of vec of videos
    let last_watched:PathBuf= match fs::read_to_string(p.join("watched.txt")) {
        Ok(video) => {
            // trim new line because data comes from file
            Path::new(p).join(video.trim())
        },
        _ => {
            //String::from(videos.pop_front().unwrap().file_name().unwrap().to_str().unwrap())
            videos.pop_front().unwrap()
        }
    };
    // jump iterator to the last video watched
    let mut videos = videos.iter().skip_while(|video| {
        !video.as_path().file_name().unwrap().to_str().unwrap().contains(&last_watched.file_name().unwrap().to_str().unwrap())
    });
    // make it peekable

    videos.clone().by_ref().for_each(|f| println!("Remaining Videos:{:?}",f));
    videos.next(); // this was the last watched video
    // for all videos > last watched
    // watch it
    // update watched.txt
    videos.for_each(|video|{
        Command::new("mpv").arg(video).output().expect("couldn't start video");
        println!("finished:{:?}\t incrementing last_watched",video);
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(p.join("watched.txt"))
            .expect("Coulnd't create file to write render to")
            .write_all(video.to_str().unwrap().as_bytes())
            .expect("Couldn't update watched.txt"); // and update watched.txt once mpv exits
    });
}
#[cfg(test)]
mod tests {
    use std::path;

    use crate::play_list;

    #[test]
    fn king_dir_test(){
        let p = path::Path::new("/home/watashi/Areas/Anime/imadsoku");
        play_list(p);
    }
}
// run this code if user runs "king host"
fn host() {
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
    let mut f = OpenOptions::new().write(true).create(true).open("king/renders/index.html").expect("Coulnd't create file to write render to");
    f.write_all(render_hmtl.as_bytes()).expect("Failed to write bytes");
    println!("Server starting");
    start_server(&addr);
}
fn main() {
 // use clap to do command line parsing
 // call host or play_list(p)
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
