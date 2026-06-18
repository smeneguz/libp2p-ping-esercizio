use std::time::Duration;

use libp2p::{
    Multiaddr, core::ConnectedPoint, futures::StreamExt, identity::Keypair, noise, ping,
    swarm::SwarmEvent, tcp, yamux,
};

use crate::msg::{behavior, event, req::Req, res::Res};

mod msg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(Keypair::generate_ed25519())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_key| behavior::Behaviour::new())?
        .with_swarm_config(|config| {
            config.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
        })
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    println!("Listening with peer id {}", swarm.local_peer_id());

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;

        println!("Dialed {addr}");
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr {
                listener_id,
                address,
            } => {
                println!("Listening on address {address:?} with id {listener_id:?}");
            }
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                println!("A new friend has joined! {peer_id:?}");

                match endpoint {
                    ConnectedPoint::Dialer { .. } => {
                        swarm
                            .behaviour_mut()
                            .req_res
                            .send_request(&peer_id, Req::GetPeers);
                    }
                    _ => {}
                }
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                println!("A friend has left :( {peer_id:?} {cause:?}");
            }
            SwarmEvent::Behaviour(event) => match event {
                event::Event::Message(req_res) => match req_res {
                    libp2p::request_response::Event::Message { message, .. } => match message {
                        libp2p::request_response::Message::Request { channel, .. } => {
                            swarm.behaviour_mut().req_res.send_response(
                                channel,
                                Res::PeerList(vec![
                                    "/ip4/172.21.0.1/tcp/43383".parse::<Multiaddr>()?,
                                ]),
                            );
                        }
                        libp2p::request_response::Message::Response { response, .. } => {
                            println!("Yay! {:?}", response);
                        }
                    },
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}
