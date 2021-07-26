use crate::Redis;
use anyhow::anyhow;
use futures::StreamExt;
use polygon::ws::PolygonMessage;
use rdkafka::consumer::StreamConsumer;
use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
use rust_decimal::prelude::*;
use tracing::{debug, error, trace};

pub struct Relay {
    consumer: StreamConsumer,
    redis: Redis,
}

impl Relay {
    pub fn new(consumer: StreamConsumer, redis: Redis) -> Self {
        Self { consumer, redis }
    }

    pub async fn run(&self) {
        tracing::info!("Starting relay");
        self.consumer
            .stream()
            .for_each_concurrent(100, |msg| async {
                if let Err(e) = self.handle_message(msg).await {
                    error!("{}", e)
                }
            })
            .await;
    }

    async fn handle_message(
        &self,
        msg: Result<BorrowedMessage<'_>, rdkafka::error::KafkaError>,
    ) -> anyhow::Result<()> {
        if let Ok(msg) = msg {
            if let Some(payload) = msg.payload() {
                if let Ok(PolygonMessage::Trade(trade)) =
                    serde_json::from_slice::<PolygonMessage>(payload)
                {
                    if trade.is_eligible() {
                        trace!(ticker = %trade.symbol, price = %trade.price, "Trade");
                        let key = format!("price/{}", trade.symbol);
                        let _ = self.redis.set(&key, convert_price(trade.price)?).await;
                    }
                    if trade.is_opening() {
                        debug!(ticker = %trade.symbol, price = %trade.price, "Open");
                        let key = format!("open/{}", trade.symbol);
                        let _ = self.redis.set(&key, convert_price(trade.price)?).await;
                    } else if trade.is_closing() {
                        debug!(ticker = %trade.symbol, price = %trade.price, "Close");
                        let key = format!("close/{}", trade.symbol);
                        let _ = self.redis.set(&key, convert_price(trade.price)?).await;
                    }
                }
            }
        };
        Ok(())
    }
}

fn convert_price(price: Decimal) -> anyhow::Result<f64> {
    let price = price
        .to_f64()
        .ok_or_else(|| anyhow!("Failed to convert decimal to float"))?;
    Ok(price)
}
