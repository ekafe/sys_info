extern crate systemstat;
use log::{info};

use std::thread;
use std::time::{Duration};

mod util;
use util::start_log;
mod remote_pc;
use remote_pc::RemotePC;

fn main() {

    let one_minute = Duration::from_secs(5);

    start_log();

    // loop{
    for _ in 0..5 {
        let mut pc1 = RemotePC::new("PMD RasPi");
        pc1.get_all();
        info!("\n{}",pc1);
        thread::sleep(one_minute);
    }

    // }
}

