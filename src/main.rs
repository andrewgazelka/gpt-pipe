use std::io::Write;
use std::process::ExitCode;

use anyhow::{ensure, Context};
use futures::StreamExt;
use tokio::io::{stdin, AsyncReadExt};
use tokio_openai::ChatRequest;

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(e) = run().await {
        eprintln!("error: {}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

async fn run() -> anyhow::Result<()> {
    // read all stdin
    let mut input = Vec::new();

    stdin().read_to_end(&mut input).await?;

    ensure!(!input.is_empty(), "no input from stdin");

    let input = String::from_utf8(input).context("invalid utf8")?;

    let openai = tokio_openai::Client::simple()?;

    let instructions = std::env::args().skip(1).collect::<Vec<_>>();

    ensure!(!instructions.is_empty(), "no instructions provided");

    let instructions = instructions.join(" ");

    let input = format!("{input}\n---\n{instructions}");

    let request = ChatRequest::from(&input);

    let mut res = openai.stream_chat(request).await?.boxed();

    while let Some(res) = res.next().await {
        let res = res?;

        print!("{}", res);

        // flush
        std::io::stdout().flush()?;
    }

    println!();

    Ok(())
}
