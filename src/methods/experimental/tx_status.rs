use super::*;

pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
use near_jsonrpc_primitives::types::transactions::SignedTransaction;
pub use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_primitives::views::TxExecutionStatus;

pub type RpcTransactionStatusResponse =
    near_primitives::views::FinalExecutionOutcomeWithReceiptView;

#[derive(Debug)]
pub struct RpcTransactionStatusRequest {
    pub transaction_info: TransactionInfo,
}

impl From<RpcTransactionStatusRequest>
    for near_jsonrpc_primitives::types::transactions::RpcTransactionStatusRequest
{
    fn from(this: RpcTransactionStatusRequest) -> Self {
        Self {
            transaction_info: this.transaction_info,
            wait_until: TxExecutionStatus::default(),
        }
    }
}

impl RpcHandlerResponse for RpcTransactionStatusResponse {}

impl RpcMethod for RpcTransactionStatusRequest {
    type Response = RpcTransactionStatusResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_tx_status"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(match &self.transaction_info {
            TransactionInfo::Transaction(SignedTransaction::SignedTransaction(
                signed_transaction,
            )) => {
                json!([common::serialize_signed_transaction(signed_transaction)?])
            }
            TransactionInfo::TransactionId {
                tx_hash,
                sender_account_id,
            } => {
                json!([tx_hash, sender_account_id])
            }
        })
    }
}

impl private::Sealed for RpcTransactionStatusRequest {}
