#[macro_use]
extern crate prettytable;
use clap::{AppSettings, Clap};
use datastore_client::{Client, GetAggregates, GetDividends, GetSplits};
use prettytable::Table;

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
    #[clap(arg_enum)]
    resource: Resource,
}

#[derive(Clap, Debug)]
enum Resource {
    Aggregates,
    Dividends,
    Splits,
}

#[tokio::main]
async fn main() {
    let opts: Options = Options::parse();
    let client = Client::from_env().unwrap();
    match opts.method {
        Method::Get(get) => {
            let mut table = Table::new();
            match get.resource {
                Resource::Aggregates => {
                    let result = client.send(GetAggregates {}).await.unwrap();
                    table.add_row(row![
                        "open", "high", "low", "close", "volume", "start", "end", "ticker",
                    ]);
                    for agg in result {
                        table.add_row(row![
                            agg.open, agg.high, agg.low, agg.close, agg.volume, agg.start, agg.end,
                            agg.ticker,
                        ]);
                    }
                }
                Resource::Dividends => {
                    let result = client.send(GetDividends {}).await.unwrap();
                    table.add_row(row![
                        "amount",
                        "declared_date",
                        "ex_date",
                        "record_date",
                        "payment_date",
                        "ticker",
                    ]);
                    for dividend in result {
                        table.add_row(row![
                            dividend.amount,
                            dividend.declared_date,
                            dividend.ex_date,
                            dividend.record_date,
                            dividend.payment_date,
                            dividend.ticker
                        ]);
                    }
                }
                Resource::Splits => {
                    let result = client.send(GetSplits {}).await.unwrap();
                    table.add_row(row![
                        "ratio",
                        "declared_date",
                        "ex_date",
                        "ticker",
                        "from_factor",
                        "to_factor"
                    ]);
                    for split in result {
                        table.add_row(row![
                            split.ratio,
                            split.declared_date,
                            split.ex_date,
                            split.ticker,
                            split.from_factor,
                            split.to_factor
                        ]);
                    }
                }
            }
            table.printstd()
        }
    };
}
