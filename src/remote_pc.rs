use systemstat::{
        System, 
        Platform, 
        saturating_sub_bytes,
        IpAddr::V4,
        IpAddr::V6,
        NetworkAddrs};

use std::thread;
use std::time::{Duration};
use chrono::{DateTime,Local};

pub struct RemotePC{
    name:String,
    network:String,
    memory:String,
    load:String,
    temp:f32,
    up_time:String,
    boot_time:String,
    system_stat:System,
    time:String,
}

impl RemotePC{

    pub fn new(name:&str)->Self{

        let stat = System::new();
        let time = get_time_stamp();

        Self{
            name:name.to_string(),
            network:"-1".to_string(),
            memory:"-1".to_string(),
            load:"-1".to_string(),
            temp:-1.0,
            up_time:"-1".to_string(),
            boot_time:"-1".to_string(),
            system_stat:stat,
            time:time,
        }
    }

    /**
     *  Call All Status functions
     *    
     *  1 - Network Details
     *  2 - Memory Details
     *  3 - CPU Load
     *  4 - Boot and UpTime
     *  5 - Temperature
     *  So while printing in console will be neat
    */
    pub fn get_all(&mut self){
        self.get_network_details();
        self.get_memory_details();
        self.get_cpu_load();
        self.get_up_time();
        self.get_temperature();
    }

    pub fn get_network_details(&mut self){

        let net = &self.system_stat.networks();
        let mut network_names = Vec::<String>::new();

        match net {
            Ok(netifs) => {
                // println!("\nNetworks Details :");
                
                for netif in netifs.values() {

                    let ip_details = get_ip_as_string(netif.addrs.clone());
                    let network_name = netif.name.clone();

                    let network_details = format!("{} : {}",network_name,ip_details);
                    network_names.push(network_details);

                    // println!("{} ({:?}) - ({:?})", netif.name, netif.addrs[0].addr,sys.network_stats(&netif.name));
                }
            }
            Err(x) => {
                println!("\nNetworks: error: {}", x);
            }
        }

        let to_print = array_to_string_print(network_names);
        self.network = to_print;
    }

    pub fn get_memory_details(&mut self){

        let memory_details = match self.system_stat.memory() {
                Ok(mem) => {
                    let mut print_statement = "{".to_owned();
                    print_statement += &format!("\n\t\t Ram : {} used / {}",saturating_sub_bytes(mem.total, mem.free), mem.total);
                    print_statement += &format!("\n\t\t Active : {}", mem.platform_memory.meminfo["Active"]);
                    print_statement += &format!("\n\t\t Active(anon) : {}", mem.platform_memory.meminfo["Active(anon)"]);
                    print_statement += &format!("\n\t\t Cached : {}", mem.platform_memory.meminfo["Cached"]);
                    
                    print_statement += "\n\t}";
                    print_statement
                    // println!("Memory: {} used / {})", saturating_sub_bytes(mem.total, mem.free), mem.total);
                    // println!("Active : {:?} \nActive(Anonymous) : {:?} \nCached : {:?}" ,mem.platform_memory.meminfo["Active"],mem.platform_memory.meminfo["Active(anon)"],mem.platform_memory.meminfo["Cached"]);
                },
                Err(_x) => {
                    "-1".to_string()
                    //println!("\nMemory: error: {}", x)
                }
        };
        self.memory = memory_details;
    }

    pub fn get_cpu_load(&mut self){

        let cpu_load = match self.system_stat.cpu_load_aggregate() {
            Ok(cpu)=> {
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                let str_pr = format!("{}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                    cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
                str_pr
            },
            Err(_x) => {
                "-1".to_string()
                // println!("\nCPU load: error: {}", x)
            }
        };

        self.load = cpu_load;
    }

    pub fn get_up_time(&mut self){

        let sys = &self.system_stat;
        
        let up_time = match sys.uptime() {
            Ok(uptime) => format!("{:?}",uptime),
            Err(_x) => {
                "-1".to_string()
                //println!("\nUptime: error: {}", x)
            }
        };

        let boot_time = match sys.boot_time() {
            Ok(boot_time) => format!("{}", boot_time),
            Err(_x) => "-1".to_string()
        };

        self.up_time = up_time;
        self.boot_time = boot_time;
    }

    pub fn get_temperature(&mut self){

        let temp = match self.system_stat.cpu_temp() {
            Ok(cpu_temp) => cpu_temp,
            Err(_x) => -1.0
        };

        self.temp = temp;
    }

}

impl std::fmt::Display for RemotePC {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut print:String = format!("System Details - {}\n",self.time);
        print += "-----------------------------------";
        print += &format!("\nName: {}",self.name);
        print += &format!("\nNetworks : {}",self.network);
        print += &format!("\nMemory-Details : {}",self.memory);
        print += &format!("\nCPU-Load : {}",self.load);
        print += &format!("\nCPU-Temp : {}°C",self.temp);
        print += &format!("\nUp-Time : {}",self.up_time);
        print += &format!("\nOn-Time : {}",self.boot_time);
        print += "\n-----------------------------------";
        write!(f, "{}", print)
        // write!(f, "Name : {}", self.name)
    }
 }

 /**
  *  Converting IP Struct to String
  * 
  *  From System Stat Crate IP struct has to converted. 
  *  Then its matched to get IpV4 and V6 as String
 */
 fn get_ip_as_string(address:Vec<NetworkAddrs>)->String{

    //! Condition Check for Zero Length Array
    if address.len() == 0 { 
        return "-1".to_string()
    }

    let ip_v4_address = address[0].clone().addr;
    let ip_v6_address = address[1].clone().addr;

    let ip_v4 = match ip_v4_address{
        V4(add) => add.to_string(),
        _ => "-1".to_string()
    };

    let ip_v6 = match ip_v6_address{
        V6(add) => add.to_string(),
        _ => "-1".to_string()
    };

    let return_ip = format!("IPv4 : {}, IPv6 : {}",ip_v4,ip_v6);
    return_ip

 }


/** 
 *  Convert Array of Strings to String
 *   
 *  Each values will be append with '\n' as new line
 *  So while printing in console will be neat
*/
fn array_to_string_print(datas:Vec<String>)->String{

    
    if datas.len() == 0 {
        return "".to_string()
    }
    
    let mut return_string:String = "{".to_string();

    for data in datas.iter() {
        return_string += &format!("\n\t\t{}",data);
    }
    return_string += "\n\t}";
    return_string
}

pub fn get_time_stamp()->String{
    let utc: DateTime<Local> = Local::now();
    let date_time = utc.format("%Y-%m-%d_%H:%M:%S").to_string();
    date_time
}

