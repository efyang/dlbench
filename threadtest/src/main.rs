extern crate num_cpus;
extern crate download;

use download::*;
use std::thread;

fn main() {
    let maxworkers = num_cpus::get();
    let mut workers = Vec::with_capacity(maxworkers);
    for id in 0..maxworkers {
        workers.push(thread::spawn(move || {
            download_url(DOWNLOAD_LINK, id);
        }));
    }
    for t in workers {
        t.join().unwrap();
    }
}
