use std::convert::TryFrom;

use super::utils::*;

pub type AtomKind = u64;

// TODO(marcelgarus): Add atom for packed bytes.
#[derive(Clone, Debug)]
pub enum Atom {
    Block { kind: AtomKind, num_children: u8 },
    Reference(u64),
    Bytes(Vec<u8>),
}

pub trait LengthInWords {
    fn length_in_words(&self) -> usize;
}
impl LengthInWords for Atom {
    fn length_in_words(&self) -> usize {
        use Atom::*;

        match self {
            Block { .. } => 1,
            Reference(_) => 1,
            Bytes(bytes) => 1 + (bytes.len().round_up_to_multiple_of(8)) / 8,
        }
    }
}

impl Atom {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Atom::Block { num_children, kind } => {
                let mut bytes = vec![0];
                bytes.push(*num_children);
                bytes.extend_from_slice(&kind.to_be_bytes()[2..]);
                bytes
            }
            Atom::Reference(offset) => {
                let mut bytes = vec![1];
                bytes.extend_from_slice(&offset.to_be_bytes()[1..]);
                bytes
            }
            Atom::Bytes(payload_bytes) => {
                let mut bytes = vec![2];
                bytes.extend_from_slice(
                    &u64::try_from(payload_bytes.len()).unwrap().to_be_bytes()[1..],
                );
                bytes.extend_from_slice(&payload_bytes);
                bytes.align();
                bytes
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Atom, ()> {
        match bytes.first().unwrap() {
            0 => Ok(Atom::Block {
                kind: u64::clone_from_slice(&bytes[0..8]) & 0x00_00_ff_ff_ff_ff_ff_ff,
                num_children: *bytes.get(1).unwrap(),
            }),
            1 => {
                let offset = u64::clone_from_slice(&bytes[0..8]) & 0x00_ff_ff_ff_ff_ff_ff_ff;
                Ok(Atom::Reference(offset))
            }
            2 => {
                let length =
                    (u64::clone_from_slice(&bytes[0..8]) & 0x00_ff_ff_ff_ff_ff_ff_ff) as usize;
                let payload_bytes = &bytes[8..(8 + length)];
                // TODO(marcelgarus): Check alignment bytes.
                Ok(Atom::Bytes(payload_bytes.to_vec()))
            }
            _ => Err(()),
        }
    }
}
