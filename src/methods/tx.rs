//! Queries the status of a transaction.
//!
//! ## Example
//! Returns the final transaction result for
//! <https://explorer.near.org/transactions/B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq>
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//! let tx_hash = "B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq".parse()?;
//!
//! let request = methods::tx::RpcTransactionStatusRequest {
//!     transaction_info: methods::tx::TransactionInfo::TransactionId {
//!         hash: tx_hash,
//!         account_id: "itranscend.near".parse()?,
//!    }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert_eq!(tx_hash, response.transaction.hash);
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
use near_jsonrpc_primitives::types::transactions::SignedTransaction;
pub use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_primitives::views::TxExecutionStatus;

pub type RpcTransactionStatusResponse = near_primitives::views::FinalExecutionOutcomeView;

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

impl RpcMethod for RpcTransactionStatusRequest {
    type Response = RpcTransactionStatusResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "tx"
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
