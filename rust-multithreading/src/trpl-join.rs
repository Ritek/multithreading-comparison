use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // Produces a single new future
        // Output is a tuple containing the output of each passed in feature
        // Output returned once both complete
        trpl::join(fut1, fut2).await;

        // Output will always be the same because join is "fair"
        // join checks features equally often
        // join does not allow one feature to race ahead
    });
}
