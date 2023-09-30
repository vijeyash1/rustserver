
use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
pub struct Server {
    addr : String
    }

    impl Server {
    pub fn new(addr:String)->Self{
        Self {
        addr,
            }
        }
    pub fn run(self){
        println!("listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
          match listener.accept() {
            Ok((mut stream,_)) => {
                let mut buffer:[u8;1024] = [0;1024];
               match stream.read(&mut buffer) {
                Ok(_)=>{
                    println!("{}",String::from_utf8_lossy(&buffer))
                    Request::try_from(&buffer);
                },
                Err(e)=>println!("error while reading the buffer {}",e)
               }
            },
            Err(e) => print!("{}",e)
          }

        }
        }
    }
