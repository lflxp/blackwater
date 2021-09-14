use blackwater::*;
use structopt::StructOpt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let mut opt: Params = Params::from_args();
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

    let mut core = Core::new(&opt).await;
    for (index,ip) in ips.iter().enumerate() {
        println!("Index {} IP {} scanning", index, ip);
        match core.runasip(ports.clone(), ip.to_string()).await {
            Ok(_s) => {
                println!("Ip success");
            },
            _ => {}
        };
    }
    


    // opt.ip = Option::from(ips[0].to_string());

    // let mut core = Core::new(&opt).await;
    // core.run(ports).await?;
    Ok(())
}
