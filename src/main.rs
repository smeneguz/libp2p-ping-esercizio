use std::time::Duration;

use libp2p::{Multiaddr, futures::StreamExt, noise, ping, swarm::SwarmEvent, tcp, yamux};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| ping::Behaviour::default())?
        .with_swarm_config(|config| {
            config.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
        })
        .build();

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
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }
}