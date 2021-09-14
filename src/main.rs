use blackwater::*;
use structopt::StructOpt;
use stopwatch::{Stopwatch};

mod ping;
use ping::{pingmethod, print_result};

#[tokio::main]
async fn main() -> Result<()> {
    let opt: Params = Params::from_args();
    println!("{}", LOGO);
    if opt.ip == None {
        println!("Please -h");
        return Ok(());
    }

    // println!("{:#?}",opt);

    let ports = opt.get_ports().await?;
    if ports.len() == 0 {
        println!("Parameter Error");
        return Ok(());
    }

    // match opt.get_ips().await {
    //     Ok(data) => {
    //         if data.len() == 0 {
    //             println!("Parse IP Error");
    //             return Ok(());
    //         }

    //         for i in data {
    //             println!("push {}", i);
    //         }
    //     },
    //     _ => {}
    // }

    let ips = opt.get_ips().await?;
    if ips.len() == 0 {
        println!("IPS Len Error");
        return Ok(());
    }

    let start = Stopwatch::start_new();

    // ping ip 获取有效ip
    match pingmethod(ips).await {
        Ok(data) => {
            let mut core = Core::new(&opt).await;
            for (index,ip) in data.iter().enumerate() {
                println!("Index {} IP {} scanning", index, ip);
                match core.runasip(ports.clone(), ip.to_string()).await {
                    Ok(_s) => {
                        println!("Ip success");
                    },
                    _ => {}
                };
            }
        }
        Err(e) => println!("{}", e),
    }

    print_result(start).await;

    // opt.ip = Option::from(ips[0].to_string());

    // let mut core = Core::new(&opt).await;
    // core.run(ports).await?;
    Ok(())
}
