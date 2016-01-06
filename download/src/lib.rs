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

pub const DL_LINK_KR: &'static str = "http://localhost:8080/why-rust.pdf";

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
    //let mut buf: [u8; 16] = [0; 16];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                outfile.write(&buf).unwrap();
                //outfile.write(&buf[..n]).unwrap();
            }
            //Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => break,
            Err(e) => panic!(e),
        }
     }
    outfile.flush().unwrap()
}

pub fn download_url_16(url: &str, tid: usize) {
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
    //let mut buf: [u8; 1] = [0; 1];
    let mut buf: [u8; 16] = [0; 16];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                outfile.write(&buf).unwrap();
                outfile.write(&buf[..n]).unwrap();
            }
            //Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => break,
            Err(e) => panic!(e),
        }
     }
    outfile.flush().unwrap()
}

pub fn download_url_128(url: &str, tid: usize) {
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
    //let mut buf: [u8; 1] = [0; 1];
    let mut buf: [u8; 128] = [0; 128];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                outfile.write(&buf).unwrap();
                outfile.write(&buf[..n]).unwrap();
            }
            //Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => break,
            Err(e) => panic!(e),
        }
     }
    outfile.flush().unwrap()
}


pub fn download_url_256(url: &str, tid: usize) {
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
    //let mut buf: [u8; 1] = [0; 1];
    let mut buf: [u8; 256] = [0; 256];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                outfile.write(&buf).unwrap();
                outfile.write(&buf[..n]).unwrap();
            }
            //Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => break,
            Err(e) => panic!(e),
        }
     }
    outfile.flush().unwrap()
}

pub fn download_url_512(url: &str, tid: usize) {
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
    //let mut buf: [u8; 1] = [0; 1];
    let mut buf: [u8; 512] = [0; 512];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                outfile.write(&buf).unwrap();
                outfile.write(&buf[..n]).unwrap();
            }
            //Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => break,
            Err(e) => panic!(e),
        }
     }
    outfile.flush().unwrap()
}

pub fn download_url_1024(url: &str, tid: usize) {
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
    //let mut buf: [u8; 1] = [0; 1];
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                outfile.write(&buf).unwrap();
                outfile.write(&buf[..n]).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn buf_1(b: &mut Bencher) {
        b.iter(|| {
            download_url(DL_LINK_KR, 0);
        });
    }

    #[bench]
    fn buf_16(b: &mut Bencher) {
        b.iter(|| {
            download_url_16(DL_LINK_KR, 0);
        });
    }
    
    #[bench]
    fn buf_128(b: &mut Bencher) {
        b.iter(|| {
            download_url_128(DL_LINK_KR, 0);
        });
    }

    #[bench]
    fn buf_256(b: &mut Bencher) {
        b.iter(|| {
            download_url_256(DL_LINK_KR, 0);
        });
    }

    #[bench]
    fn buf_512(b: &mut Bencher) {
        b.iter(|| {
            download_url_512(DL_LINK_KR, 0);
        });
    }

    #[bench]
    fn buf_1024(b: &mut Bencher) {
        b.iter(|| {
            download_url_1024(DL_LINK_KR, 0);
        });
    }

}
