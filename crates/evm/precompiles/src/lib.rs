//! # Custom EVM Precompiles
//!
//! This crate provides infrastructure and implementations for custom
//! EVM precompiles built on top of `alloy_evm` / `revm`.
//!
//! It defines a trait-based abstraction for writing deterministic,
//! gas-metered precompiles, along with concrete implementations such
//! as Poseidon hashing over BN254 field elements.
//!
//! ## Architecture
//!
//! The crate is organized around two main modules:
//!
//! - [`common`]: Core abstractions shared by all precompiles, including the [`CustomPrecompile`]
//!   trait and execution interface.
//! - [`poseidon`]: A Poseidon hash precompile implementation for BN254 scalar field elements.
//!
//! ## Precompile Model
//!
//! Each precompile in this crate typically follows a consistent
//! execution model:
//!
//! 1. Compute deterministic gas usage from input via `required_gas`.
//! 2. Validate input format and constraints.
//! 3. Ensure sufficient gas is available for execution.
//! 4. Execute the cryptographic or computational primitive.
//! 5. Return an [`EthPrecompileResult`] containing output or an error.
//!
//! ## Gas Accounting
//!
//! Gas costs are fully input-dependent and deterministic.
//! Implementations are expected to define both:
//!
//! - A base cost for invocation
//! - A per-word or per-element cost model
//!
//! This ensures consistent execution across all EVM nodes.
//!
//! ## Modules
//!
//! ### `common`
//!
//! Defines the [`CustomPrecompile`] trait, which standardizes:
//!
//! - Precompile identity via [`PrecompileId`]
//! - Gas estimation via `required_gas`
//! - Execution interface via `run`
//!
//! ### `poseidon`
//!
//! Implements a Poseidon hash precompile over BN254 field elements,
//! compatible with Circom-style parameterization and 32-byte big-endian
//! field inputs.
//!
//! The implementation enforces:
//!
//! - 32-byte aligned inputs
//! - Maximum of 12 field elements
//! - Deterministic Poseidon instantiation based on input size
//!
//! ## Error Handling
//!
//! Precompiles return errors through the underlying REVM precompile
//! result types. Errors may be returned for invalid inputs,
//! insufficient gas, or execution failures.
//!
//! [`CustomPrecompile`]: crate::common::CustomPrecompile
//! [`PrecompileId`]: alloy_evm::revm::precompile::PrecompileId
//! [`EthPrecompileResult`]: alloy_evm::revm::precompile::EthPrecompileResult
//! [`PrecompileError`]: alloy_evm::revm::precompile::PrecompileError

pub mod common;
pub mod poseidon;
#[cfg(test)]
mod poseidon_test;
