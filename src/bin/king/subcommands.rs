use askama::Template;
#[derive(Template)]
#[template(path="index.html")]
struct IndexTmpl<'a>{// html code that will have server addr
    addr:&'a str,
}
use loafer_lib::webserver::api::start_server;
use std::{fmt::Display, fs::{self, read_dir, OpenOptions, ReadDir}, io::Write, iter::SkipWhile, mem, path::{self, Path, PathBuf}, process::{Command}};
// run this code if user runs "king host"
// renders/index.html gets appended to path
pub fn host(p:&Path) {
    let p = p.join("renders/index.html");
    // get the path to the render folder as the path to host
    let args = format!("-m http.server -b 0.0.0.0 8000 -d {}",p.parent().unwrap().display());
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
    // write rendered data to path
    println!("Writing path is: {}",p.display());
    let mut f = OpenOptions::new().write(true).create(true).open(p).expect("Coulnd't create file to write render to");
    f.write_all(render_hmtl.as_bytes()).expect("Failed to write bytes");
    // maybe start python web server after write_all(render is done)
    Command::new("python3").args(args.split_ascii_whitespace()).spawn().expect("Failed to spawn python3 server");
    println!("Server starting");
    start_server(&addr);
}

/// get last watched for a given path
/// play remaining videos in list
// run in another thread
// do this is someone runs "king binge"
pub fn play_list(p:&Path) {
    let accepted_types = ["mkv","mp4"];
    // list of videos as path(might need path again)
    let mut videos:Vec<PathBuf>= read_dir(p)
        .unwrap()
        .into_iter()
        // get list of accepted video types to play
        .filter(|entry_opt| entry_opt.as_ref().is_ok_and(|entry| entry.path().extension().is_some() == true)) // get rid of all files without a filetype
        .filter(|entry_opt| entry_opt.as_ref().is_ok_and(|entry| accepted_types.contains(&entry.path().extension().expect("found file without an extension").to_str().expect("couldn convert path to string"))))
        // return get rid of result types
        .map(|entry|
            entry.unwrap().path()
        )
        .collect();
    // sort videos
    videos.sort();
    if videos.is_empty() {
        println!("No videos in directory to play");
        return;
    }
    // get last watched_video
    // either from watched.txt or front of vec of videos
    // last watched video is either read from watched.txt or just play the first video
    if let Ok(video) = fs::read_to_string(p.join("watched.txt")) {
        println!("Read from watched.txt");
        let last_watched = Path::new(p).join(video.trim());
        // jump iterator to the last video watched
        let mut v = videos.into_iter().skip_while(|video| {
            !video.as_path().to_str().unwrap().trim().contains(&last_watched.file_name().unwrap().to_str().unwrap())
            // !video.as_path().file_name().unwrap().to_str().unwrap().contains(&last_watched.file_name().unwrap().to_str().unwrap())
        });
        v.next(); // get next video(unwatched)
        videos = v.collect(); // now this only contains unwatched videos
    }
    videos.iter().for_each(|f| println!("Remaining Videos:{:?}",f));
    // for all videos > last watched
    // watch it
    // update watched.txt
    videos.into_iter().for_each(|video|{
        Command::new("mpv").arg(&video).output().expect("couldn't start video");
        println!("finished:{:?}\t incrementing last_watched",&video);
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
    use crate::subcommands::play_list;
    #[test]
    fn king_dir_test(){
        let p = path::Path::new("/home/watashi/Areas/Anime/imadsoku");
        play_list(p);
    }
}
