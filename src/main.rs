use std::time::Duration;

use libp2p::{
    Multiaddr, futures::StreamExt, mdns, noise, ping,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};

#[derive(NetworkBehaviour)]
struct Behaviour {
    ping: ping::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| -> Result<_, Box<dyn std::error::Error + Send + Sync>> {
            Ok(Behaviour {
                ping: ping::Behaviour::default(),
                mdns: mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?,
            })
        })?
        .with_swarm_config(|config| {
            config.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
        })
        .build();

    println!("Local peer id: {}", swarm.local_peer_id());

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let multiaddr: Multiaddr = addr.parse()?;
        swarm.dial(multiaddr)?;
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {:?}", address)
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                println!("New connection with {}", peer_id)
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                println!("Connection closed with {}", peer_id)
            }
            SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, addr) in list {
                    println!("mDNS discovered {peer_id} at {addr}");
                    let _ = swarm.dial(addr);
                }
            }
            SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                for (peer_id, _) in list {
                    println!("mDNS expired {peer_id}");
                }
            }
            SwarmEvent::Behaviour(BehaviourEvent::Ping(event)) => {
                println!("{:?}", event)
            }
            _ => {}
        }
    }
}
