#![feature(test)]
extern crate test;
extern crate hyper;

use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io::BufWriter;
use std::time::Duration;
use std::path::Path;
use std::env::current_exe;
use hyper::client::Client;

pub const DOWNLOAD_LINK: &'static str = "https://hassanolity.files.wordpress.com/2013/11/the_c_programming_language_2.pdf";

pub fn download_url(url: &str, tid: usize) {
    let ce = current_exe().unwrap();
    let cd = ce.parent().unwrap();
    let dldir = cd.join("downloads");
    fs::create_dir_all(dldir.clone()).unwrap();
    let filename = dldir.join(&format!("t{}{}", tid, get_url_filename(url).unwrap()));
    //println!("Downloading to {}", filename.to_str().unwrap());
    let mut client = Client::new();
    client.set_read_timeout(Some(Duration::from_millis(500)));
    let mut outfile = BufWriter::new(File::create(filename).unwrap());
    let mut stream = client.get(url).send().unwrap();
    let mut buf: [u8; 1] = [0; 1];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                outfile.write(&buf).unwrap();
            }
            //Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => break,
            Err(e) => panic!(e),
        }
    }
    outfile.flush().unwrap()
}

pub fn get_url_filename(url: &str) -> Option<&str> {
    url.split('/').last()
}
