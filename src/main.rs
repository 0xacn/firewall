use std::env;
use std::error::Error;
use std::net::IpAddr;
use std::process;

use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;

use tokio::stream::StreamExt;
use tokio::task;

#[tokio::main]
pub async fn main() {
    let interface_name = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("en0"));

    let interface = datalink::interfaces()
        .into_iter()
        .find(|iface: &NetworkInterface| iface.name == interface_name)
        .unwrap_or_else(|| panic!("Network interface doesn't exist: {}", interface_name));

    let (mut tx, mut rx) = tokio::sync::mpsc::channel(100);
}

async fn capture_packets(
    interface: NetworkInterface,
    mut tx: tokio::sync::mpsc::Sender<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    let (mut _tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };

    while let Some(packet) = rx.next().await {
        let packet = EthernetPacket::new(packet).unwrap();
        let payload = packet.payload();
        tx.send(payload.to_vec()).await.unwrap();
    }
}

async fn process_packet(packet: Vec<u8>) {
    if let Some(ethernet)
}