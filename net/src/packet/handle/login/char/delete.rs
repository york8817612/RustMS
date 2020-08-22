use crate::{
    error::NetworkError,
    io::client::MapleClient,
    packet::{build, handle::PacketHandler},
};
use build::login;
use db::character;
use packet::{io::read::PktRead, Packet};
use std::io::BufReader;

pub struct DeleteCharHandler {}

impl DeleteCharHandler {
    pub fn new() -> Self {
        DeleteCharHandler {}
    }
}

impl PacketHandler for DeleteCharHandler {
    fn handle(&self, packet: &mut Packet, client: &mut MapleClient) -> Result<(), NetworkError> {
        let mut reader = BufReader::new(&**packet);
        reader.read_short()?;

        let _pic = reader.read_str_with_length()?;
        let character_id = reader.read_int()?;

        let user = client.user.take();
        let accountid: i32;

        match user {
            Some(acc) => {
                accountid = acc.id;
                client.user = Some(acc);
            }
            _ => panic!("No account found!"),
        }

        match character::delete_character(character_id, accountid) {
            Ok(_) => client.send(&mut login::char::build_char_delete(character_id, 0x00)?),
            Err(_) => Err(NetworkError::PacketHandlerError(
                "Could not delete character",
            )),
        }
    }
}
