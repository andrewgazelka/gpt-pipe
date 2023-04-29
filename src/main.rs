use std::io::Write;
use std::pin::pin;
use std::process::ExitCode;

use anyhow::{ensure, Context};
use clap::Parser;
use futures::future::Either;
use futures::StreamExt;
use once_cell::sync::Lazy;
use tokio::io::{stdin, AsyncReadExt};
use tokio_openai::ChatRequest;
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

#[derive(Parser)]
#[clap(author, about, version)]
struct Args {
    /// System message as a file
    #[clap(short, long)]
    sys: Option<String>,
}

async fn sys_message(path: &str) -> anyhow::Result<String> {
    let res = tokio::fs::read_to_string(path).await?;
    Ok(res)
}

async fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    let sys_message = match args.sys {
        Some(path) => Some(sys_message(&path).await?),
        None => None,
    };

    // read all stdin
    let mut input = Vec::new();
    let openai = tokio_openai::Client::simple()?;

    stdin().read_to_end(&mut input).await?;
    ensure!(!input.is_empty(), "no input from stdin");
    let input = String::from_utf8(input).context("invalid utf8")?;

    let mut request = ChatRequest::new();

    if let Some(sys_message) = sys_message {
        request = request.sys_msg(sys_message);
    }

    request = request.user_msg(input);

    let mut res = openai.stream_chat(request).await?;

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
