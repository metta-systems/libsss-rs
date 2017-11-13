//
// Part of Metta OS. Check https://metta.systems for latest version.
//
// Copyright 2007 - 2017, Stanislav Karchebnyy <berkus@metta.systems>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use std::io;
use std::sync::Arc;
use std::net::{SocketAddr, IpAddr, Ipv4Addr, ToSocketAddrs};
use tokio_core::reactor::Core;
use tokio_core::net::UdpSocket;

const MIN_PACKET_SIZE: usize = 64;
const MAX_PACKET_SIZE: usize = 1280;

#[derive(Debug)]
struct Handler;

/// This struct encapsulates all per-host state used by the sss protocol.
/// By centralizing this state here instead of using global/static variables,
/// the host environment can be virtualized for simulation purposes
/// and multiple sss instances can be run in one process.
#[derive(Debug)]
pub struct Host {
    io_loop: Core,
    handler: Arc<Handler>,
}

impl Host {
    pub fn new() -> io::Result<Host> {
        Ok(Host {
            io_loop: try!(Core::new()),
            handler: Arc::new(handler),
        })
    }

    pub fn register_socket(self, socket: UdpSocket) {
        let (buf_stream, stream_handle) = UdpStream::with_bound(socket, &self.io_loop.handle());
        let request_stream = RequestStream::new(buf_stream, stream_handle);
        let handler = self.handler.clone();

        // this spawns a ForEach future which handles all the requests into a Handler.
        self.io_loop.handle().spawn(
            request_stream
                .for_each(move |(request, response_handle)| {
                    Self::handle_request(request, response_handle, handler.clone())
                })
                .map_err(|e| debug!("error in UDP request_stream handler: {}", e)),
        );
    }

    pub fn listen(&mut self) -> io::Result<()> {
        info!("Host starting up");
        try!(self.io_loop.run(Forever));

        Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "Server interrupted",
        ))
    }

    fn handle_request(
        request: Request,
        mut response_handle: ResponseHandle,
        handler: Arc<Handler>,
    ) -> io::Result<()> {
        let response = handler.handle_request(&request);
        response_handle.send(response)
    }
}

// Based on https://github.com/bluejekyll/trust-dns/blob/master/server/src/named.rs
fn startup() {
    let handle = Core::new().expect("Cannot create reactor");

    let v4addr = config.get_listen_addrs_ipv4();
    let v6addr = config.get_listen_addrs_ipv6();
    let mut listen_addrs: Vec<IpAddr> = v4addr
        .into_iter()
        .map(|x| IpAddr::V4(x))
        .chain(v6addr.into_iter().map(|x| IpAddr::V6(x)))
        .collect();
    let listen_port: u16 = args.flag_port.unwrap_or(config.get_listen_port());

    if listen_addrs.len() == 0 {
        listen_addrs.push(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
    }
    let sockaddrs: Vec<SocketAddr> = listen_addrs
        .iter()
        .flat_map(|x| (*x, listen_port).to_socket_addrs().unwrap())
        .collect();
    let udp_sockets: Vec<UdpSocket> = sockaddrs
        .iter()
        .map(|x| {
            UdpSocket::bind(x, &handle).expect(&format!("could not bind to udp: {}", x))
        })
        .collect();

    // now, run the server, based on the config
    let mut host = Host::new().expect("error creating Host");

    // load all the listeners
    for udp_socket in udp_sockets {
        info!("listening for UDP on {:?}", udp_socket);
        host.register_socket(udp_socket);
    }

    if let Err(e) = host.listen() {
        error!("failed to listen: {}", e);
    }

    // will exit when finished - @todo
}
