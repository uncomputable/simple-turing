use crate::jet;
use crate::jet::turing::TuringJetName;
use simplicity::bititer::BitIter;
use simplicity::bitwriter::BitWriter;
use simplicity::exec::BitMachine;
use simplicity::jet::{AppError, Application, JetNode};
use simplicity::Error;
use std::io::Write;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Turing;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum TuringError {}

impl std::fmt::Display for TuringError {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {}
    }
}

impl std::error::Error for TuringError {}
impl AppError for TuringError {}

impl std::fmt::Display for TuringJetName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Application for Turing {
    type Environment = ();
    type Error = TuringError;
    type JetName = TuringJetName;

    fn decode_jet<I: Iterator<Item = u8>>(
        iter: &mut BitIter<I>,
    ) -> Result<&'static JetNode<Self>, Error> {
        let code = iter.read_bits_be(7).ok_or(Error::EndOfStream)?;

        match code {
            0 => Ok(&jet::turing::EQ256),
            1 => Ok(&jet::turing::RIGHT_SHIFT_256),
            2 => Ok(&jet::turing::LEFT_SHIFT_256),
            3 => Ok(&jet::turing::COMPLEMENT_256),
            4 => Ok(&jet::turing::AND_256),
            5 => Ok(&jet::turing::OR_256),
            6 => Ok(&jet::turing::IS_ZERO256),
            _ => Err(Error::ParseError("Illegal jet encoding")),
        }
    }

    fn encode_jet<W: Write>(jet: &JetNode<Self>, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        match jet.name {
            TuringJetName::Eq256 => w.write_bits_be(128, 8),
            TuringJetName::RightShift256 => w.write_bits_be(128 + 1, 8),
            TuringJetName::LeftShift256 => w.write_bits_be(128 + 2, 8),
            TuringJetName::Complement256 => w.write_bits_be(128 + 3, 8),
            TuringJetName::And256 => w.write_bits_be(128 + 4, 8),
            TuringJetName::Or256 => w.write_bits_be(128 + 5, 8),
            TuringJetName::IsZero256 => w.write_bits_be(128 + 6, 8),
        }
    }

    fn exec_jet(
        jet: &JetNode<Self>,
        mac: &mut BitMachine,
        _env: &Self::Environment,
    ) -> Result<(), Self::Error> {
        match jet.name {
            TuringJetName::Eq256 => {
                let a = mac.read_32bytes();
                let b = mac.read_32bytes();
                mac.write_bit(a.eq(&b));
            }
            TuringJetName::RightShift256 => {
                let input = mac.read_32bytes();
                let shifted = right_shift_256(&input);
                mac.write_bytes(&shifted);
            }
            TuringJetName::LeftShift256 => {
                let input = mac.read_32bytes();
                let shifted = left_shift_256(&input);
                mac.write_bytes(&shifted);
            }
            TuringJetName::Complement256 => {
                let a = mac.read_32bytes();
                let mut a_complemented = Vec::with_capacity(32);

                for byte in a {
                    a_complemented.push(!byte);
                }

                mac.write_bytes(&a_complemented);
            }
            TuringJetName::And256 => {
                let a = mac.read_32bytes();
                let b = mac.read_32bytes();
                let mut a_and_b = Vec::with_capacity(32);

                for i in 0..32 {
                    a_and_b.push(a[i] & b[i]);
                }

                mac.write_bytes(&a_and_b);
            }
            TuringJetName::Or256 => {
                let a = mac.read_32bytes();
                let b = mac.read_32bytes();
                let mut a_and_b = Vec::with_capacity(32);

                for i in 0..32 {
                    a_and_b.push(a[i] | b[i]);
                }

                mac.write_bytes(&a_and_b);
            }
            TuringJetName::IsZero256 => {
                let a = mac.read_32bytes();
                mac.write_bit(a.eq(&[0; 32]));
            }
        }

        Ok(())
    }
}

fn right_shift_256(input: &[u8; 32]) -> [u8; 32] {
    let mut shifted = [0; 32];

    for (byte_index, byte) in input.iter().enumerate() {
        let bit_index = match one_hot_get_index(*byte) {
            None => continue,
            Some(x) => x,
        };

        if bit_index < 7 {
            shifted[byte_index] = one_hot_of(bit_index + 1);
        } else {
            // Panics if last bit in last byte is shifted right
            shifted[byte_index + 1] = one_hot_of(0);
        }

        break;
    }

    shifted
}

fn left_shift_256(input: &[u8; 32]) -> [u8; 32] {
    let mut shifted = [0; 32];

    for (byte_index, byte) in input.iter().enumerate() {
        let bit_index = match one_hot_get_index(*byte) {
            None => continue,
            Some(x) => x,
        };

        if bit_index > 0 {
            shifted[byte_index] = one_hot_of(bit_index - 1);
        } else {
            // Panics if first bit in first byte is shifted left
            shifted[byte_index - 1] = one_hot_of(7);
        }

        break;
    }

    shifted
}

fn one_hot_get_index(byte: u8) -> Option<usize> {
    match byte {
        0b10000000 => Some(0),
        0b01000000 => Some(1),
        0b00100000 => Some(2),
        0b00010000 => Some(3),
        0b00001000 => Some(4),
        0b00000100 => Some(5),
        0b00000010 => Some(6),
        0b00000001 => Some(7),
        _ => None,
    }
}

fn one_hot_of(index: usize) -> u8 {
    match index {
        0 => 0b10000000,
        1 => 0b01000000,
        2 => 0b00100000,
        3 => 0b00010000,
        4 => 0b00001000,
        5 => 0b00000100,
        6 => 0b00000010,
        7 => 0b00000001,
        _ => panic!("Bad index: {}", index),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn right_shift_256() {
        let base = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let input_output_carry = vec![
            (0b10000000, 0b01000000, false),
            (0b01000000, 0b00100000, false),
            (0b00100000, 0b00010000, false),
            (0b00010000, 0b00001000, false),
            (0b00001000, 0b00000100, false),
            (0b00000100, 0b00000010, false),
            (0b00000010, 0b00000001, false),
            (0b00000001, 0b10000000, true),
        ];

        for (input_byte, output_byte, carry) in input_output_carry {
            let mut input = base.clone();
            input[0] = input_byte;
            let mut expected = base.clone();
            if carry {
                expected[1] = output_byte
            } else {
                expected[0] = output_byte
            };

            let output = super::right_shift_256(&input);
            assert_eq!(expected, output);
        }
    }

    #[test]
    fn left_shift_256() {
        let base = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let input_output_carry = vec![
            (0b00000001, 0b00000010, false),
            (0b00000010, 0b00000100, false),
            (0b00000100, 0b00001000, false),
            (0b00001000, 0b00010000, false),
            (0b00010000, 0b00100000, false),
            (0b00100000, 0b01000000, false),
            (0b01000000, 0b10000000, false),
            (0b10000000, 0b00000001, true),
        ];

        for (input_byte, output_byte, carry) in input_output_carry {
            let mut input = base.clone();
            input[1] = input_byte;
            let mut expected = base.clone();
            if carry {
                expected[0] = output_byte
            } else {
                expected[1] = output_byte
            };

            let output = super::left_shift_256(&input);
            assert_eq!(expected, output);
        }
    }
}
