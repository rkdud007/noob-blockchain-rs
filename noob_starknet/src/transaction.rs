use starknet::core::types::{ContractClass, FieldElement};

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    /// Transaction executed unsuccessfully and thus was skipped.
    Rejected,
    /// When the transaction pass validation but encountered error during execution.
    Reverted,
    /// Transactions that have been included in the L2 block which have
    /// passed both validation and execution.
    AcceptedOnL2,
    /// When the block of which the transaction is included in have been committed to the
    /// L1 settlement layer.
    AcceptedOnL1,
}

#[derive(Debug, Clone)]
pub enum Transaction {
    Invoke(InvokeTransaction),
    Declare(DeclareTransaction),
}

#[derive(Debug, Clone)]
pub struct InvokeTransaction {
    pub status: TransactionStatus,

    pub sender: FieldElement,
    pub call_data: Vec<FieldElement>,
    pub version: FieldElement,
    pub nonce: FieldElement,
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
}

#[derive(Debug, Clone)]
pub struct DeclareTransaction {
    pub status: TransactionStatus,

    pub chain_id: FieldElement,
    pub contract_class: ContractClass,
    pub compiled_class_hash: FieldElement,
    pub sender_address: FieldElement,
    pub signature: Vec<FieldElement>,
    pub max_fee: FieldElement,
    /// version is 2
    pub version: FieldElement,
    pub nonce: FieldElement,
}
