mod input;

use input::decode_prove_input;
use std::fs::{read_to_string, write};
use zypher_circom_compat::bls12_381::{init_from_bytes, prove, verify, proofs_to_abi_bytes};

// const WASM_BYTES: &[u8] = include_bytes!("../materials/game2048_60.wasm");
// const R1CS_BYTES: &[u8] = include_bytes!("../materials/game2048_60.r1cs");
// const ZKEY_BYTES: &[u8] = include_bytes!("../materials/game2048_60.zkey");

const WASM_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.wasm");
const R1CS_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.r1cs");
const ZKEY_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.zkey");

/// INPUT=test_input OUTPUT=test_output PROOF=test_proof cargo run --release
fn main() {
    let input_path = std::env::var("INPUT").expect("env INPUT missing");
    let output_path = std::env::var("OUTPUT").expect("env OUTPUT missing");
    let proof_path = std::env::var("PROOF").expect("env PROOF missing");

    let input_hex = read_to_string(input_path).expect("Unable to read input file");
    let input_bytes =
        hex::decode(input_hex.trim_start_matches("0x")).expect("Unable to decode input file");
    let input = decode_prove_input(&input_bytes).expect("Unable to decode input");

    init_from_bytes(WASM_BYTES, R1CS_BYTES, ZKEY_BYTES).unwrap();
    let (pi, proof) = prove(input).unwrap();
    assert!(verify(&pi, &proof).unwrap());
    let (pi_bytes, proof_bytes) = proofs_to_abi_bytes(&pi, &proof).unwrap();

    let pi_hex = format!("0x{}", hex::encode(pi_bytes));
    write(output_path, pi_hex).expect("Unable to create output file");

    let proof_hex = format!("0x{}", hex::encode(proof_bytes));
    write(proof_path, proof_hex).expect("Unable to create proof file");
}
