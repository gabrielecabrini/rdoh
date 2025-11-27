use clap::Parser;
use doh::resolve;
use doh::{Query, RRType};
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    name: String,

    #[arg(value_enum, required = false)]
    r#type: Option<RRType>,

    #[arg(
        required = false,
        default_value = "https://cloudflare-dns.com/dns-query"
    )]
    server: String,

    #[arg(long, action=clap::ArgAction::SetTrue)]
    r#do: bool,

    #[arg(long, action=clap::ArgAction::SetTrue)]
    r#cd: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let query = Query {
        name: args.name,
        r#type: args.r#type,
        r#do: args.r#do,
        cd: args.r#cd,
    };

    match resolve(&args.server, &query).await {
        Ok(response) => {
            println!("{}", response);
        }
        Err(error) => {
            println!("DNS resolve failed: {}", error);
            exit(1);
        }
    }
}
