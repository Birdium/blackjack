# 实验报告

在本次实验中，我使用 rust 语言实现了一个简单的 Blackjack Client-Server 小游戏 

## 分析与设计

首先流程上，服务器会与每个客户端建立连接，并维护游戏的状态。客户端通过发送RPC调用来进行监护，服务器在收到RPC后对游戏状态进行操作，并返回客户可见的游戏状态。

首先要考虑的是：需要设计哪些RPC接口？ 

在实验中，我们给出了下面这些接口：

```rust
#[tarpc::service]
pub trait Blackjack {
    async fn new_game() -> GameState;
    async fn hit() -> GameState;
    async fn stand() -> GameState;
}
```

其中 `GameState` 定义如下：

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub player_hand: Vec<Card>, // 玩家手牌
    pub dealer_hand: Vec<Card>, // 庄家手牌（明牌）
    pub result: Option<String>, // 游戏结果
}
```

客户端通过发送 `new_game()` 来开始游戏， 服务端会初始化牌堆，并将明牌发送给玩家。

客户端此时可以选择 hit 或者 stand。在分别调用函数后，服务端进行逻辑操作，判断游戏状态。如果客户端 burst 了，那么 `GameState.result` 会返回 "Dealer wins"。

如果客户端选择 stand，那么服务端会在手牌点数不超过17的情况下持续抽卡。最后服务端进行结果的逻辑判断。

接下来我将介绍实验每一部分的具体实现：

`lib.rs`: 这个文件提供了客户端服务端都需要的数据结构（卡牌属性相关）和RPC接口。在编译的时候，这一部分作为库文件被提供给客户端与服务端

下面是服务端的实现：

`deck.rs`: 这个文件抽象了牌堆的操作：初始化、洗牌、抽牌。

`game.rs`: 这个文件抽象了服务端所见游戏状态与操作：开始新游戏、抽初始牌、玩家hit、庄家hit和判断胜负

`player.rs`: 这个文件抽象了玩家与庄家状态：拥有的手牌、抽牌操作以及一些方便计算玩家状态的函数

`server.rs`: 这个文件实现了上文的 RPC 接口，同时包含了服务器的初始化流程。这里我们用到了 `tarpc` 作为 rpc 库， 同时使用了 rust 的 `tokio` 实现服务器的异步操作，`serde` 作为传输的序列化方案。  

下面是客户端的实现：

`client.rs`: client 从命令行读取指令，并调用RPC接口和 server 通信。在每次操作后，client 会在终端打印当前游戏状态信息。

## 实现演示

编译运行服务端：

```bash
cargo run --bin server
```

服务端默认监听 11451 端口

接下来编译运行客户端：

```bash
cargo run --bin client
```

终端显示如下信息：

```
输入命令 (new, hit, stand, quit):
```

我们输入 new:

```
new
玩家手牌: [Card { suit: Spades, rank: Eight }, Card { suit: Hearts, rank: Five }]
庄家手牌: [Card { suit: Spades, rank: Five }]
游戏进行中
输入命令 (new, hit, stand, quit):
```

输入 hit:

```
hit
玩家手牌: [Card { suit: Spades, rank: Eight }, Card { suit: Hearts, rank: Five }, Card { suit: Diamonds, rank: Six }]
庄家手牌: [Card { suit: Spades, rank: Five }]
游戏进行中
输入命令 (new, hit, stand, quit):
```

玩家手牌和已经达到19了，因此输入stand：

```
stand
玩家手牌: [Card { suit: Spades, rank: Eight }, Card { suit: Hearts, rank: Five }, Card { suit: Diamonds, rank: Six }]
庄家手牌: [Card { suit: Spades, rank: Five }, Card { suit: Diamonds, rank: Eight }, Card { suit: Clubs, rank: Ten }]
结果: Player wins
游戏结果: Player wins
输入命令 (new, hit, stand, quit):
```

很不幸，庄家 bust 了，因此玩家获胜。

玩家 bust 庄家获胜的情况：

```
new
玩家手牌: [Card { suit: Diamonds, rank: Nine }, Card { suit: Spades, rank: Ten }]
庄家手牌: [Card { suit: Hearts, rank: Ace }]
游戏进行中
输入命令 (new, hit, stand, quit):
hit
玩家手牌: [Card { suit: Diamonds, rank: Nine }, Card { suit: Spades, rank: Ten }, Card { suit: Diamonds, rank: Eight }]
庄家手牌: [Card { suit: Hearts, rank: Ace }]
结果: Dealer wins
游戏结果: Dealer wins
```

玩家点数不如庄家的情况：

```
new
玩家手牌: [Card { suit: Clubs, rank: Seven }, Card { suit: Diamonds, rank: Two }]
庄家手牌: [Card { suit: Clubs, rank: Two }]
游戏进行中
输入命令 (new, hit, stand, quit):
hit
玩家手牌: [Card { suit: Clubs, rank: Seven }, Card { suit: Diamonds, rank: Two }, Card { suit: Hearts, rank: Seven }]
庄家手牌: [Card { suit: Clubs, rank: Two }]
游戏进行中
输入命令 (new, hit, stand, quit):
stand
玩家手牌: [Card { suit: Clubs, rank: Seven }, Card { suit: Diamonds, rank: Two }, Card { suit: Hearts, rank: Seven }]
庄家手牌: [Card { suit: Clubs, rank: Two }, Card { suit: Hearts, rank: Two }, Card { suit: Hearts, rank: Ace }, Card { suit: Diamonds, rank: Ten }, Card { suit: Spades, rank: Four }]
结果: Dealer wins
游戏结果: Dealer wins
```

## 总结

通过本次实验，我了解了RPC的接口设计，对分布式网络通信有了更深刻的了解。同时我学习了rust的tarpc, tokio等库，提升了自己的编程能力。