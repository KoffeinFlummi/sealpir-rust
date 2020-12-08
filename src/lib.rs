extern crate libc;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PirQuery {
    #[serde(with = "serde_bytes")]
    pub query: Vec<u8>,
    pub num: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PirReply {
    #[serde(with = "serde_bytes")]
    pub reply: Vec<u8>,
    pub num: u32,
}

pub mod client;
pub mod server;
