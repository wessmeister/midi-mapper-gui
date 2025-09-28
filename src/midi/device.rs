use alsa::seq::{self, Addr, PortSubscribe, Seq};
use std::ffi::CString;

const PORT_NAME: &str = "MIDI Mapper";

pub fn connect_midi_device(device_name: &str) -> Result<Seq, String> {
    // Find device
    let temp_seq = Seq::open(None, Some(alsa::Direction::Capture), true)
        .map_err(|e| format!("Failed to open ALSA: {}", e))?;

    let client_id = seq::ClientIter::new(&temp_seq)
        .find(|client| {
            client.get_client() >= 16
                && client.get_name().unwrap_or_default().contains(device_name)
        })
        .map(|c| c.get_client())
        .ok_or_else(|| format!("Device '{}' not found", device_name))?;

    // Create connection
    let seq = Seq::open(None, Some(alsa::Direction::Capture), true)
        .map_err(|e| format!("Failed to create sequencer: {}", e))?;

    let port_name = CString::new(PORT_NAME).unwrap();
    let port = seq
        .create_simple_port(
            &port_name,
            seq::PortCap::WRITE | seq::PortCap::SUBS_WRITE,
            seq::PortType::MIDI_GENERIC | seq::PortType::APPLICATION,
        )
        .map_err(|e| format!("Failed to create port: {}", e))?;

    // Find and subscribe to target port
    let target_port = seq::PortIter::new(&seq, client_id)
        .find(|p| p.get_capability().contains(seq::PortCap::READ))
        .map(|p| p.get_port())
        .ok_or("No readable port found")?;

    let sub = PortSubscribe::empty().map_err(|e| format!("Failed to create subscription: {}", e))?;
    sub.set_sender(Addr {
        client: client_id,
        port: target_port,
    });
    sub.set_dest(Addr {
        client: seq.client_id().unwrap(),
        port,
    });

    seq.subscribe_port(&sub)
        .map_err(|e| format!("Failed to subscribe: {}", e))?;

    Ok(seq)
}