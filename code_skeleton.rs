// engine/mod.rs
pub mod orderbook;
pub mod matching;
pub mod risk;
pub mod types;

// engine/types.rs
#[derive(Debug, Clone)]
pub enum Side {
    Long,
    Short,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub market: String,
    pub user: String,
    pub side: Side,
    pub price: f64,
    pub size: f64,
    pub leverage: u8,
    pub ts: u64,
}

// chain_adapter/mod.rs
pub mod types;
pub mod executor;
pub mod solana_executor;

// chain_adapter/types.rs
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ChainTxAction {
    OpenPosition,
    ClosePosition,
    Liquidate,
    AddMargin,
    RemoveMargin,
}

#[derive(Debug, Clone)]
pub struct ChainTx {
    pub action: ChainTxAction,
    pub market: String,
    pub user: String,
    pub params: HashMap<String, String>,
    pub nonce: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ChainTxResult {
    pub success: bool,
    pub tx_hash: Option<String>,
    pub block_height: Option<u64>,
    pub gas_used: Option<u64>,
    pub error_message: Option<String>,
}

pub trait ChainExecutor {
    fn execute(&self, tx: ChainTx) -> anyhow::Result<ChainTxResult>;
}

// settlement/mod.rs
pub mod liquidation;
pub mod funding;
pub mod pnl;

// api/mod.rs
pub mod http;
pub mod ws;
pub mod auth;

// infra/mod.rs
pub mod db;
pub mod kv;
pub mod pubsub;
pub mod config;

// indexer/mod.rs
pub mod solana_indexer;
pub mod monoli_indexer;

// onchain/Anchor.toml
# [placeholder file for Anchor project manifest]

// onchain/programs/monoli_dex/src/lib.rs
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

#[program]
pub mod monoli_dex {
    use super::*;
}

// onchain/programs/monoli_dex/src/state/mod.rs
pub mod vault;
pub mod position;
pub mod market;

// onchain/programs/monoli_dex/src/instructions/mod.rs
pub mod open_position;
pub mod close_position;
pub mod liquidate;
pub mod add_margin;
