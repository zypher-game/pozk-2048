mod input;

use ark_circom::zkp::{
    init_bn254_circom_from_bytes, init_bn254_params_from_bytes, multiple_proofs_to_abi_bytes,
    prove_bn254, verify_bn254,
};
use input::decode_prove_inputs;
use std::fs::{read_to_string, write};

const WASM_BYTES: &[u8] = include_bytes!("../materials/game2048_60.wasm");
const R1CS_BYTES: &[u8] = include_bytes!("../materials/game2048_60.r1cs");
const ZKEY_BYTES: &[u8] = include_bytes!("../materials/game2048_60.zkey");

// const WASM_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.wasm");
// const R1CS_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.r1cs");
// const ZKEY_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.zkey");

/// INPUT=test_input OUTPUT=test_output PROOF=test_proof cargo run --release
#[tokio::main]
async fn main() {
    let input_path = std::env::var("INPUT").expect("env INPUT missing");
    let output_path = std::env::var("OUTPUT").expect("env OUTPUT missing");
    let proof_path = std::env::var("PROOF").expect("env PROOF missing");

    let input_hex = read_to_string(input_path).expect("Unable to read input file");
    let input_bytes =
        hex::decode(input_hex.trim_start_matches("0x")).expect("Unable to decode input file");
    let inputs = decode_prove_inputs(&input_bytes).expect("Unable to decode inputs");

    let mut pis = vec![];
    let mut proofs = vec![];
    let params = init_bn254_params_from_bytes(ZKEY_BYTES, false).unwrap();
    for input in inputs {
        let circom = init_bn254_circom_from_bytes(WASM_BYTES, R1CS_BYTES).unwrap();
        let (pi, proof) = prove_bn254(&params, circom, input).unwrap();
        assert!(verify_bn254(&params.vk, &pi, &proof).unwrap());
        pis.push(pi);
        proofs.push(proof);
    }
    let (pi_bytes, proof_bytes) = multiple_proofs_to_abi_bytes(&pis, &proofs).unwrap();

    let pi_hex = format!("0x{}", hex::encode(pi_bytes));
    write(output_path, pi_hex).expect("Unable to create output file");

    let proof_hex = format!("0x{}", hex::encode(proof_bytes));
    write(proof_path, proof_hex).expect("Unable to create proof file");
}
