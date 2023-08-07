use blockifier::{
    block_context::BlockContext,
    state::{cached_state::CachedState, state_api::StateReader},
    transaction::{
        errors::TransactionExecutionError, objects::TransactionExecutionInfo,
        transaction_execution::Transaction as ExecutionTransaction,
        transactions::ExecutableTransaction,
    },
};
use tracing::warn;

pub fn execute_transaction<S: StateReader>(
    transaction: ExecutionTransaction,
    pending_state: &mut CachedState<S>,
    block_context: &BlockContext,
    charge_fee: bool,
    validate: bool,
) -> Result<TransactionExecutionInfo, TransactionExecutionError> {
    let res = match transaction {
        ExecutionTransaction::AccountTransaction(tx) => {
            tx.execute(pending_state, block_context, charge_fee, validate)
        }
        ExecutionTransaction::L1HandlerTransaction(tx) => {
            tx.execute(pending_state, block_context, charge_fee, validate)
        }
    };

    match res {
        Ok(exec_info) => {
            if let Some(err) = &exec_info.revert_error {
                let formatted_err = format!("{:?}", err).replace("\\n", "\n");
                warn!("Transaction execution error: {formatted_err}");
            }
            Ok(exec_info)
        }
        Err(err) => {
            warn!("Transaction validation error: {err:?}");
            Err(err)
        }
    }
}
