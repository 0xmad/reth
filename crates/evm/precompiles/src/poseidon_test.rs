use alloy_evm::revm::precompile::PrecompileHalt;

use crate::{common::CustomPrecompile, poseidon::PoseidonPrecompile};

const GAS_USED: [u64; 12] =
    [6000, 11400, 16800, 22200, 27600, 33000, 38400, 43800, 49200, 54600, 60000, 65400];

const EXPECTED_HASHES: [&str; 12] = [
    "0x2a09a9fd93c590c26b91effbb2499f07e8f7aa12e2b4940a3aed2411cb65e11c",
    "0x2098f5fb9e239eab3ceac3f27b81e481dc3124d55ffed523a839ee8446b64864",
    "0x0bc188d27dcceadc1dcfb6af0a7af08fe2864eecec96c5ae7cee6db31ba599aa",
    "0x0532fd436e19c70e51209694d9c215250937921b8b79060488c1206db73e9946",
    "0x2066be41bebe6caf7e079360abe14fbf9118c62eabc42e2fe75e342b160a95bc",
    "0x1fdb1d1757a3a3502bec7084abc047ae86a4f442b8a073d5b3482bb02eb353d5",
    "0x0a47ead74da5372e7d2598e4f93c389bf03e8330219f8bf1e49b362f73491a26",
    "0x035ebc384d320413c9b97d446bf7de69e04d6278d68d52934a4f5f653348622a",
    "0x01c4da168cbfb5014e1dc256d82ba808033c11cc3bd113ef0a44ad86b2075728",
    "0x121abf316742b318e84638b1fd477962b2bb4b352a5abdcf8a4850cc5e863a4f",
    "0x23376b08cad4f9a7c9c0cfeb9c8a1c1b9aa6de067dcffda16198f3180d6d4d7f",
    "0x14b1efe6a1d69ba28d677d97f02e5063aa47e82e9b139396eb39dafa33f48453",
];

#[test]
fn test_required_gas() {
    let precompile = PoseidonPrecompile;

    for index in 0..GAS_USED.len() {
        let input = vec![0u8; (index + 1) * 32];
        let actual = precompile.required_gas(&input);
        assert_eq!(actual, GAS_USED[index]);
    }
}

#[test]
fn test_name() {
    let precompile = PoseidonPrecompile;

    let name = precompile.name();

    assert_eq!(name.to_string(), "poseidon");
}

#[test]
fn test_run() {
    let precompile = PoseidonPrecompile;

    for index in 0..GAS_USED.len() {
        let input = vec![0u8; (index + 1) * 32];
        let output = precompile.run(&input, GAS_USED[index], 0).unwrap();

        assert_eq!(output.gas_used, GAS_USED[index]);
        assert_eq!(output.bytes.to_string(), EXPECTED_HASHES[index]);
    }
}

#[test]
fn test_run_not_enough_gas() {
    let precompile = PoseidonPrecompile;

    let err = precompile.run(&[0u8; 32], 0, 0).unwrap_err();

    assert_eq!(err, PrecompileHalt::OutOfGas);
}

#[test]
fn test_run_invalid_length() {
    let precompile = PoseidonPrecompile;

    let err = precompile.run(&[0], GAS_USED[0], 0).unwrap_err();

    assert_eq!(err, PrecompileHalt::other_static("Invalid length"));
}

#[test]
fn test_run_max_invalid_length() {
    let input = vec![0u8; 13 * 32];
    let precompile = PoseidonPrecompile;

    let err = precompile.run(&input, GAS_USED[11] * 2, 0).unwrap_err();

    assert_eq!(err, PrecompileHalt::other_static("Too many params"));
}

#[test]
fn test_run_max_field_value() {
    let input = vec![
        0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29, 0xb8, 0x50, 0x45, 0xb6, 0x81, 0x81, 0x58,
        0x5d, 0x97, 0x81, 0x6a, 0x91, 0x68, 0x71, 0xca, 0x8d, 0x3c, 0x20, 0x8c, 0x16, 0xd8, 0x7c,
        0xfd, 0x47,
    ];
    let precompile = PoseidonPrecompile;

    let err = precompile.run(&input, GAS_USED[11] * 2, 0).unwrap_err();
    assert_eq!(
        err,
        PrecompileHalt::other_static("Input is larger than the modulus of the prime field.")
    );
}

#[test]
fn test_run_deterministic() {
    let precompile = PoseidonPrecompile;

    let input = vec![42u8; 3 * 32];

    let gas = precompile.required_gas(&input);

    let result1 = precompile.run(&input, gas, 0).unwrap();
    let result2 = precompile.run(&input, gas, 0).unwrap();

    assert_eq!(result1.bytes.to_string(), result2.bytes.to_string());
}

#[test]
fn test_run_different_inputs_produce_different_hashes() {
    let precompile = PoseidonPrecompile;

    let input1 = vec![1u8; 32];
    let input2 = vec![2u8; 32];

    let gas1 = precompile.required_gas(&input1);
    let gas2 = precompile.required_gas(&input2);

    let result1 = precompile.run(&input1, gas1, 0).unwrap();
    let result2 = precompile.run(&input2, gas2, 0).unwrap();

    assert_ne!(result1.bytes.to_string(), result2.bytes.to_string());
}
