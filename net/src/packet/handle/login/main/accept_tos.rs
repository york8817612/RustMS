use crate::{
    error::NetworkError,
    io::client::MapleClient,
    packet::{build, handle::PacketHandler},
};
use account::Account;
use db::account;
use packet::{io::read::PktRead, Packet};
use std::io::BufReader;

pub struct AcceptTOSHandler {}

impl AcceptTOSHandler {
    pub fn new() -> Self {
        AcceptTOSHandler {}
    }

    fn accept_logon(&self, client: &mut MapleClient, acc: Account) -> Result<(), NetworkError> {
        let mut return_packet = build::login::status::build_successful_login_packet(&acc);

        client.user = Some(acc);

        match client.send(&mut return_packet) {
            Ok(_) => Ok(()),
            Err(e) => Err(NetworkError::CouldNotSend(e)),
        }
    }
}

impl PacketHandler for AcceptTOSHandler {
    fn handle(&self, packet: &mut Packet, client: &mut MapleClient) -> Result<(), NetworkError> {
        let mut reader = BufReader::new(&**packet);
        reader.read_short().unwrap();

        let confirmed = reader.read_byte().unwrap();
        let user = client.user.take();

        match (confirmed, user) {
            (0x01, Some(mut user)) => {
                user.accepted_tos = true;

                let user = account::update_account(&user).unwrap();

                self.accept_logon(client, user)
            }
            _ => Err(NetworkError::PacketHandlerError(
                "Accept TOS packet is invalid.",
            )),
        }
    }
}
