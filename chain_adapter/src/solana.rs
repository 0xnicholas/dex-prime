use crate::adapter_trait::ChainAdapter;
use crate::tx::{ChainTx, TxMeta, TxType};
use common::types::ExecuteCommand;
use anyhow::Result;

pub struct SolanaAdapter;

impl SolanaAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ChainAdapter for SolanaAdapter {
    fn build_tx(&self, cmd: ExecuteCommand) -> Result<ChainTx> {
        let (tx_type, memo) = match &cmd {
            ExecuteCommand::Open { .. } => (TxType::OpenOrder, Some("Open Order".to_string())),
            ExecuteCommand::Close { .. } => (TxType::CloseOrder, Some("Close Order".to_string())),
            ExecuteCommand::Liquidate { .. } => (TxType::Liquidation, Some("Liquidation".to_string())),
        };
        Ok(ChainTx {
            payload: vec![],   //Replace it with Solana transaction data
            meta: TxMeta {
                chain_id: "solana_mainnet".to_string(),
                nonce: None,
                expiry_slot: None,
                memo,
                client_order_id: None,
            },
            signers: vec!["solana_signer_placeholder".to_string()],
            simulation_only: false,
            tx_type,
        })
    }

    fn submit_tx(&self, tx: ChainTx) -> Result<String> {
        println!("[Solana] Submitting tx: {:?}", tx);
        Ok("mock_tx_hash_solana_123".to_string())
    }
}