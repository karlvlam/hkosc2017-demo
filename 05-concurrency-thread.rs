use std::io::*;
use std::net::*;
use std::{thread, time};


fn main(){

    let src_addr = "0.0.0.0:12345";
    let dest_addr = "127.0.0.1:22";
    thread::spawn( move|| {
        start_listener(src_addr, dest_addr);
    });

    loop {
        thread::sleep(time::Duration::from_millis(3600000));
    }

}

fn start_listener(src_addr: &str, dest_addr: &str) {
    let listener = TcpListener::bind(src_addr).unwrap();
    println!("Port forward started {} -> {}", src_addr, dest_addr);
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                let dest_addr = dest_addr.to_owned();
                thread::spawn( move|| {
                    handle_client(stream, &dest_addr);
                });
            }
            Err(_) => {
                println!("sth error!");
            }

        }
    }
}

fn handle_client(mut src_stream: TcpStream, dest_addr: &str){

    let dest_connection = TcpStream::connect(dest_addr);
    let mut dest_stream: TcpStream;

    match dest_connection{
        Ok(stream) => {
            dest_stream = stream;
        }
        Err(_) => {
            println!("Dest Error!");
            return;
        }
    }

    let _ = src_stream.set_nonblocking(true);
    let _ = dest_stream.set_nonblocking(true);

    let mut src_buf: [u8; 128] = [0; 128];
    let mut dest_buf: [u8; 128] = [0; 128];
    loop {
        let res = src_stream.read(&mut src_buf);
        match res {
            Ok(byte_count) => {
                if byte_count == 0 {
                    let _ = src_stream.shutdown(Shutdown::Both);
                    break;
                }
                let _ = dest_stream.write(&src_buf[0..byte_count]);
                //println!("{:?}", &buf[0 .. byte_count]);
            }
            Err(e) => {
                //println!("Error: {:?}", e);
                //stream.shutdown(Shutdown::Both);
                //break;
            }
        }

        let res = dest_stream.read(&mut dest_buf);
        match res {
            Ok(byte_count) => {
                if byte_count == 0 {
                    let _ = dest_stream.shutdown(Shutdown::Both);
                    break;
                }
                let _ = src_stream.write(&dest_buf[0..byte_count]);
                //println!("{:?}", &buf[0 .. byte_count]);
            }
            Err(e) => {
                //println!("Error: {:?}", e);
                //stream.shutdown(Shutdown::Both);
                //break;
                thread::sleep(time::Duration::from_millis(5));
            }
        }

    }


}
