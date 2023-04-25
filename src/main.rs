use std::io::Write;
use std::pin::pin;
use std::process::ExitCode;

use anyhow::{ensure, Context};
use futures::future::Either;
use futures::StreamExt;
use once_cell::sync::Lazy;
use tokio::io::{stdin, AsyncReadExt};
use tokio_util::sync::CancellationToken;

static CANCEL_TOKEN: Lazy<CancellationToken> = Lazy::new(CancellationToken::new);

#[tokio::main]
async fn main() -> ExitCode {
    ctrlc::set_handler(move || {
        CANCEL_TOKEN.cancel();
    })
    .unwrap();

    if let Err(e) = run().await {
        eprintln!("error: {}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

async fn run() -> anyhow::Result<()> {
    // read all stdin
    let mut input = Vec::new();
    let openai = tokio_openai::Client::simple()?;

    stdin().read_to_end(&mut input).await?;
    ensure!(!input.is_empty(), "no input from stdin");
    let input = String::from_utf8(input).context("invalid utf8")?;

    let instructions = std::env::args().skip(1).collect::<Vec<_>>();
    ensure!(!instructions.is_empty(), "no instructions provided");

    let instructions = instructions.join(" ");

    let input = format!("{input}\n---\n{instructions}");
    let mut res = openai.stream_chat(&input).await?;

    loop {
        let next = res.next();
        let cancel = CANCEL_TOKEN.cancelled();

        let next = pin!(next);
        let cancel = pin!(cancel);

        let next = match futures::future::select(next, cancel).await {
            Either::Left((Some(next), ..)) => next?,
            _ => {
                break;
            }
        };

        print!("{}", next);
        std::io::stdout().flush()?; // flush
    }

    Ok(())
}
