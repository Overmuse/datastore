use crate::Redis;
use futures::StreamExt;
use polygon::ws::PolygonMessage;
use rdkafka::consumer::StreamConsumer;
use rdkafka::Message;
use rust_decimal::prelude::*;
use tracing::{debug, trace};

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
                if let Ok(msg) = msg {
                    if let Some(payload) = msg.payload() {
                        let maybe = serde_json::from_slice::<PolygonMessage>(payload);
                        if let Ok(PolygonMessage::Trade(trade)) = maybe {
                            if trade.is_eligible() {
                                trace!(ticker = %trade.symbol, price = %trade.price, "Trade");
                                let key = format!("price/{}", trade.symbol);
                                let _ = self.redis.set(&key, trade.price.to_f64().unwrap()).await;
                            }
                            if trade.is_opening() {
                                debug!(ticker = %trade.symbol, price = %trade.price, "Open");
                                let key = format!("open/{}", trade.symbol);
                                let _ = self.redis.set(&key, trade.price.to_f64().unwrap()).await;
                            } else if trade.is_closing() {
                                debug!(ticker = %trade.symbol, price = %trade.price, "Close");
                                let key = format!("close/{}", trade.symbol);
                                let _ = self.redis.set(&key, trade.price.to_f64().unwrap()).await;
                            }
                        }
                    }
                }
            })
            .await;
    }
}
