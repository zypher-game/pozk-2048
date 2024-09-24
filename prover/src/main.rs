mod input;

use ark_circom::zkp::{
    init_bn254_circom_from_bytes, init_bn254_params_from_bytes, multiple_proofs_to_abi_bytes,
    prove_bn254, verify_bn254,
};
use input::decode_prove_inputs;

const WASM_BYTES: &[u8] = include_bytes!("../../../materials/game2048_60.wasm");
const R1CS_BYTES: &[u8] = include_bytes!("../../../materials/game2048_60.r1cs");
const ZKEY_BYTES: &[u8] = include_bytes!("../../../materials/game2048_60.zkey");

// const WASM_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.wasm");
// const R1CS_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.r1cs");
// const ZKEY_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.zkey");

/// INPUT=test_input OUTPUT=test_output PROOF=test_proof cargo run --release
#[tokio::main]
async fn main() {
    let input_path = std::env::var("INPUT").expect("env INPUT missing");
    let input_bytes = reqwest::get(&input_path).await.unwrap().bytes().await.unwrap();
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

    let mut bytes = vec![];
    bytes.extend((pi_bytes.len() as u32).to_be_bytes());
    bytes.extend(pi_bytes);
    bytes.extend(proof_bytes);

    let client = reqwest::Client::new();
    client.post(&input_path).body(bytes).send().await.unwrap();
}
