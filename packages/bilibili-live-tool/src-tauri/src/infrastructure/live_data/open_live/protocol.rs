#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Packet {
    pub version: u16,
    pub operation: u32,
    pub sequence: u32,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ProtocolError {
    #[error("invalid open live packet")]
    InvalidPacket,
    #[error("open live packet compression is invalid")]
    InvalidCompression,
}

pub fn encode_packet(version: u16, operation: u32, sequence: u32, body: &[u8]) -> Vec<u8> {
    let packet_length = HEADER_LENGTH
        .checked_add(body.len())
        .and_then(|length| u32::try_from(length).ok())
        .expect("open live packet fits into u32");
    let mut packet = Vec::with_capacity(packet_length as usize);
    packet.extend_from_slice(&packet_length.to_be_bytes());
    packet.extend_from_slice(&(HEADER_LENGTH as u16).to_be_bytes());
    packet.extend_from_slice(&version.to_be_bytes());
    packet.extend_from_slice(&operation.to_be_bytes());
    packet.extend_from_slice(&sequence.to_be_bytes());
    packet.extend_from_slice(body);
    packet
}

pub fn decode_packets(bytes: &[u8]) -> Result<Vec<Packet>, ProtocolError> {
    decode_packets_at_depth(bytes, 0)
}

fn decode_packets_at_depth(bytes: &[u8], depth: usize) -> Result<Vec<Packet>, ProtocolError> {
    if bytes.is_empty() || bytes.len() > MAX_PACKET_LENGTH || depth > MAX_NESTING_DEPTH {
        return Err(ProtocolError::InvalidPacket);
    }
    let mut offset = 0;
    let mut packets = Vec::new();
    while offset < bytes.len() {
        let remaining = &bytes[offset..];
        if remaining.len() < HEADER_LENGTH {
            return Err(ProtocolError::InvalidPacket);
        }
        let packet_length = read_u32(&remaining[0..4])? as usize;
        let header_length = read_u16(&remaining[4..6])? as usize;
        let version = read_u16(&remaining[6..8])?;
        let operation = read_u32(&remaining[8..12])?;
        let sequence = read_u32(&remaining[12..16])?;
        if header_length != HEADER_LENGTH
            || packet_length < header_length
            || packet_length > remaining.len()
            || packet_length > MAX_PACKET_LENGTH
        {
            return Err(ProtocolError::InvalidPacket);
        }
        let body = &remaining[header_length..packet_length];
        match version {
            0 | 1 => packets.push(Packet {
                version,
                operation,
                sequence,
                body: body.to_vec(),
            }),
            2 => {
                let mut decoded = Vec::new();
                ZlibDecoder::new(body)
                    .take((MAX_PACKET_LENGTH + 1) as u64)
                    .read_to_end(&mut decoded)
                    .map_err(|_| ProtocolError::InvalidCompression)?;
                if decoded.len() > MAX_PACKET_LENGTH {
                    return Err(ProtocolError::InvalidCompression);
                }
                packets.extend(decode_packets_at_depth(&decoded, depth + 1)?);
            }
            _ => return Err(ProtocolError::InvalidPacket),
        }
        offset += packet_length;
    }
    Ok(packets)
}

fn read_u16(bytes: &[u8]) -> Result<u16, ProtocolError> {
    bytes
        .try_into()
        .map(u16::from_be_bytes)
        .map_err(|_| ProtocolError::InvalidPacket)
}

fn read_u32(bytes: &[u8]) -> Result<u32, ProtocolError> {
    bytes
        .try_into()
        .map(u32::from_be_bytes)
        .map_err(|_| ProtocolError::InvalidPacket)
}
use std::io::Read;

use flate2::read::ZlibDecoder;

const HEADER_LENGTH: usize = 16;
const MAX_PACKET_LENGTH: usize = 8 * 1024 * 1024;
const MAX_NESTING_DEPTH: usize = 8;
