extern crate num_cpus;
extern crate download;

fn main() {
    let maxworkers = num_cpus::get();
    for id in 0..maxworkers {
        download_url(DOWNLOAD_LINK, id);
    }
}
