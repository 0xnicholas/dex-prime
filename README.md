# dex-prime
DEX using CLOB, based on solana/monoli L1.

**codebase**
```
monoli-dex/
├── Cargo.toml
├── README.md
│
├── engine/                      # 撮合引擎核心逻辑 (Rust，高性能)
│   ├── mod.rs
│   ├── orderbook.rs            # 限价订单簿
│   ├── matching.rs             # 撮合算法（价格优先、时间优先）
│   ├── risk.rs                 # 仓位检查 / 杠杆风控
│   └── types.rs                # Order, Position, Market 等结构体定义
│
├── chain_adapter/              # 链执行适配器（链可插拔接口）
│   ├── mod.rs
│   ├── types.rs                # ChainTx, ChainTxResult, Action Enum
│   ├── executor.rs             # Trait 定义
│   ├── solana_executor.rs      # Solana-specific 实现
│   ├── monoli_executor.rs      # Monoli-specific 实现（预留）
│   └── mock_executor.rs        # 本地测试用模拟链
│
├── settlement/                 # 清算模块（与链适配器协作）
│   ├── mod.rs
│   ├── liquidation.rs          # 清算触发判断逻辑
│   ├── funding.rs              # Funding Rate 结算
│   └── pnl.rs                  # PnL 计算器
│
├── api/                        # 对外 HTTP / WS 接口服务
│   ├── mod.rs
│   ├── http.rs                 # Restful API：下单、查询、撤单
│   ├── ws.rs                   # WebSocket：行情订阅、推送
│   └── auth.rs                 # 身份认证 / API Key
│
├── infra/                      # 数据库、消息队列、KV 等
│   ├── db.rs                   # PostgreSQL / Clickhouse 存储接口
│   ├── kv.rs                   # Redis KV（用户仓位、市场快照等）
│   ├── pubsub.rs               # Kafka / Redis pubsub 推送
│   └── config.rs               # 全局配置项加载（.env / TOML）
│
├── indexer/                    # 链上事件监听器（链上状态同步）
│   ├── mod.rs
│   ├── solana_indexer.rs       # 从 Solana 读取 Vault/Position 状态
│   └── monoli_indexer.rs       # Monoli L1 用于替代 Solana
│
├── tests/                      # 集成测试套件（可用 Rust e2e test 或 BDD）
│   ├── engine_tests.rs
│   ├── executor_tests.rs
│   └── api_tests.rs
│
├── scripts/                    # 启动脚本 / 模拟部署 / 本地测试
│   ├── localnet.sh
│   ├── migrate_db.rs
│   └── start_dev.rs
│
└── onchain/                    # Anchor 程序目录（Solana链上逻辑）
    ├── programs/monoli_dex/
    │   └── src/
    │       ├── lib.rs
    │       ├── instructions/    # open_position, close, settle 等
    │       └── state/           # vault, position, market, funding 等
    └── Anchor.toml
```
