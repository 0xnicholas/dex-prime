#[derive(Debug, Clone)]
pub struct ChainTx {
    pub payload: Vec<u8>,
    pub meta: TxMeta,
    pub signers: Vec<String>,
    pub simulation_only: bool,
    pub tx_type: TxType,
}

#[derive(Debug, Clone)]
pub struct TxMeta {
    pub chain_id: String,
    pub nonce: Option<u64>,
    pub expiry_slot: Option<u64>,
    pub memo: Option<String>,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TxType {
    OpenOrder,
    CloseOrder,
    Liquidation,
    Transfer,
    FundingUpdate,
}