//! Poseidon hash precompile for BN254 field elements.
//!
//! This module implements a custom EVM precompile that computes a
//! Poseidon hash over a sequence of BN254 scalar field elements.
//!
//! ## Input Format
//!
//! Input is interpreted as 32-byte big-endian chunks representing field elements.
//!
//! ## Constraints
//!
//! - Input length must be divisible by 32 bytes
//! - Maximum of 12 field elements
//!
//! ## Gas Model
//!
//! - Base cost: 600 gas
//! - Per element: 5400 gas
//!
//! The cost is deterministic and depends only on input length.

use alloy_evm::revm::precompile::{
    EthPrecompileOutput, EthPrecompileResult, PrecompileHalt, PrecompileId,
};
use ark_bn254::Fr;
use light_poseidon::{Poseidon, PoseidonBytesHasher};

use crate::common::CustomPrecompile;

const FIELD_SIZE: usize = 32;
const MAX_PARAMS: usize = 12;
const BASE_GAS: u64 = 600;
const PER_WORD_GAS: u64 = 5400;

/// Poseidon hash precompile.
///
/// Computes a Poseidon hash over a sequence of BN254 field elements.
///
/// # Example
///```
/// use reth_evm_precompiles::{common::CustomPrecompile, poseidon::PoseidonPrecompile};
///
/// let precompile = PoseidonPrecompile;
///
/// let input = vec![1u8; 32 * 2]; // 2 field elements
/// let gas_limit = 1_000_000;
///
/// let result = precompile.run(&input, gas_limit, 0);
/// assert!(result.is_ok());
/// ```
///
/// # Input Format
///
/// The input is interpreted as a concatenation of 32-byte big-endian
/// field elements. Each 32-byte chunk represents a single BN254 scalar.
///
/// ```text
/// | field_0 (32 bytes) | field_1 (32 bytes) | ... | field_n (32 bytes) |
/// ```
///
/// # Constraints
///
/// - Input length must be a multiple of 32 bytes.
/// - At most 12 field elements may be provided.
/// - The Poseidon parameterization is selected based on the number of supplied field elements.
///
/// # Gas Cost
///
/// Gas is calculated as:
///
/// ```text
/// BASE_GAS + (number_of_fields × PER_WORD_GAS)
/// ```
///
/// where:
///
/// - `BASE_GAS = 600`
/// - `PER_WORD_GAS = 5400`
///
/// # Output
///
/// Returns the Poseidon hash as a 32-byte value.
///
/// # Errors
///
/// Returns a precompile halt (`PrecompileHalt`) in the following cases:
///
/// - The supplied gas limit is insufficient.
/// - The input length is not divisible by 32.
/// - More than 12 field elements are supplied.
/// - Poseidon initialization or hashing fails.
#[derive(Debug)]
pub struct PoseidonPrecompile;

impl CustomPrecompile for PoseidonPrecompile {
    /// See [`CustomPrecompile::name`].
    fn name(&self) -> PrecompileId {
        PrecompileId::custom("poseidon")
    }

    /// See [`CustomPrecompile::required_gas`].
    fn required_gas(&self, input: &[u8]) -> u64 {
        (input.len() / FIELD_SIZE) as u64 * PER_WORD_GAS + BASE_GAS
    }

    /// See [`CustomPrecompile::run`].
    fn run(&self, input: &[u8], gas_limit: u64, _reservoir: u64) -> EthPrecompileResult {
        if input.is_empty() || !input.len().is_multiple_of(FIELD_SIZE) {
            return Err(PrecompileHalt::other_static("Invalid length"));
        }

        let number_params = input.len() / FIELD_SIZE;

        if number_params > MAX_PARAMS {
            return Err(PrecompileHalt::other_static("Too many params"));
        }

        let gas = self.required_gas(input);

        if gas > gas_limit {
            return Err(PrecompileHalt::OutOfGas);
        }

        let mut poseidon = Poseidon::<Fr>::new_circom(number_params)
            .map_err(|err| PrecompileHalt::other(err.to_string()))?;

        let chunks: Vec<&[u8]> = input.chunks(FIELD_SIZE).collect();

        let hash = poseidon
            .hash_bytes_be(&chunks)
            .map_err(|err| PrecompileHalt::other(err.to_string()))?;

        Ok(EthPrecompileOutput::new(gas, hash.into()))
    }
}
