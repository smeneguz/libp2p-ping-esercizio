#[derive(Clone,Debug)]
pub struct Codec;
use std::fmt::format;

use libp2p::futures::{AsyncReadExt, AsyncWrite, AsyncWriteExt, io};

use crate::msg::{protocol::Protocol, req::Req, res::Res};

impl Default for Codec { 
    fn default() -> Self {
        Self{}
    }
}

//3 tipi
#[async_trait]
impl request_response::Codec for Codec {
    type Protocol = Protocol;
    type Request = Req;
    type Response = Res;


    async fn read_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T
    ) -> io::Result<Self::Request>
        where 
            T:AsyncReadExt + Send + Unpin
            {
                let mut buf: [u8;1] = [0u8;1];
                io.read_exact(&mut buf).await?;

                let tag = u8::from_be_bytes(buf);
                let req = match tag {
                    0x00 => Ok(Req::GetPeers),
                    _ => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("mamma mia")
                    )),
                };
                req
            }


    async fn write_request<T>(
        &mut self,
        _protocol: &Self::Protocol, // non ci servirà quindi metto _ davanti
        io: &mut T,
        req: Self::Request
    ) -> io::Result<()>
    // where per dare caratteristiche al generic T , send dice che questo riferimento può essere trasferito tra più thread e poi Unpin dato che la memoria non facciamo malloc a mano (allocazione memoria), e in certe situazioni potrebbe volere pigliare la roba e ficcarla dove ci sono dei buchi di byte, tutte le varibili di rust sono quasi tutte Unpin e possono essere appunto spostate in giro per la memoria. Quindi se scrivo Pin quella roba non si sposta, rimane allocata dove sta. invece con Unpin gli diciamo che il tipo T si sposta e si può spostare.
    where
        T:AsyncWrite + Send + Unpin,
    {
        let tag: u8 = match req {
            Req::GetPeers => 0x00 as u8
        };
        io.write_all(&tag.to_be_bytes()).await?;
        io.flush().await
    }
}