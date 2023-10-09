use miden_lib::SatKernel;
use miden_objects::{
    accounts::{Account, AccountCode, AccountId},
    assembly::CodeBlock,
    notes::{Note, NoteOrigin, NoteScript},
    transaction::{PreparedTransaction, TransactionResult},
    utils::collections::BTreeMap,
    AccountError, BlockHeader, ChainMmr, Digest, Hasher, TransactionResultError,
};
use vm_core::{Operation, Program};
use vm_processor::{ExecutionError, RecAdviceProvider};

mod compiler;
pub use compiler::{NoteTarget, TransactionComplier};

mod data;
pub use data::DataStore;

mod executor;
pub use executor::TransactionExecutor;

mod prover;
pub use prover::TransactionProver;

mod result;
pub use result::TryFromVmResult;

mod verifier;
pub use verifier::TransactionVerifier;

mod error;
pub use error::{
    DataStoreError, TransactionCompilerError, TransactionError, TransactionExecutorError,
    TransactionProverError, TransactionVerifierError,
};

#[cfg(test)]
mod tests;
