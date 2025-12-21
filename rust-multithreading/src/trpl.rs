use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;

    // or
    // let response_text = trpl::get(url).await.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

// Can be run with 'cargo run --bin trpl -- "https://www.rust-lang.org"'
// main function cannot be async
// Rust hands back control to the runtime whenever await is encountered
// Rust creates a state machine for every async code block
fn main() {
    let args: Vec<String> = std::env::args().collect();

    // block_on blocks until feature completes
    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
