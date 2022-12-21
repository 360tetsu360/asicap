use std::{
    io::Read,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    pin::Pin,
    sync::Arc,
    time::Duration,
};

use async_std::{
    channel,
    future::{timeout, TimeoutError},
    io::ReadExt,
    net::{TcpStream, UdpSocket},
    sync::Mutex,
    task::{self, JoinHandle},
};
use futures::{
    channel::oneshot::{self, Canceled},
    select, Future, FutureExt,
};

use crate::{
    packets::{Requests, Responses},
    MAGIC,
};

pub type SearchServer = Option<Pin<Box<dyn Future<Output = Result<ServerStatus, TimeoutError>>>>>;

pub struct NetworkManager {
    udp: Option<Arc<UdpSocket>>,
    stream: Option<TcpStream>,
    request_sender: Option<channel::Sender<NewRequest>>,
    switch_req_handle: Option<JoinHandle<()>>,
    next_id: u32,

    pub server_finder: SearchServer,
}

impl NetworkManager {
    pub fn new() -> Self {
        let udp = task::block_on(async {
            let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
            let udp = UdpSocket::bind(addr).await.unwrap();
            udp.set_broadcast(true).unwrap();
            Arc::new(udp)
        });

        Self {
            udp: Some(udp.clone()),
            stream: None,
            request_sender: None,
            switch_req_handle: None,
            next_id: 1,
            server_finder: Some(
                task::spawn(timeout(Duration::from_secs(3), find_server(udp))).boxed_local(),
            ),
        }
    }

    pub fn retry_find_server(&mut self) {
        self.server_finder = Some(
            task::spawn(timeout(
                Duration::from_secs(3),
                find_server(self.udp.clone().unwrap()),
            ))
            .boxed_local(),
        )
    }

    pub fn start_connection(&mut self, address: SocketAddr) -> std::io::Result<()> {
        let tcp = task::block_on(TcpStream::connect(address))?;

        let (sender, receiver) = channel::bounded(8);
        let handle = task::spawn(switch_res(tcp.clone(), receiver));

        self.stream = Some(tcp);
        self.request_sender = Some(sender);
        self.switch_req_handle = Some(handle);

        Ok(())
    }

    pub fn request_api(
        &mut self,
        packet: Requests,
    ) -> Pin<Box<dyn futures::Future<Output = Result<Responses, Canceled>>>> {
        let (sender, receiver) = oneshot::channel::<Responses>();

        let mut writer = self.stream.clone().unwrap();

        let id = self.next_id;
        task::spawn(async move {
            packet.encode(&mut writer, id).await.unwrap();
        });

        let request = NewRequest {
            id: self.next_id,
            sender,
        };
        self.next_id += 1;

        self.request_sender
            .clone()
            .unwrap()
            .send_blocking(request)
            .unwrap();

        receiver.boxed_local()
    }
}

#[derive(Debug)]
struct NewRequest {
    id: u32,
    sender: oneshot::Sender<Responses>,
}

async fn switch_res(mut tcp: TcpStream, newrequest_receiver: channel::Receiver<NewRequest>) {
    let requests: Arc<Mutex<Vec<NewRequest>>> = Arc::new(Mutex::new(vec![]));
    loop {
        let receive_tcp = async {
            let mut id = [0u8; 1];
            tcp.read_exact(&mut id).await.unwrap();
            if id[0] != 0xA5 {
                return;
            }

            let mut seq = [0u8; 4];
            tcp.read_exact(&mut seq).await.unwrap();
            let seq_number = u32::from_be_bytes(seq);

            // sequence number 0 is for system message.
            if seq_number == 0 {}

            let request_index = requests
                .lock()
                .await
                .iter()
                .position(|x| x.id == seq_number)
                .unwrap();

            let response = Responses::decode(&mut tcp).await.unwrap();

            requests
                .lock()
                .await
                .remove(request_index)
                .sender
                .send(response)
                .unwrap();
        };

        let receive_newreq = async {
            let new_request = newrequest_receiver.recv().await.unwrap();
            requests.lock().await.push(new_request);
        };

        select! {
            recvtcp = receive_tcp.fuse() => recvtcp,
            recvreq = receive_newreq.fuse() => recvreq,
        };
    }
}

#[derive(Debug)]
pub struct ServerStatus {
    pub ip: IpAddr,
    pub port: u16,
    pub connected_cams: u32,
}

async fn find_server(udp: Arc<UdpSocket>) -> ServerStatus {
    udp.send_to(
        MAGIC,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), 4510),
    )
    .await
    .unwrap();

    loop {
        let mut buffer = [0u8; 2048];
        let (size, src) = udp.recv_from(&mut buffer).await.unwrap();
        let mut cursor = std::io::Cursor::new(&buffer[..size]);

        let mut magic_buf = [0u8; 6];
        if cursor.read_exact(&mut magic_buf).is_err() {
            continue;
        }

        if magic_buf.ne(MAGIC) {
            continue;
        }

        let mut port_buf = [0u8; 2];
        let port = match cursor.read_exact(&mut port_buf) {
            Ok(_) => u16::from_be_bytes(port_buf),
            Err(_) => continue,
        };

        let mut cams_buf = [0u8; 1];
        let connected_cams = match cursor.read_exact(&mut cams_buf) {
            Ok(_) => cams_buf[0],
            Err(_) => continue,
        };

        return ServerStatus {
            ip: src.ip(),
            port,
            connected_cams: connected_cams as u32,
        };
    }
}
