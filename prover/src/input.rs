use std::collections::HashMap;

use ethabi::{decode, encode, ethereum_types::U256, ParamType, Token};
use num_bigint::{BigInt, Sign};
use serde::{Deserialize, Serialize};
use ark_circom::zkp::Input;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Game2048Input {
    pub board: Vec<Vec<u8>>,
    #[serde(rename = "packedBoard")]
    pub packed_board: Vec<String>,
    #[serde(rename = "packedDir")]
    pub packed_dir: String,
    pub direction: Vec<u8>,
    pub address: String,
    pub nonce: String,
    pub step: u64,
    #[serde(rename = "stepAfter")]
    pub step_after: u64,
}

impl Game2048Input {
    #[allow(dead_code)]
    pub fn to_hex(&self) -> String {
        let mut packed_board = vec![];
        for x in self.packed_board.iter() {
            packed_board.push(Token::Uint(U256::from_dec_str(x).unwrap()))
        }

        let packed_dir = Token::Uint(U256::from_dec_str(&self.packed_dir).unwrap());
        let address = Token::Uint(U256::from_dec_str(&self.address).unwrap());
        let nonce = Token::Uint(U256::from_dec_str(&self.nonce).unwrap());
        let step = Token::Uint(U256::from(self.step));
        let step_after = Token::Uint(U256::from(self.step_after));

        let bytes = encode(&[
            Token::Array(packed_board),
            packed_dir,
            address,
            nonce,
            step,
            step_after,
        ]);
        format!("0x{}", hex::encode(&bytes))
    }
}

const BOARD_STEP: u32 = 32;
const BOARD_LEN: usize = 16;
const DIR_STEP: u32 = 4;
const DIR_LEN: usize = 60;

fn unpack(t: Token, step: u32, len: usize) -> Vec<BigInt> {
    let mut d = t.into_uint().unwrap_or(U256::zero());
    let step = U256::from(step);
    let mut items = vec![];

    loop {
        if d < step {
            items.push(BigInt::from(d.as_u64()));
            break;
        }
        let (next, n) = d.div_mod(step);

        d = next;
        items.push(BigInt::from(n.as_u64()));
    }

    if items.len() < len {
        for _ in items.len()..len {
            items.push(BigInt::from(0));
        }
    }

    items.reverse();
    return items;
}

pub fn decode_prove_input(bytes: &[u8]) -> Result<Input, anyhow::Error> {
    let input_tokens = decode(
        &[
            ParamType::Array(Box::new(ParamType::Uint(256))), // packed_board
            ParamType::Uint(256),                             // packed_dir
            ParamType::Uint(256),                             // address
            ParamType::Uint(256),                             // nonce
            ParamType::Uint(256),                             // step
            ParamType::Uint(256),                             // step_after
        ],
        bytes,
    )?;

    let f_uint = |token: Token| -> BigInt {
        let mut bytes = [0u8; 32];
        token.into_uint().unwrap().to_big_endian(&mut bytes);
        BigInt::from_bytes_be(Sign::Plus, &bytes)
    };

    let mut board = vec![];
    let mut packed_board = vec![];
    for x in input_tokens[0].clone().into_array().unwrap() {
        board.extend(unpack(x.clone(), BOARD_STEP, BOARD_LEN));
        packed_board.push(f_uint(x));
    }

    let packed_token = input_tokens[1].clone();
    let direction = unpack(packed_token.clone(), DIR_STEP, DIR_LEN);
    let packed_dir = f_uint(packed_token);

    let address = f_uint(input_tokens[2].clone());
    let nonce = f_uint(input_tokens[3].clone());
    let step = f_uint(input_tokens[4].clone());
    let step_after = f_uint(input_tokens[5].clone());

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

#[cfg(test)]
mod test {
    use crate::input::decode_prove_input;
    use ethabi::ethereum_types::U256;

    use super::Game2048Input;

    fn pack_board(board: &[u8]) -> U256 {
        let mut packed = U256::zero();
        let step = U256::from(32u32);
        for b in board {
            packed = packed * step + U256::from(*b);
        }
        return packed;
    }

    fn pack_direction(directions: &[u8]) -> U256 {
        let mut packed = U256::zero();
        let step = U256::from(4u32);
        for d in directions {
            packed = packed * step + U256::from(*d);
        }
        return packed;
    }

    fn unpack_direction(directions: &str) -> Vec<u8> {
        let mut d = U256::from_dec_str(directions).unwrap();
        let step = U256::from(4u32);
        let mut items = vec![];

        loop {
            if d < step {
                items.push(d.as_u64() as u8);
                break;
            }
            let (next, n) = d.div_mod(step);

            d = next;
            items.push(n.as_u64() as u8);
        }
        if items.len() < 60 {
            for _ in items.len()..60 {
                items.push(0);
            }
        }
        items.reverse();
        println!("{:?}", items);

        return items;
    }

    #[test]
    fn test_serialize() {
        let input = r##"
        {
            "board": [
                [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 2, 4, 6, 0, 1, 2, 4, 0, 0, 0, 5, 0, 0, 1, 3]
            ],
            "packedBoard": ["35218731827200", "2515675923718842875939"],
            "packedDir": "311800516178808354245949615821275955",
            "direction": [0, 3, 3, 0, 0, 0, 3, 0, 3, 3, 0, 3, 3, 0, 3, 0, 2, 0, 3, 3, 0, 2, 0, 3, 0, 0, 3, 0, 2, 0, 3, 3, 0, 0, 3, 0, 3, 3, 0, 3, 3, 3, 3, 3, 0, 0, 3, 2, 3, 3, 0, 3, 3, 0, 0, 3, 0, 3, 0, 3],
            "address": "6789",
            "step": 0,
            "stepAfter": 60,
            "nonce": "456"
        }
        "##;

        let input: Game2048Input = serde_json::from_str(input).unwrap();
        for b in &input.board {
            println!("{}", pack_board(b));
        }
        println!("{:?}", input.packed_board);
        println!("{}", pack_direction(&input.direction));
        println!("{}", input.packed_dir);

        println!("{:?}", input.direction);
        unpack_direction(&input.packed_dir);

        let hex = input.to_hex();
        println!("{}", hex);

        let input_hex = hex.trim_start_matches("0x");
        let input_bytes = hex::decode(input_hex).expect("Unable to decode input file");
        decode_prove_input(&input_bytes).expect("Unable to decode input");
    }
}
