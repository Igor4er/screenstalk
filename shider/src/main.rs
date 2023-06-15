use ureq::{self, Response};
use std::path::PathBuf;
use std::{thread, time};
use std::fs;
use directories::UserDirs;
use std::io::{self, copy};
use std::process;


fn get_screenstalk_build_response() -> Response {
    let reqw = ureq::get("https://github.com/Igor4er/screenstalk/blob/main/release/screenstalk.exe").call();
    let file = match reqw {
        Ok(f) => f,
        Err(e) => {
            thread::sleep(time::Duration::from_secs(10));
            get_screenstalk_build_response()
        }
    };
    file
}

fn create_directory(dir_path: PathBuf) -> io::Result<()> {
    match fs::create_dir(dir_path) {
        Ok(()) => Ok(()),
        Err(ref err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        Err(err) => Err(err),
    }
}

fn main() {
    let resp = get_screenstalk_build_response();
    if let Some(user_dirs) = UserDirs::new() {
        let docs_dir = user_dirs.document_dir().unwrap();
        let proj_dir = docs_dir.join("screenstalk");
        create_directory(proj_dir.clone()).unwrap();
        let file_path = proj_dir.join("screenstalk.exe");
        let mut file = fs::File::create(file_path.clone()).unwrap();
        copy(&mut resp.into_reader(), &mut file).unwrap();
        process::Command::new(file_path).spawn().unwrap();
    };

}