use crate::tx::ChainTx;
use common::types::ExecuteCommand;

pub trait ChainAdapter {
    fn build_tx(&self, cmd: ExecuteCommand) -> anyhow::Result<ChainTx>;
    fn submit_tx(&self, tx: ChainTx) -> anyhow::Result<String>; // 返回链上交易哈希
}
