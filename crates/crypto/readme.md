##  助记词钱包的生成流程（符合 BIP-39 / BIP-32 标准）
1. 生成熵
   - 首先生成一段随机熵值（Entropy），例如 128 位、160 位或 256 位
     - 128位熵 - 12个助记词
     - 256位熵 - 24个助记词
   - 熵的随机性必须来自安全随机源（CSPRNG），是整个钱包安全的根基
     - 关于安全随机源 
2. 熵->助记词（Mnemonic, BIP-39）
   - 对熵计算一次 SHA-256，取前若干位作为校验位（checksum）
   - 将（熵+校验位） 按11位一组分割，每组对应一个2048词表中的单词，从而得到助记词，如：
```legal winner thank year wave sausage worth useful legal winner thank yellow```
3. 
- 使用椭圆曲线算法得到私钥和区块链码
- 通过区块链码得到HD子地址