use libp2p::request_response;

use crate::msg::{req::Req, res::Res};

#[derive(Debug)]
pub enum Event {
    Message(request_response::Event<Req, Res>),
}

impl From<request_response::Event<Req, Res>> for Event {
    fn from(event: request_response::Event<Req, Res>) -> Self {
        Event::Message(event)
    }
}
