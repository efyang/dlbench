extern crate hyper;

use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io::BufWriter;
use std::time::Duration;
use std::path::Path;
use std::env::current_exe;
use hyper::client::Client;

pub const DOWNLOAD_LINK: &'static str = "https://docs.python.org/3/archives/python-3.5.\
                                         1-docs-pdf-a4.zip";

pub fn download_url(url: &str, tid: usize) {
    let ce = current_exe().unwrap();
    let cd = ce.parent().unwrap();
    let dldir = cd.join("downloads");
    fs::create_dir_all(dldir.clone());
    let filename = dldir.join(&format!("t{}{}", tid, get_url_filename(url).unwrap()));
    println!("Downloading to {}", filename.to_str().unwrap());
    let mut client = Client::new();
    client.set_read_timeout(Some(Duration::from_millis(500)));
    let mut outfile = BufWriter::new(File::create(filename).unwrap());
    let mut stream = client.get(url).send().unwrap();
    // let mut buf: [u8; 1] = [0];
    // loop {
    // match stream.read(&mut buf) {
    // Ok(0) => break,
    // Ok(_) => {
    // outfile.write(&buf).unwrap();
    // },
    // Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => break,
    // Err(e) => panic!(e),
    // }
    // }
    for byte in stream.bytes() {
        outfile.write(&[byte.unwrap()]).unwrap();
    }
}

pub fn get_url_filename(url: &str) -> Option<&str> {
    url.split('/').last()
}
