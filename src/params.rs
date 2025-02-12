use std::path::PathBuf;

use structopt::StructOpt;

use super::*;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "blackwater", about = "Asynchronous Port Scanner written in rust  https://github.com/lflxp/blackwater")]
pub struct Params {
    /// Scanned IP Range address eg: 127.0.0.1-255
    #[structopt(short = "i", long = "ip")]
    pub ip: Option<String>,

    /// Number of concurrent scans
    #[structopt(short = "c", long = "concurrency", default_value = "65535")]
    pub concurrency: u32,

    /// Port Range <port,port,port> or <port-port> or <port,port,port-port>
    #[structopt(short = "p", long = "port", default_value = "21,22,23,25,69,79,80-89,110,111,113,115,119,135,137,138,139,143,152,153,158,161,162,179,194,201,209,213,218,220,259,264,308,389,443,445,512,513,514,524,530,531,532,540,542,544,546,547,548,554,556,563,591,593,604,631,636,647,648,652,665,666,674,691,692,695,699,700,701,702,706,771,720,749,782,829,860,873,901,902,904,981,989,990,991,992,993,995,1025,1433,1521,2082,2083,2086,2087,2095,2096,2077,2078,2222,2601,2604,3128,3306,3311,3312,3389,5432,5560,5900,5984,6379,7001,7002,7778,8080-9090,9200,9300,9418,10000,11211,27017,27018,50000,50030,50070")]
    pub port: String,

    /// Result output file address
    #[structopt(short = "f", long = "outfile", parse(from_os_str))]
    pub outfile: Option<PathBuf>,

    /// Timeout  Milliseconds
    #[structopt(short = "t", long = "timeout", default_value = "800")]
    pub timeout: u64,

    /// Scanning with UDP
    #[structopt(short = "u", long = "udp")]
    pub udp: bool,
}

impl Params {
    pub async fn get_ips(&self) -> Result<Vec<String>> {
        let ips = self.ip.as_ref().unwrap();
        // 判断是否存在 - 10.1-244.0.1
        let idx1 = match ips.find("-") {
            Some(idx) => idx,
            None => 0,
        };

        if idx1 == 0 {
            return Ok(vec![format!("{}", ips)])
        }

        let mut a = Vec::new();
		let mut b = Vec::new();
		let mut c = Vec::new();
		let mut d = Vec::new();

        let tmp: Vec<&str> = ips.split(".").collect();
        for (index,n) in tmp.iter().enumerate() {
            if index == 0 {
				a = self.strtoi32(n);
			} else if index == 1 {
				b = self.strtoi32(n)
			} else if index == 2 {
				c = self.strtoi32(n)
			} else if index == 3 {
				d = self.strtoi32(n)
			}
        }

        let data = &mut Vec::new();
        for x in a.clone() {
            for y in b.clone() {
                for z in c.clone() {
                    for g in d.clone() {
                        data.push(format!("{}.{}.{}.{}",x,y,z,g))
                    }
                }
            }
        }
            
        Ok(data.to_vec())
    }

    // 1-3 -> [1,2,3]
    fn strtoi32(&self, input: &str) -> Vec<i32> {
        let mut v = Vec::new();
		if input.contains("-") {
			// Vec<str> -> Vec<i32> -> for x in a..b
			let tmp:Vec<i32> = input.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
			for x in tmp[0]..tmp[1] {
				v.push(x);
			}
		} else {
			v.push(input.parse::<i32>().unwrap());
		}
		v
    }

    pub async fn get_ports(&self) -> Result<Vec<String>> {
        let idx1 = match self.port.find("-") {
            Some(idx) => idx,
            None => 0,
        };

        let idx2 = match self.port.find(",") {
            Some(idx) => idx,
            None => 0,
        };

        if idx1 == 0 && idx2 == 0 {
            return match self.port.parse::<i32>() {
                Ok(i) => {
                    Ok(vec![format!("{}", i)])
                }
                Err(_) => {
                    Err("Parameter Error".into())
                }
            };
        }
        let mut lists = Vec::new();

        // 处理情况: 80,84,86-89,59
        if idx1 != 0 && idx2 != 0 {
            let sli: Vec<&str> = self.port.split(",").collect();
            for i in sli {
                if let Some(t) = i.find("-") {
                    let start = i[..t].parse::<i32>().unwrap();
                    let end = i[t + 1..].parse::<i32>().unwrap();
                    for ic in start..=end {
                        lists.push(format!("{}", ic));
                    }
                } else {
                    lists.push(i.trim().to_string());
                }
            }

            return Ok(lists);
        }

        // param1
        if idx1 != 0 {
            let start = *&self.port[..idx1].parse::<i32>().unwrap();
            let end = *&self.port[idx1 + 1..].parse::<i32>().unwrap();
            for i in start..=end {
                lists.push(format!("{}", i));
            }
            return Ok(lists);
        }

        // param2
        let sli: Vec<&str> = self.port.split(",").collect();
        for i in sli {
            lists.push(i.trim().to_string());
        }

        Ok(lists)
    }
}

