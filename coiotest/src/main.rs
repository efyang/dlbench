extern crate download;
extern crate coio;
extern crate num_cpus;

use coio::Scheduler;
use download::*;


fn main() {
    let maxworkers = num_cpus::get();
    Scheduler::new()
        .run(move || {
            let mut workers = Vec::with_capacity(maxworkers);
            for id in 0..maxworkers {
                workers.push(coio::spawn(move || {
                    download_url(DOWNLOAD_LINK, id);
                }));
            }
            for r in workers {
                r.join().unwrap();
            }
        })
        .unwrap();
}
