use libp2p::Multiaddr;
#[derive(Clone,Debug)]
pub enum Res {
    PeerList(Vec<Multiaddr>),
}