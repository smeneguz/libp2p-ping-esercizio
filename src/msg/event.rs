use crate::msg::{req::Req, res::Res};

#[derive(Debug)]

pub enum Event {
    Message(request_response::Event<Req,Res>),
}

impl From<request_response