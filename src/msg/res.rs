use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Res {
    PeerList(Vec<Multiaddr>),
}
