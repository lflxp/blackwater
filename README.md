# blackwater
Blaskwate rust tool for: 
1. port scanner
2. ip range ping

Scan all ports in one second, basically no missing
### Install
`sudo curl -L "https://cdn.jsdelivr.net/gh/dollarkillerx/st/blackwater/v1.0.1/blackwater" -o /usr/local/bin/blackwater`

`sudo chmod +x /usr/local/bin/blackwater`

### Usage
`blackwater -h`

``` 
blackwater 0.1.0
Asynchronous Port Scanner written in rust  https://github.com/dollarkillerx/blackwater

USAGE:
    blackwater [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -u, --udp        Scanning with UDP
    -V, --version    Prints version information

OPTIONS:
    -c, --concurrency <concurrency>    Number of concurrent scans [default: 65535]
    -i, --ip <ip>                      Scanned IP Range address eg: 127.0.0.1-255 
    -f, --outfile <outfile>            Result output file address
    -p, --port <port>                  Port Range <port,port,port> or <port-port> [default:
                                       21,22,23,25,69,79,80,88,110,113,119,220,443,1433,1521,2082,2083,2086,2087,2095,2096,2077,2078,3306,3389,5432,6379,8080,9000,9001,9200,9300,11211,27017]
    -t, --timeout <timeout>            Timeout  Milliseconds [default: 800]
```

### speed
Amazing speed Not a single port is missed
``` 
➜  38202 blackwater -i 172.20.176.1-10 -p1-65535

 ▄▄▄▄    ██▓     ▄▄▄       ▄████▄   ██ ▄█▀ █     █░ ▄▄▄      ▄▄▄█████▓▓█████  ██▀███
▓█████▄ ▓██▒    ▒████▄    ▒██▀ ▀█   ██▄█▒ ▓█░ █ ░█░▒████▄    ▓  ██▒ ▓▒▓█   ▀ ▓██ ▒ ██▒
▒██▒ ▄██▒██░    ▒██  ▀█▄  ▒▓█    ▄ ▓███▄░ ▒█░ █ ░█ ▒██  ▀█▄  ▒ ▓██░ ▒░▒███   ▓██ ░▄█ ▒
▒██░█▀  ▒██░    ░██▄▄▄▄██ ▒▓▓▄ ▄██▒▓██ █▄ ░█░ █ ░█ ░██▄▄▄▄██ ░ ▓██▓ ░ ▒▓█  ▄ ▒██▀▀█▄
░▓█  ▀█▓░██████▒ ▓█   ▓██▒▒ ▓███▀ ░▒██▒ █▄░░██▒██▓  ▓█   ▓██▒  ▒██▒ ░ ░▒████▒░██▓ ▒██▒
░▒▓███▀▒░ ▒░▓  ░ ▒▒   ▓▒█░░ ░▒ ▒  ░▒ ▒▒ ▓▒░ ▓░▒ ▒   ▒▒   ▓▒█░  ▒ ░░   ░░ ▒░ ░░ ▒▓ ░▒▓░
▒░▒   ░ ░ ░ ▒  ░  ▒   ▒▒ ░  ░  ▒   ░ ░▒ ▒░  ▒ ░ ░    ▒   ▒▒ ░    ░     ░ ░  ░  ░▒ ░ ▒░
 ░    ░   ░ ░     ░   ▒   ░        ░ ░░ ░   ░   ░    ░   ▒     ░         ░     ░░   ░
 ░          ░  ░      ░  ░░ ░      ░  ░       ░          ░  ░            ░  ░   ░
      ░                   ░

Black Water v1.0.1
Asynchronous Port Scanner written in rust
https://github.com/lflxp/blackwater

Response from host 172.20.176.1 (address 172.20.176.1): latency 0.281 ms
No response from host 172.20.176.2 (loss)
No response from host 172.20.176.3 (loss)
No response from host 172.20.176.4 (loss)
No response from host 172.20.176.5 (loss)
No response from host 172.20.176.6 (loss)
No response from host 172.20.176.7 (loss)
No response from host 172.20.176.8 (loss)
No response from host 172.20.176.9 (loss)
Index 0 IP 172.20.176.1 scanning
172.20.176.1:139
172.20.176.1:135
172.20.176.1:445
Ip success
--- Ping result ---
TOTAL  : 9 packets
SUCCESS: 1
FAILURE: 8
TIME   : 0.031 ms
Times: 1018.000 ms

ubuntu@ubuntu:~/$ time ./blackwater -i www.baidu.com -p1-65535
 
 ▄▄▄▄    ██▓     ▄▄▄       ▄████▄   ██ ▄█▀ █     █░ ▄▄▄      ▄▄▄█████▓▓█████  ██▀███  
▓█████▄ ▓██▒    ▒████▄    ▒██▀ ▀█   ██▄█▒ ▓█░ █ ░█░▒████▄    ▓  ██▒ ▓▒▓█   ▀ ▓██ ▒ ██▒
▒██▒ ▄██▒██░    ▒██  ▀█▄  ▒▓█    ▄ ▓███▄░ ▒█░ █ ░█ ▒██  ▀█▄  ▒ ▓██░ ▒░▒███   ▓██ ░▄█ ▒
▒██░█▀  ▒██░    ░██▄▄▄▄██ ▒▓▓▄ ▄██▒▓██ █▄ ░█░ █ ░█ ░██▄▄▄▄██ ░ ▓██▓ ░ ▒▓█  ▄ ▒██▀▀█▄  
░▓█  ▀█▓░██████▒ ▓█   ▓██▒▒ ▓███▀ ░▒██▒ █▄░░██▒██▓  ▓█   ▓██▒  ▒██▒ ░ ░▒████▒░██▓ ▒██▒
░▒▓███▀▒░ ▒░▓  ░ ▒▒   ▓▒█░░ ░▒ ▒  ░▒ ▒▒ ▓▒░ ▓░▒ ▒   ▒▒   ▓▒█░  ▒ ░░   ░░ ▒░ ░░ ▒▓ ░▒▓░
▒░▒   ░ ░ ░ ▒  ░  ▒   ▒▒ ░  ░  ▒   ░ ░▒ ▒░  ▒ ░ ░    ▒   ▒▒ ░    ░     ░ ░  ░  ░▒ ░ ▒░
 ░    ░   ░ ░     ░   ▒   ░        ░ ░░ ░   ░   ░    ░   ▒     ░         ░     ░░   ░ 
 ░          ░  ░      ░  ░░ ░      ░  ░       ░          ░  ░            ░  ░   ░     
      ░                   ░                                                           

Black Water
Asynchronous Port Scanner written in rust
https://github.com/dollarkillerx/blackwater

www.baidu.com:80
www.baidu.com:2000
www.baidu.com:443
www.baidu.com:5060

real    0m1.151s
user    0m0.654s
sys     0m0.697s

ubuntu@ubuntu:~/$ time ./blackwater -i www.bing.com -p1-65535
 
 ▄▄▄▄    ██▓     ▄▄▄       ▄████▄   ██ ▄█▀ █     █░ ▄▄▄      ▄▄▄█████▓▓█████  ██▀███  
▓█████▄ ▓██▒    ▒████▄    ▒██▀ ▀█   ██▄█▒ ▓█░ █ ░█░▒████▄    ▓  ██▒ ▓▒▓█   ▀ ▓██ ▒ ██▒
▒██▒ ▄██▒██░    ▒██  ▀█▄  ▒▓█    ▄ ▓███▄░ ▒█░ █ ░█ ▒██  ▀█▄  ▒ ▓██░ ▒░▒███   ▓██ ░▄█ ▒
▒██░█▀  ▒██░    ░██▄▄▄▄██ ▒▓▓▄ ▄██▒▓██ █▄ ░█░ █ ░█ ░██▄▄▄▄██ ░ ▓██▓ ░ ▒▓█  ▄ ▒██▀▀█▄  
░▓█  ▀█▓░██████▒ ▓█   ▓██▒▒ ▓███▀ ░▒██▒ █▄░░██▒██▓  ▓█   ▓██▒  ▒██▒ ░ ░▒████▒░██▓ ▒██▒
░▒▓███▀▒░ ▒░▓  ░ ▒▒   ▓▒█░░ ░▒ ▒  ░▒ ▒▒ ▓▒░ ▓░▒ ▒   ▒▒   ▓▒█░  ▒ ░░   ░░ ▒░ ░░ ▒▓ ░▒▓░
▒░▒   ░ ░ ░ ▒  ░  ▒   ▒▒ ░  ░  ▒   ░ ░▒ ▒░  ▒ ░ ░    ▒   ▒▒ ░    ░     ░ ░  ░  ░▒ ░ ▒░
 ░    ░   ░ ░     ░   ▒   ░        ░ ░░ ░   ░   ░    ░   ▒     ░         ░     ░░   ░ 
 ░          ░  ░      ░  ░░ ░      ░  ░       ░          ░  ░            ░  ░   ░     
      ░                   ░                                                           

Black Water
Asynchronous Port Scanner written in rust
https://github.com/dollarkillerx/blackwater

www.bing.com:80
www.bing.com:443
www.bing.com:2000

real    0m1.079s
user    0m0.589s
sys     0m0.442s

ubuntu@ubuntu:~/$ time ./blackwater -i github.com -p1-65535 

 ▄▄▄▄    ██▓     ▄▄▄       ▄████▄   ██ ▄█▀ █     █░ ▄▄▄      ▄▄▄█████▓▓█████  ██▀███  
▓█████▄ ▓██▒    ▒████▄    ▒██▀ ▀█   ██▄█▒ ▓█░ █ ░█░▒████▄    ▓  ██▒ ▓▒▓█   ▀ ▓██ ▒ ██▒
▒██▒ ▄██▒██░    ▒██  ▀█▄  ▒▓█    ▄ ▓███▄░ ▒█░ █ ░█ ▒██  ▀█▄  ▒ ▓██░ ▒░▒███   ▓██ ░▄█ ▒
▒██░█▀  ▒██░    ░██▄▄▄▄██ ▒▓▓▄ ▄██▒▓██ █▄ ░█░ █ ░█ ░██▄▄▄▄██ ░ ▓██▓ ░ ▒▓█  ▄ ▒██▀▀█▄  
░▓█  ▀█▓░██████▒ ▓█   ▓██▒▒ ▓███▀ ░▒██▒ █▄░░██▒██▓  ▓█   ▓██▒  ▒██▒ ░ ░▒████▒░██▓ ▒██▒
░▒▓███▀▒░ ▒░▓  ░ ▒▒   ▓▒█░░ ░▒ ▒  ░▒ ▒▒ ▓▒░ ▓░▒ ▒   ▒▒   ▓▒█░  ▒ ░░   ░░ ▒░ ░░ ▒▓ ░▒▓░
▒░▒   ░ ░ ░ ▒  ░  ▒   ▒▒ ░  ░  ▒   ░ ░▒ ▒░  ▒ ░ ░    ▒   ▒▒ ░    ░     ░ ░  ░  ░▒ ░ ▒░
 ░    ░   ░ ░     ░   ▒   ░        ░ ░░ ░   ░   ░    ░   ▒     ░         ░     ░░   ░ 
 ░          ░  ░      ░  ░░ ░      ░  ░       ░          ░  ░            ░  ░   ░     
      ░                   ░                                                           

Black Water
Asynchronous Port Scanner written in rust
https://github.com/dollarkillerx/blackwater

github.com:2000
github.com:22
github.com:80
github.com:443
github.com:5060

real    0m1.137s
user    0m0.685s
sys     0m0.756s
```

### build
`make build`

### Development Plan
- [ ] c-segment scanning
- [ ] File import batch scanning
- [ ] Distributed
- [ ] Fingerprint recognition

## Parameter adjustment, solve the problem of packet loss
There are many reasons for packet loss, roughly divided into two. 
1. network problems (solution: 1. switch to a better network 2. modify the `-t` timeout time) 
2. cpu processing super link performance is insufficient (solution: reduce the number of concurrent)
- Extranet scan, not missing a port
    - Modify thread parameters `-c` Calculation formula: Number of current CPU logical cores * 100
    - Example: Current CPU logical core is 4 cores Parameter is `blackwater -i github.com -c 400 -p 1-65535 `
- Intranet scan, not missing a port
    - Modify thread parameters `-c` Calculation formula: Number of current CPU logical cores * 250
    - Example: Current CPU logical core is 4 cores Parameter is `blackwater -i 192.168.88.11 -c 1000 -p 1-65535 `
