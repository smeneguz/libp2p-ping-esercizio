use std::io;

use async_trait::async_trait;
use libp2p::{
    Multiaddr,
    futures::{AsyncReadExt, AsyncWrite, AsyncWriteExt},
    request_response,
};

use crate::msg::{protocol::Protocol, req::Req, res::Res};

#[derive(Clone, Debug)]
pub struct Codec;

impl Default for Codec {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait]
impl request_response::Codec for Codec {
    type Protocol = Protocol;
    type Request = Req;
    type Response = Res;

    async fn read_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncReadExt + Send + Unpin,
    {
        let mut buf = [0u8; 1];
        io.read_exact(&mut buf).await?;

        let tag = u8::from_be_bytes(buf);
        let req = match tag {
            0x00 => Ok(Req::GetPeers),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid request tag 0x{:02x}", tag),
            )),
        };

        req
    }

    async fn read_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncReadExt + Unpin + Send,
    {
        let mut buf = [0u8; 1];
        io.read_exact(&mut buf).await?;

        let tag = u8::from_be_bytes(buf);

        let res = match tag {
            0x00 => {
                let mut peer_count_buf = [0u8; 1];
                io.read_exact(&mut peer_count_buf).await?;
                let peer_count = u8::from_be_bytes(peer_count_buf);

                let mut peers: Vec<Multiaddr> = vec![];

                for _ in 0..peer_count {
                    let mut len_buf = [0u8; 4];
                    io.read_exact(&mut len_buf).await?;
                    let len = u32::from_be_bytes(len_buf) as usize;

                    let mut data = vec![0u8; len];
                    io.read_exact(&mut data).await?;

                    let s = std::str::from_utf8(&data)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                    let peer_addr = s
                        .parse::<Multiaddr>()
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                    peers.push(peer_addr);
                }

                Ok(Res::PeerList(peers))
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid response tag 0x{:02x}", tag),
            )),
        };

        res
    }

    async fn write_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Send + Unpin,
    {
        let tag: u8 = match req {
            Req::GetPeers => 0x00 as u8,
        };

        io.write_all(&tag.to_be_bytes()).await?;
        io.flush().await
    }

    async fn write_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let mut frame = Vec::<u8>::new();

        let frame = match res {
            Res::PeerList(peers) => {
                let tag = 0x00 as u8;
                let peer_count = u8::try_from(peers.len())
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Too many peers"))?;

                frame.push(tag);
                frame.push(peer_count);

                for peer in peers {
                    let s = peer.to_string();
                    let s_len = u32::try_from(s.len()).map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Peer is too long")
                    })?;

                    frame.extend_from_slice(&s_len.to_be_bytes());
                    frame.extend_from_slice(&s.as_bytes());
                }

                frame
            }
        };

        io.write_all(&frame).await?;

        io.flush().await
    }
}
