use chain_adapter::adapter_trait::ChainAdapter;
use chain_adapter::tx::ChainTx;
use common::types::ExecutionRequest;
use anyhow::Result;

pub struct Executor<A: ChainAdapter> {
    pub adapter: A,
}

impl<A: ChainAdapter> Executor<A> {
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }
    
    pub fn execute(&self, req: ExecutionRequest) -> Result<String> {
        let mut tx: ChainTx = self.adapter.build_tx(req.command.clone())?;
        tx.simulation_only = req.dry_run;
        
        if req.dry_run {
            println!("Simulating tx: {:?}", tx);
            Ok("simulated".to_string())
        } else {
            let tx_hash = self.adapter.submit_tx(tx)?;
            println!("Submitting tx: {:?}", tx_hash);
            Ok(tx_hash)
        }
    }
}



/*
use chain_adapter::solana::SolanaAdapter;

let chain = SolanaAdapter::new("https://api.mainnet-beta.solana.com");
let tx = chain.build_tx(cmd)?;
chain.submit_tx(tx)?;

*/