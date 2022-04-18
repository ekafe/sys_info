extern crate simplelog;
use simplelog::*;

use std::fs::File;
use std::fs;

use crate::remote_pc::get_time_stamp;
use crate::remote_pc::get_date;
// use std::env;

/**  Start the Logging Configuration */
pub fn start_log(){

    let date = get_date();

    let dir = fs::create_dir_all(format!("log/{date}"));
    match dir{
        Ok(_) =>{},
        Err(err) =>{
            panic!("Error Creating Directory:{}",err);
        }
    }
    // let dir = env::current_dir().unwrap();
    // let path = dir.into_os_string().into_string().unwrap();
    // let path = "/home/pi/Desktop/usage_log";
    // let log_file_name = format!("{}/log/system_stat_log_{}.log",path,get_time_stamp());
    let log_file_name = format!("./log/{}/system_stat_log_{}.log",date,get_time_stamp());
    println!("{}",log_file_name);
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(log_file_name).unwrap()),
        ]
    ).unwrap();

}



