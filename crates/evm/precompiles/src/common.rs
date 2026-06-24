//! Common abstractions and shared types for custom EVM precompiles.
//!
//! This module defines the [`CustomPrecompile`] trait, which standardizes
//! how precompiles expose:
//!
//! - A unique identifier (`PrecompileId`)
//! - Deterministic gas calculation
//! - Execution logic with gas-aware semantics
//!
//! It is intended to be implemented by all custom precompiles in this crate.

use alloy_evm::revm::precompile::{EthPrecompileResult, PrecompileId};

/// A custom EVM precompile implementation.
///
/// Implementors define a precompile's identity, gas cost calculation,
/// and execution logic. The execution model follows the standard EVM
/// precompile pattern:
///
/// Typical implementations:
///
/// 1. Determine the gas cost for the supplied input.
/// 2. Validate that sufficient gas is available.
/// 3. Execute the precompile logic.
/// 4. Return either output or an execution error.
///
/// # Gas Semantics
///
/// - [`CustomPrecompile::required_gas`] should return the minimum amount of gas required to execute
///   the precompile for the given input.
/// - [`CustomPrecompile::run`] receives the caller-provided `gas_limit`. Depending on the
///   integration, implementations may be responsible for enforcing gas limits and returning an
///   error when insufficient gas is supplied.
/// - Implementations should return an appropriate error in the [`EthPrecompileResult`] if execution
///   cannot be completed.
///
/// # Identity
///
/// Each precompile must expose a unique [`PrecompileId`] through
/// [`CustomPrecompile::name`], which is used for registration and lookup.
pub trait CustomPrecompile {
    /// Returns the amount of gas required to execute this precompile
    /// for the provided input.
    ///
    /// The returned value should be deterministic and depend only on
    /// the input bytes.
    ///
    /// # Arguments
    ///
    /// * `input` - Raw calldata passed to the precompile.
    fn required_gas(&self, input: &[u8]) -> u64;

    /// Executes the precompile.
    ///
    /// # Arguments
    ///
    /// * `input` - Raw calldata passed to the precompile.
    /// * `gas_limit` - Maximum gas available for execution.
    /// * `reservoir` - Additional gas reservoir available to the precompile implementation. The
    ///   exact semantics are implementation-specific.
    ///
    /// # Returns
    ///
    /// A [`EthPrecompileResult`] containing either the execution output
    /// or an error indicating why execution failed.
    fn run(&self, input: &[u8], gas_limit: u64, reservoir: u64) -> EthPrecompileResult;

    /// Returns the unique identifier for this precompile.
    ///
    /// This identifier is used to register and locate the precompile
    /// within the EVM execution environment.
    fn name(&self) -> PrecompileId;
}
