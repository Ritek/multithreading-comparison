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

async fn player(
    name: String,
    mut receiver: tokio::sync::mpsc::Receiver<GameStateMessage>,
    sender: tokio::sync::mpsc::Sender<GameStateMessage>,
    engine: std::sync::Arc<dyn GameEngine>,
    is_starter: bool,
) {
    if is_starter {
        engine.log_event(&format!("{} hit {:?} (count: {})", name, Ball::Ping, 1));

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

        if !engine.should_hit(&name) {
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

        engine.log_event(&format!(
            "{} hit {:?} (count: {})",
            name, msg.ball, msg.counter
        ));

        if sender.send(msg).await.is_err() {
            break;
        }
    }
}

trait GameEngine: Send + Sync {
    fn should_hit(&self, player_name: &str) -> bool;
    fn log_event(&self, message: &str);
}

struct RealEngine;
impl GameEngine for RealEngine {
    fn should_hit(&self, _: &str) -> bool {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos()
            % 10
            != 0
    }
    fn log_event(&self, m: &str) {
        println!("[PROD] {}", m);
    }
}

struct MockEngine {
    max_hits: usize,
    hits_count: std::sync::atomic::AtomicUsize,
}
impl GameEngine for MockEngine {
    fn should_hit(&self, _: &str) -> bool {
        let current = self
            .hits_count
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        current < self.max_hits
    }
    fn log_event(&self, m: &str) {
        eprintln!("[TEST] {}", m);
    }
}

async fn run_game(dependencies: std::sync::Arc<dyn GameEngine>) {
    let (p1_send, p1_receive) = tokio::sync::mpsc::channel::<GameStateMessage>(1);
    let (p2_send, p2_receive) = tokio::sync::mpsc::channel::<GameStateMessage>(1);

    let player1 = tokio::spawn(player(
        String::from("Player1"),
        p1_receive,
        p2_send,
        dependencies.clone(),
        true,
    ));
    let player2 = tokio::spawn(player(
        String::from("Player2"),
        p2_receive,
        p1_send,
        dependencies.clone(),
        false,
    ));

    let _ = tokio::join!(player1, player2);
}

#[tokio::main]
async fn main() -> () {
    let dependencies = std::sync::Arc::new(RealEngine);
    run_game(dependencies).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fixed_length_game() {
        let engine = std::sync::Arc::new(MockEngine {
            max_hits: 4,
            hits_count: std::sync::atomic::AtomicUsize::new(0),
        });

        run_game(engine.clone()).await;

        assert_eq!(
            engine.hits_count.load(std::sync::atomic::Ordering::SeqCst),
            5
        );
    }
}
