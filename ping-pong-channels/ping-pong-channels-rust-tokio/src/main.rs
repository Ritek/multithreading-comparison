#[derive(Debug)]
enum Ball {
    Ping,
    Pong,
    Done(String),
}

#[derive(Debug)]
struct GameStateMessage {
    ball: Ball,
    counter: usize,
}

fn try_hit() -> bool {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    nanos % 100 != 99
}

async fn player(
    name: String,
    mut receiver: tokio::sync::mpsc::Receiver<GameStateMessage>,
    sender: tokio::sync::mpsc::Sender<GameStateMessage>,
    is_starter: bool,
) {
    if is_starter {
        let _ = sender
            .send(GameStateMessage {
                ball: Ball::Ping,
                counter: 1,
            })
            .await;
    }

    while let Some(mut msg) = receiver.recv().await {
        if let Ball::Done(reason) = &msg.ball {
            println!("{} received Game Over: {}", name, reason);
            break;
        }

        if !try_hit() {
            println!("{} dropped the ball!", name);
            let _ = sender
                .send(GameStateMessage {
                    ball: Ball::Done(format!("Missed by {}", name)),
                    counter: msg.counter,
                })
                .await;
            break;
        }

        msg.counter += 1;
        msg.ball = match msg.ball {
            Ball::Ping => Ball::Pong,
            _ => Ball::Ping,
        };

        println!("{} hit {:?} (count: {})", name, msg.ball, msg.counter);

        if sender.send(msg).await.is_err() {
            break;
        }
    }
}

#[tokio::main]
async fn main() -> () {
    let (p1_send, p1_receive) = tokio::sync::mpsc::channel::<GameStateMessage>(1);
    let (p2_send, p2_receive) = tokio::sync::mpsc::channel::<GameStateMessage>(1);

    let player1 = tokio::spawn(player(String::from("Player1"), p1_receive, p2_send, true));
    let player2 = tokio::spawn(player(String::from("Player2"), p2_receive, p1_send, false));

    let _ = tokio::join!(player1, player2);
}
