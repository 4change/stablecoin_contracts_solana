## 运行
```shell
nvm use v22.19.0

anchor build

anchor deploy

anchor test                             # 启动本地 Solana 验证器，并运行 Anchor 测试
anchor test --skip-local-validator      # 不启动本地 Solana 验证器，使用已部署的本地 Solana 验证器运行 Anchor 测试
```