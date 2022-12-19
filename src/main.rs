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

use futures::{select, FutureExt};

use crate::{
    camera::{BayerPattern, Camera, CameraInfo, CameraManager, ControlCaps, ImageType},
    packet::{ConnectedCamerasPacket, Requests, Responses},
};

mod asi;
mod bytes;
mod camera;
mod packet;

const MAGIC: &[u8] = b"A51C4P";

fn main() {
    task::block_on(app()).unwrap();
}

async fn app() -> std::io::Result<()> {
    let camera_manager = Arc::new(Mutex::new(CameraManager::new().await.unwrap()));
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
            let (stream, _address) = tcp_listener.accept().await?;
            let cam_manager = camera_manager.clone();
            task::spawn(async move { handle_new_connection(stream, cam_manager).await.unwrap() });
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
        .write_all(&(cam_manager.lock().await.connected_cams_count().await as u8).to_be_bytes())
        .await?;

    udp_socket.send_to(&cursor.into_inner(), src).await?;

    Ok(())
}

#[allow(unused_variables, unreachable_code)]
async fn handle_new_connection(
    mut stream: TcpStream,
    cam_manager: Arc<Mutex<CameraManager>>,
) -> std::io::Result<()> {
    dbg!();

    loop {
        let mut id = [0u8; 1];
        stream.read_exact(&mut id).await.unwrap();

        if id[0] != 0xA5 {
            continue;
        }

        let mut seq = [0u8; 4];
        stream.read_exact(&mut seq).await.unwrap();
        let seq_num = u32::from_be_bytes(seq);

        let request = Requests::decode(&mut stream).await.unwrap();

        let mut response = Responses::None;
        match request {
            Requests::GetConnectedCameras => {
                let cams = vec![
                    Camera {
                        id: 0,
                        info: CameraInfo {
                            name: "ASI178MM".to_string(),
                            camera_id: 0,
                            max_height: 100,
                            max_width: 100,
                            is_color_cam: false,
                            bayer_pattern: BayerPattern::BG,
                            supported_bins: vec![1, 2, 3, 4],
                            supported_video_format: vec![ImageType::Raw8, ImageType::Raw16],
                            pixel_size: 0.2,
                            mechanical_shutter: false,
                            st4_port: true,
                            is_cooler_cam: false,
                            is_usb3_host: true,
                            is_usb3_camera: true,
                            elec_per_adu: 0.21,
                            bit_depth: 10,
                            is_trigger_cam: false,
                        },
                        controls: vec![ControlCaps {
                            name: "IDK".to_string(),
                            description: "I don't know".to_string(),
                            max_value: 1000,
                            min_value: 20,
                            default_value: 490,
                            is_auto_supported: false,
                            is_writable: true,
                            control_type: camera::ControlType::Exposure,
                        }],
                    },
                    Camera {
                        id: 0,
                        info: CameraInfo {
                            name: "ASI183MC".to_string(),
                            camera_id: 0,
                            max_height: 100,
                            max_width: 100,
                            is_color_cam: true,
                            bayer_pattern: BayerPattern::BG,
                            supported_bins: vec![1, 2, 3, 4],
                            supported_video_format: vec![ImageType::Raw8, ImageType::Raw16],
                            pixel_size: 0.2,
                            mechanical_shutter: false,
                            st4_port: true,
                            is_cooler_cam: false,
                            is_usb3_host: true,
                            is_usb3_camera: true,
                            elec_per_adu: 0.21,
                            bit_depth: 10,
                            is_trigger_cam: false,
                        },
                        controls: vec![ControlCaps {
                            name: "IDK".to_string(),
                            description: "I don't know".to_string(),
                            max_value: 1000,
                            min_value: 20,
                            default_value: 490,
                            is_auto_supported: false,
                            is_writable: true,
                            control_type: camera::ControlType::Exposure,
                        }],
                    },
                ];
                response = Responses::ConnectedCameras(ConnectedCamerasPacket(cams));
            }
            _ => {}
        }

        stream.write(&[0xA5]).await.unwrap();
        let seq_bytes = seq_num.to_be_bytes();
        stream.write(&seq_bytes).await.unwrap();
        response.encode(&mut stream).await.unwrap();
    }

    Ok(())
}
