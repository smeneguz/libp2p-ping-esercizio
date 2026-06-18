use libp2p::swarm::NetworkBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "Event")]

pub struct Behaviour{
    pub req_res: request_response::cbor::Behaviour<Req,Res>,
}

impl Behaviour {
    fn new() -> Self {
        let config: Config = request_response::Config::default();

        let protocol: impl Iterator<Item = (StramProtocol, ..)> = std::iter::once((
            StreamProtocol::new("/msg/1.0")
        ))
    }
}