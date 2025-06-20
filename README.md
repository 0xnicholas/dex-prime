# dex-prime
Perp DEX using CLOB, based on Solana/Monoli L1.

**codebase**
```
dex-prime/
├── engine/                   # 撮合引擎（无状态）
│   ├── types.rs             # Order, MatchResult, Side
│   ├── orderbook.rs         # BTreeMap + PriceLevel queue
│   ├── matcher.rs           # 核心匹配算法（price-time）
│   ├── matching.rs          # 撮合接口层（submit_order）
│   ├── execution.rs         # 成交执行器（生成事件）
│   └── event.rs             # 成交事件定义

├── indexer/                 # 交易索引/状态同步

├── settlement/              # 持仓、清算、资金管理
│   ├── position.rs          # Position 状态、PNL、apply_trade
│   ├── margin.rs          
│   ├── liquidation.rs       # 清算检查与触发（多模式）
│   └── funding.rs           # Funding rate 定期更新逻辑

├── executor/              # 提交执行链上交易
│   ├── executor.rs         

├── chain_adapter/           # 解耦底层链的适配器层
│   ├── adapter_trait.rs          # 抽象链交互
│   ├── solana.rs            
│   └── monoli.rs      

├── storage/                   # 持久化存储

├── gateway/                 # 用户入口层（API / websocket / gRPC）
│   └── api.rs               # HTTP or WS 处理逻辑（非必须模块）

└── app.rs                   # 主进程，连接 matcher + settlement + chain
```

```
└── onchain/                    # Anchor 程序目录（Solana链上逻辑）
    ├── programs/monoli_dex/
    │   └── src/
    │       ├── lib.rs
    │       ├── instructions/    # open_position, close, settle 等
    │       └── state/           # vault, position, market, funding 等
    └── Anchor.toml
```

**Docs**
- [Monoli Dex Dev.](https://nicholas.feishu.cn/wiki/CvqYwM7eliHImLkSGmPcSPByn6d)
- [Monoli](https://github.com/0xnicholas/monoli)
