#[macro_use]
extern crate prettytable;
use clap::{AppSettings, Clap};
use datastore_client::Client;
use std::str::FromStr;

mod get;
use get::get_resource;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    #[clap(subcommand)]
    method: Method,
}

#[derive(Clap, Debug)]
enum Method {
    #[clap()]
    Get(Get),
}

#[derive(Clap, Debug)]
struct Get {
    #[clap(short, long, default_value = "table")]
    format: OutputFormat,
    #[clap(arg_enum)]
    resource: Resource,
}

#[derive(Clap, Debug)]
pub enum OutputFormat {
    Table,
    Json,
}

impl FromStr for OutputFormat {
    type Err = String;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            _ => Err("invalid output format".to_string()),
        }
    }
}

#[derive(Clap, Debug)]
pub enum Resource {
    Aggregates,
    Dividends,
    Splits,
}

#[tokio::main]
async fn main() {
    let opts: Options = Options::parse();
    let client = Client::from_env().unwrap();
    match opts.method {
        Method::Get(get) => get_resource(client, get.resource, get.format).await,
    };
}
