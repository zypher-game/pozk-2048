use std::{collections::HashMap, str::FromStr};

use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use zypher_circom_compat::Input;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Game2048Input {
    pub board: Vec<Vec<u8>>,
    pub packed_board: Vec<String>,
    pub packed_dir: String,
    pub direction: Vec<u8>,
    pub address: String,
    pub nonce: String,
    pub step: u8,
    pub step_after: u8,
}

impl Game2048Input {
    #[allow(dead_code)]
    pub fn to_hex(&self) -> String {
        let bytes = bincode::serialize(self).unwrap();
        hex::encode(&bytes)
    }

    pub fn from_hex(hex: String) -> Result<Self, anyhow::Error> {
        let bytes = hex::decode(hex)?;
        let input = bincode::deserialize(&bytes)?;
        Ok(input)
    }
}

impl TryInto<Input> for Game2048Input {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Input, Self::Error> {
        let mut board = vec![];
        for x in self.board.iter().flatten() {
            board.push(BigInt::from(*x))
        }

        let mut packed_board = vec![];
        for x in self.packed_board.iter() {
            packed_board.push(BigInt::from_str(x)?)
        }

        let mut direction = vec![];
        for x in self.direction {
            direction.push(BigInt::from(x))
        }

        let packed_dir = BigInt::from_str(&self.packed_dir)?;
        let address = BigInt::from_str(&self.address)?;
        let nonce = BigInt::from_str(&self.nonce)?;
        let step = BigInt::from(self.step);
        let step_after = BigInt::from(self.step_after);

        let mut maps = HashMap::new();
        maps.insert("board".to_string(), board);
        maps.insert("packedBoard".to_string(), packed_board);
        maps.insert("packedDir".to_string(), vec![packed_dir]);
        maps.insert("direction".to_string(), direction);
        maps.insert("address".to_string(), vec![address]);
        maps.insert("step".to_string(), vec![step]);
        maps.insert("stepAfter".to_string(), vec![step_after]);
        maps.insert("nonce".to_string(), vec![nonce]);

        Ok(Input { maps })
    }
}

#[cfg(test)]
mod test {
    use super::Game2048Input;

    #[test]
    fn test_serialize() {
        let input = r##"
        {
            "board": [
                [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 2, 4, 6, 0, 1, 2, 4, 0, 0, 0, 5, 0, 0, 1, 3]
            ],
            "packed_board": ["35218731827200", "2515675923718842875939"],
            "packed_dir": "311800516178808354245949615821275955",
            "direction": [0, 3, 3, 0, 0, 0, 3, 0, 3, 3, 0, 3, 3, 0, 3, 0, 2, 0, 3, 3, 0, 2, 0, 3, 0, 0, 3, 0, 2, 0, 3, 3, 0, 0, 3, 0, 3, 3, 0, 3, 3, 3, 3, 3, 0, 0, 3, 2, 3, 3, 0, 3, 3, 0, 0, 3, 0, 3, 0, 3],
            "address": "6789",
            "step": 0,
            "step_after": 60,
            "nonce": "456"
        }
        "##;

        let input: Game2048Input = serde_json::from_str(input).unwrap();
        let hex = input.to_hex();
        println!("{}", hex);
        let expect = Game2048Input::from_hex(hex).unwrap();
        assert_eq!(input, expect)
    }
}
