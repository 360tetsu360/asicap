#![feature(cstr_from_bytes_until_nul)]
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use async_std::{
    io::{Cursor, ReadExt, WriteExt},
    net::{TcpListener, TcpStream, UdpSocket},
    sync::Mutex,
    task,
};
use camera::CameraManager;
use futures::{select, FutureExt};

mod asi;
mod camera;
mod packet;

const MAGIC: &[u8] = b"A51C4P";

fn main() {
    task::block_on(app()).unwrap();
}

async fn app() -> std::io::Result<()> {
    let camera_manager = Arc::new(Mutex::new(CameraManager::new().unwrap()));
    dbg!(&camera_manager);

    let udp_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), 4510);
    let tcp_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 4514);

    let udp_socket = UdpSocket::bind(udp_address).await?;
    let tcp_listener = TcpListener::bind(tcp_address).await?;

    loop {
        let cam_manager_udp = camera_manager.clone();
        let udp_future = async {
            let mut buffer = [0u8; 2048];
            let (size, src) = udp_socket.recv_from(&mut buffer).await?;
            handle_incoming(
                &udp_socket,
                &buffer[..size],
                src,
                cam_manager_udp,
                tcp_address,
            )
            .await?;
            Ok::<(), std::io::Error>(())
        };

        let tcp_future = async {
            let (stream, address) = tcp_listener.accept().await?;
            task::spawn(async move { handle_new_connection(stream, address).await.unwrap() });
            Ok::<(), std::io::Error>(())
        };

        select! {
            udp_res = udp_future.fuse() => udp_res,
            tcp_res = tcp_future.fuse() => tcp_res
        }?;
    }
}

async fn handle_incoming(
    udp_socket: &UdpSocket,
    buffer: &[u8],
    src: SocketAddr,
    cam_manager: Arc<Mutex<CameraManager>>,
    tcp_address: SocketAddr,
) -> std::io::Result<()> {
    if buffer.len() != 6 {
        return Ok(());
    }

    if buffer[..6].ne(MAGIC) {
        return Ok(());
    }

    let mut cursor = Cursor::new(Vec::<u8>::new());
    cursor.write_all(MAGIC).await?;

    cursor
        .write_all(&(tcp_address.port()).to_be_bytes())
        .await?;
    cursor
        .write_all(&(cam_manager.lock().await.connected_cams() as u8).to_be_bytes())
        .await?;

    udp_socket.send_to(&cursor.into_inner(), src).await?;

    Ok(())
}

async fn handle_new_connection(stream: TcpStream, _address: SocketAddr) -> std::io::Result<()> {
    let mut reader = stream.clone();
    let mut _writer = stream;

    let mut id_buf = [0u8; 1];
    reader.read_exact(&mut id_buf).await?;

    Ok(())
}
