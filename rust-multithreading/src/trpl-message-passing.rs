use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        /*
        // This program will not work as intended, sending all messages after 2s
        // Every time the loop completes, it hits the await point again
        // If individual futures were awaited the flow would become sequential

          let vals = vec![
              String::from("hi"),
              String::from("from"),
              String::from("the"),
              String::from("future"),
          ];

          for val in vals {
              tx.send(val).unwrap();
              trpl::sleep(Duration::from_millis(500)).await;
          }

          while let Some(value) = rx.recv().await {
              println!("received '{value}'");
          }
        */

        // move is necessary for program to complete, tx must be dropped
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}
