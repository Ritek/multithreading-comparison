use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{self, sync::Notify};

#[derive(Debug, PartialEq, Eq)]
enum Ball {
    PING,
    PONG,
    DONE(String),
}

fn is_received() -> bool {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    nanos % 100 != 99
}

async fn hit_ball(ball: &mut Ball) {
    match ball {
        Ball::PING => {
            *ball = Ball::PONG;
        }
        Ball::PONG => {
            *ball = Ball::PING;
        }
        Ball::DONE(_) => {
            panic!("hit_ball called with Ball::DONE!");
        }
    }
}

struct GameState {
    counter: usize,
    ball: Ball,
}

struct SharedState {
    game_state: tokio::sync::Mutex<GameState>,
    notifier: Notify,
}

#[tokio::main]
async fn main() -> () {
    let initial_state = GameState {
        counter: 0,
        ball: Ball::PONG,
    };
    let shared_state = Arc::new(SharedState {
        game_state: tokio::sync::Mutex::new(initial_state),
        // Notifier outside of Mutex for easy access
        notifier: tokio::sync::Notify::new(),
    });

    let player1_ball = Arc::clone(&shared_state);
    let player2_ball = Arc::clone(&shared_state);

    let player1 = tokio::spawn(async move {
        loop {
            let recived = is_received();
            if recived {
                {
                    let mut state = player1_ball.game_state.lock().await;
                    hit_ball(&mut state.ball);
                    state.counter += 1;
                    player1_ball.notifier.notify_one();
                }
                player1_ball.notifier.notified().await;
            } else {
                {
                    let mut state = player1_ball.game_state.lock().await;
                    state.ball = Ball::DONE(String::from("Dropped by player1"));
                }
                break;
            }
        }
    });

    let player2 = tokio::spawn(async move {
        loop {
            let received = is_received();
            if received {
                {
                    let mut state = player2_ball.game_state.lock().await;
                    hit_ball(&mut state.ball);
                    state.counter += 1;
                    player2_ball.notifier.notify_one();
                }
                player2_ball.notifier.notified().await;
            } else {
                {
                    let mut state = player2_ball.game_state.lock().await;
                    state.ball = Ball::DONE(String::from("Dropped by player1"));
                }
                break;
            }
        }
    });

    tokio::select! {
        _ = player1 => (),
        _ = player2 => (),
    }

    // Awaiting game state BEFORE SELECT will casue players to never acquire the lock
    let final_result = shared_state.game_state.lock().await;
    println!(
        "{:?}. Total hits: {}",
        final_result.ball, final_result.counter
    );
}
