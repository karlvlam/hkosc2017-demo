extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = "0.0.0.0:12345".parse().unwrap();
    let src_tcp = TcpListener::bind(&addr, &handle).unwrap();

    println!("Listening on 0.0.0.0:{}",
             src_tcp.local_addr().unwrap().port());


    let server = src_tcp.incoming().for_each(|(src_stream, remote_addr)| {
        println!("Connection from : {}", &remote_addr);
        

        let (src_read, src_write) = src_stream.split();

        let addr = "127.0.0.1:22".parse().unwrap();
        let send_data = TcpStream::connect(&addr, &handle).and_then(|dst_stream| {
            let (dst_read, dst_write) = dst_stream.split();

            // port forward (copy bytes between 2 connections)
            let dst_to_src = io::copy(dst_read, src_write);
            let src_to_dst = io::copy(src_read, dst_write);

            dst_to_src.join(src_to_dst)

        }).map(|(_client_to_server,_server_to_client)| {
            println!("Disconnected!");
        }).map_err(|_err|{
            println!("{}", _err);
        });

        handle.spawn(send_data);

        Ok(())
    });

    // start the event loop
    core.run(server).unwrap();
}
