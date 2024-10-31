use std::net::{IpAddr, Ipv4Addr};
use tarpc::{
    context,
    serde_transport::tcp::connect, 
    tokio_serde::formats::Json
};
use service::{BlackjackClient, GameState};
use tokio::io::{self, AsyncBufReadExt, BufReader};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 连接服务器
    let addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 11451);
    let transport = connect(addr, Json::default).await?;

    let client = BlackjackClient::new(Default::default(), transport).spawn();

    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    loop {
        println!("输入命令 (new, hit, stand, quit):");

        match lines.next_line().await {
            Ok(Some(input)) => {
                match input.trim() {
                    "new" => {
                        let game_state = client.new_game(context::current()).await?;
                        print_game_state(&game_state);
                    }
                    "hit" => {
                        let game_state = client.hit(context::current()).await?;
                        print_game_state(&game_state);
                        if let Some(result) = &game_state.result {
                            println!("游戏结果: {}", result);
                        }
                    }
                    "stand" => {
                        let game_state = client.stand(context::current()).await?;
                        print_game_state(&game_state);
                        if let Some(result) = &game_state.result {
                            println!("游戏结果: {}", result);
                        }
                    }
                    "quit" => {
                        println!("退出游戏");
                        break;
                    }
                    _ => {
                        println!("无效命令，请输入 'new', 'hit', 'stand' 或 'quit'");
                    }
                }
            }
            Ok(None) => {
                println!("标准输入已关闭，退出客户端。");
                break;
            }
            Err(e) => {
                println!("读取输入时出错: {}", e);
                break;
            }
        }
    }
    Ok(())
}

// 打印游戏状态的帮助函数
fn print_game_state(game_state: &GameState) {

    println!("玩家手牌: {:?}", game_state.player_hand);
    println!("庄家手牌: {:?}", game_state.dealer_hand);
    if let Some(result) = &game_state.result {
        println!("结果: {}", result);
    } else {
        println!("游戏进行中");
    }
}
