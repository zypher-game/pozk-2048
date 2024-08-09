mod input;

use ark_ec::AffineRepr;
use ark_ff::{BigInteger, PrimeField};
use ethabi::{encode, ethereum_types::U256, Token};
use input::decode_prove_input;
use std::fs::{read_to_string, write};
use zypher_circom_compat::{init_from_bytes, prove};

const WASM_BYTES: &[u8] = include_bytes!("../materials/game2048_60.wasm");
const R1CS_BYTES: &[u8] = include_bytes!("../materials/game2048_60.r1cs");
const ZKEY_BYTES: &[u8] = include_bytes!("../materials/game2048_60.zkey");

fn parse_filed_to_token<F: PrimeField>(f: &F) -> Token {
    let bytes = f.into_bigint().to_bytes_be();
    Token::Uint(U256::from_big_endian(&bytes))
}

/*
export INPUT="./materials/input.bin"
export OUTPUT="./materials/output.bin"
export PROOF="./materials/proof.bin"
export WASM="./materials/game2048_60.wasm"
export R1CS="./materials/game2048_60.r1cs"
export ZKEY="./materials/game2048_60.zkey"

cargo run --release
*/
fn main() {
    let input_path = std::env::var("INPUT").expect("env INPUT missing");
    let output_path = std::env::var("OUTPUT").expect("env OUTPUT missing");
    let proof_path = std::env::var("PROOF").expect("env PROOF missing");

    let input_hex = read_to_string(input_path).expect("Unable to read input file");
    let input_bytes =
        hex::decode(input_hex.trim_start_matches("0x")).expect("Unable to decode input file");
    let input = decode_prove_input(&input_bytes).expect("Unable to decode input");

    init_from_bytes(WASM_BYTES, R1CS_BYTES, ZKEY_BYTES);
    let (pi, proof) = prove(input).unwrap();

    let mut pi_token = vec![];
    for x in pi.iter() {
        pi_token.push(parse_filed_to_token(x));
    }

    let mut proof_token = vec![];
    let (ax, ay) = proof.a.xy().unwrap();
    proof_token.push(parse_filed_to_token(ax));
    proof_token.push(parse_filed_to_token(ay));

    let (ax, ay) = proof.b.xy().unwrap();
    proof_token.push(parse_filed_to_token(&ax.c0));
    proof_token.push(parse_filed_to_token(&ax.c1));
    proof_token.push(parse_filed_to_token(&ay.c0));
    proof_token.push(parse_filed_to_token(&ay.c1));

    let (cx, cy) = proof.c.xy().unwrap();
    proof_token.push(parse_filed_to_token(cx));
    proof_token.push(parse_filed_to_token(cy));

    let pi_bytes = encode(&pi_token);
    let pi_hex = format!("0x{}", hex::encode(pi_bytes));
    write(output_path, pi_hex).expect("Unable to create output file");

    let proof_bytes = encode(&proof_token);
    let proof_hex = format!("0x{}", hex::encode(proof_bytes));
    write(proof_path, proof_hex).expect("Unable to create proof file");
}
