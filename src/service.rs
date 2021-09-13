use super::*;
use std::sync::Arc;
use tokio::net::{TcpStream};
use tokio::sync::{mpsc, Mutex};
// use tokio::prelude::*;
use tokio::time;

use stopwatch::{Stopwatch};
use futures::executor::block_on;
use oping::{Ping, PingResult};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum InputType {
    IPv4,
    IPv6,
    HOSTNAME,
    UNKNOWN
}

static mut IS_LOOPING: bool = true;
static mut SUCCESS: u32 = 0;
static mut FAILURE: u32 = 0;
static mut TIME: f64 = 0.0;

/**
 * This function registers an signal action handler to SIGINT.
 * At signal interrrupt, the function should let the ping loop
 * stop and create a report. 
 */
 pub fn register_sig_action() {
    
    // set an action handler
    match ctrlc::set_handler(move || {
        // set the value of is_pinging to false to stop
        // since it is not threadsafe, use unsafe
        // however, this program only uses main thread so it's fine
        unsafe {
            IS_LOOPING = false;
        }

    }) {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

pub struct Core<'a> {
    param: &'a params::Params,
}

impl<'a> Core<'a> {
    pub async fn new(param: &'a params::Params) -> Core<'a> {
        Core {
            param
        }
    }

    pub async fn print_result(&mut self,tt: Stopwatch) {
        println!("--- Ping result ---");
        unsafe {
            println!("TOTAL  : {} packets", SUCCESS + FAILURE);
            println!("SUCCESS: {}", SUCCESS);
            println!("FAILURE: {}", FAILURE);
            // safe casting using keyword as
            println!("TIME   : {:.3} ms", TIME / (SUCCESS + FAILURE) as f64);
            println!("Times: {:.3} ms", tt.elapsed_ms() as f64)
        }
    }

    pub async fn go(&mut self,ports: Vec<String>) -> Result<Vec<&str>> {
        let start = Stopwatch::start_new();
        match self.param.get_ips().await {
            Ok(args) => {
                // validate argument - check argument length
                // due to ownership, we give a reference of the vector to the function
                if !is_valid_length(&args) {
                    println!("Usage: ping [<hostname> | <ip>]");
                    return
                }

                // get argument type (ip, hostname)
                let arg_type: InputType = get_type_of_arg(&args[1]);
                if arg_type == InputType::UNKNOWN {
                    println!("Error: could not recongize the input type (hostname or ip)");
                    return
                }

                // println! is a macro
                println!("__________[ Ping ]__________");
                println!("Received a type: {:?}", &arg_type);
                println!("Use ctrl-C to get report");

                // 添加ctrl+c 退出
                register_sig_action();

                for r in args {
                    match ping(&r).await {
                        Ok(i) => {
                            runByIp(i, ports).await?;
                        },
                        Err(e) => {
                            println!("error ip {}", e);
                        }
                    }
                }
            },
            _ => {}
        }
        
        print_result(start).await;
        Ok(Vec::new())
    }

    // async spawn process
    pub async fn ping(&mut self,address: &str,ports: Vec<String>) -> PingResult<String>{
        // create ICMP packet using external library oping
        // while searching for external rust libraries, I realized
        // many of them are not being updated anymore or was abandoned.
        let mut ping = Ping::new();
        
        // max wait time is 5 seconds
        ping.set_timeout(0.1)?;

            // println!("ip is |{}|", addr);
        ping.add_host(address)?;        

        // send ICMP packet
        let responses = ping.send()?;

        unsafe {
            // check response and update result
            for response in responses {
                if response.dropped > 0 {
                    // println!("No response from host {} (loss)", response.address);
                    Err(format!("No response from host {} (loss)", response.address));
                    FAILURE += 1;
                } else {
                    // display success result
                    println!("Response from host {} (address {}): latency {} ms", response.hostname, response.address, response.latency_ms);
                    self.runByIp(address, ports).await?;
                    SUCCESS += 1;
                    TIME += response.latency_ms;
                }

                // if response.dropped <= 0 {
                //     println!("Response from host {} (address {}): latency {} ms", response.hostname, response.address, response.latency_ms);
                // }
            }
        }

        Ok(response.address)
    }

    pub async fn runByIp(&mut self, ip: &str,ports: Vec<String>) -> Result<()> {
        let (sen_file, rec_file) = mpsc::unbounded_channel();
        // let mut output = Arc::new(Output::new(rec_file, self.param.outfile.clone()));
        let mut output = Output::new(rec_file, self.param.outfile.clone());

        // run output
        tokio::spawn(async move {
            output.run().await;
        });

        // Concurrency Control
        let (sen_limit, rec_limit) = mpsc::channel(self.param.concurrency as usize);
        let rec_limit = Arc::new(Mutex::new(rec_limit));
        let wg = Arc::new(WaitGroup::new().await);
        // let ip = self.param.ip.as_ref().unwrap();
        let mut timeout = self.param.timeout;
        if timeout == 0 {
            timeout = 800
        }

        for port in ports {
            sen_limit.send(1).await.unwrap();
            wg.add().await;

            let wg = wg.clone();
            let rec_limit = rec_limit.clone();
            let sen_file = sen_file.clone();
            let ip = ip.clone();
            tokio::spawn(async move {
                match Self::blasting(format!("{}:{}", ip, port), timeout).await {
                    Ok(data) => {
                        sen_file.send(data).unwrap();
                        // sen_file.send(data).await.unwrap();
                    }
                    _ => {}
                }
                wg.done().await;

                rec_limit.lock().await.recv().await.unwrap();
            });
        }

        wg.wait().await;

        Ok(())
    }

    pub async fn run(&mut self, ports: Vec<String>) -> Result<()> {
        let (sen_file, rec_file) = mpsc::unbounded_channel();
        // let mut output = Arc::new(Output::new(rec_file, self.param.outfile.clone()));
        let mut output = Output::new(rec_file, self.param.outfile.clone());

        // run output
        tokio::spawn(async move {
            output.run().await;
        });

        // Concurrency Control
        let (sen_limit, rec_limit) = mpsc::channel(self.param.concurrency as usize);
        let rec_limit = Arc::new(Mutex::new(rec_limit));
        let wg = Arc::new(WaitGroup::new().await);
        let ip = self.param.ip.as_ref().unwrap();
        let mut timeout = self.param.timeout;
        if timeout == 0 {
            timeout = 800
        }

        for port in ports {
            sen_limit.send(1).await.unwrap();
            wg.add().await;

            let wg = wg.clone();
            let rec_limit = rec_limit.clone();
            let sen_file = sen_file.clone();
            let ip = ip.clone();
            tokio::spawn(async move {
                match Self::blasting(format!("{}:{}", ip, port), timeout).await {
                    Ok(data) => {
                        sen_file.send(data).unwrap();
                        // sen_file.send(data).await.unwrap();
                    }
                    _ => {}
                }
                wg.done().await;

                rec_limit.lock().await.recv().await.unwrap();
            });
        }

        wg.wait().await;

        Ok(())
    }

    async fn blasting(addr: String, timeout: u64) -> Result<String> {
        // let conn: std::result::Result<async_std::net::TcpStream, std::io::Error> = TcpStream::connect(&addr).timeout(Duration::from_millis(timeout)).await?;
        let conn = time::timeout(
            time::Duration::from_millis(timeout),
            TcpStream::connect(&addr),
        ).await;

        match conn {
            Ok(r) => {
                if let Ok(_) = r {
                    return Ok(addr)
                };

                Err("conn error".into())
            }
            _ => {
                Err("conn error".into())
            }
        }
    }
}