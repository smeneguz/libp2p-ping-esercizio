use libp2p::{
    StreamProtocol,
    request_response::{self, ProtocolSupport},
    swarm::NetworkBehaviour,
};

use crate::msg::{event::Event, req::Req, res::Res};

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "Event")]
pub struct Behaviour {
    pub req_res: request_response::cbor::Behaviour<Req, Res>,
}

impl Behaviour {
    pub fn new() -> Self {
        let config = request_response::Config::default();

        let protocols = std::iter::once((StreamProtocol::new("/msg/1.0"), ProtocolSupport::Full));

        let req_res = request_response::cbor::Behaviour::new(protocols, config);

        Self { req_res }
    }
}
