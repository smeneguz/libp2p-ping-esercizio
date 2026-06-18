use std::time::Duration;

use libp2p::{
    Multiaddr, futures::StreamExt, mdns, noise, ping,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};

pub mod msg;

#[derive(NetworkBehaviour)]
struct Behaviour {
    ping: ping::Behaviour,
    mdns: mdns::tokio::Behaviour,
}



// tokyo rende il main async 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { // non sapendo quale è il tipo dell'errore e quello sarebbe un trait devo utilizzare un qualcosa di cui so già lo spazio
    // paradigma del builder, tramite classe builder ci permette per costruire e istanziare pezzi, dobbiamo dirgli identità di questo, nella blockchain leggeremo in un file di configurazione , questo perchè l'identità del nodo deve rimanere consistente con identità definita. mentre qua per gioco le istanziamo al volo.
    // libp2p::SwarmBuilder::with_existing_identity(Keypair::generate_ed25519) questo fa uguale
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(), //impl di default per questa struct
            noise::Config::new,  // riceve un puntatore a funzione e vogliamo utilizzare noise come protocollo di handshake
            yamux::Config::default, 
        )? // operazione fallibile quindi mettiamo il punto interrogativo
        .with_behaviour(|key| -> Result<_, Box<dyn std::error::Error + Send + Sync>> { // riceve una funzione behaviour, chiava del nostro nodo e restituiamo ping::Behaviour::default()?
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
        .build(); // creatp lo swarm e adesso possiamo metterlo in ascolto

    println!("Local peer id: {}", swarm.local_peer_id());

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?; // porta 0 lasciamo che sia il sistema operativo a sceglierla

    if let Some(addr) = std::env::args().nth(1) {
        let multiaddr: Multiaddr = addr.parse()?;
        swarm.dial(multiaddr)?;
    }


    // dobbiamo fare un loop infinito che sta in ascolto
    loop {
        match swarm.select_next_some().await { // dobbiamo usarlo in un contesto di pattern matching
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {:?}", address)
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => { // .. perchè ci interessa solo peer_id
                println!("New connection with {}", peer_id)
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                println!("Connection closed with {}", peer_id)
            }

            // SwarmEvent::Behaviour(event: Event)
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
