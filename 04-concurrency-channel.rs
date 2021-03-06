use std::env;
use std::fs::File;
use std::io::*;
use std::net::*;
use std::{thread, time};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::HashMap;

fn main(){

    let src_addr = "0.0.0.0:12345";
    let dest_addr = "127.0.0.1:22";
    // listener thread
    thread::spawn( move|| {
        start_listener(src_addr, dest_addr);
    });

    // just keep the program running.
    loop {
        thread::sleep(time::Duration::from_millis(3600000));
    }

}

struct TcpBuffer {
    data: [u8; 128],
    length: usize
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

fn handle_client(src_stream: TcpStream, dest_addr: &str){

    // create channels for both side 
    let (dest_tx, dest_rx) : (Sender<TcpBuffer>, Receiver<TcpBuffer>) = channel();
    let (src_tx, src_rx) : (Sender<TcpBuffer>, Receiver<TcpBuffer>) = channel();

    let dest_connection = TcpStream::connect(dest_addr);
    let dest_stream;

    match dest_connection{
        Ok(stream) => {
            dest_stream = stream;
        }
        Err(_) => {
            println!("Dest Error!");
            return;
        }
    }

    // set non-blocking for both stream
    let _ = src_stream.set_nonblocking(true);
    let _ = dest_stream.set_nonblocking(true);

    // start threads to copy data between 2 streams 
    thread::spawn( move|| {
        pass_bytes(src_stream, dest_tx, src_rx);
    });
    thread::spawn( move|| {
        pass_bytes(dest_stream, src_tx, dest_rx);
    });
}

fn pass_bytes(mut stream: TcpStream, tx: Sender<TcpBuffer>, rx: Receiver<TcpBuffer>) {
    let mut buf: [u8; 128] = [0; 128];
    // keep reading data and push to other stream
    loop {
        let res = stream.read(&mut buf);
        match res {
            Ok(byte_count) => {
                if byte_count == 0 {
                    stream.shutdown(Shutdown::Both);
                    break;
                }
                // copy any received data to the TcpBuffer, with byte count
                // Then send to the opposite side
                tx.send(TcpBuffer{data:buf, length:byte_count});
            }
            Err(e) => {
                // read() will return error if no data, 
                // just sleep some time in order to release cpu resource
                thread::sleep(time::Duration::from_millis(5));
            }
        }
        match rx.try_recv() {
            Ok(TcpBuffer{data, length}) => {
                // simple, just write any received data to the stream
                stream.write(&data[0..length]);
            }
            Err(e) => {
                //println!("RECV error: {:?}", e);
            }
        }

    }
}


