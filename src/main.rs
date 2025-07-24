//  Compare the speed of a self-hosted node and other RPCs
use futures_util::StreamExt;
use shred_com::common::utils::init_env_logger;
use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_client::RpcClient};

const RPC: &str = "https://mainnet.helius-rpc.com/?api-key=your-api-key";
const PUBSUB: &str = "wss://mainnet.helius-rpc.com/?api-key=your-api-key";

#[tokio::main]
async fn main() {
    init_env_logger();

    { // Use get_slot to determine the time from the external source to the local node
        use std::time::Instant;
        let client = RpcClient::new(RPC);
        let _ = client.get_slot()
            .expect("fail to get slot");
        let start = Instant::now(); // Use the second request as the reference
        let _ = client.get_slot()
            .expect("fail to get slot");
        let duration = start.elapsed().as_millis() / 2;
        log::info!("external latency: {}ms", duration);
    }

    // listen external
    tokio::spawn(async {
        let client = PubsubClient::new(PUBSUB)
            .await.unwrap();
        log::info!("external connected");
        // let (mut stream, _unsubscribe) = client.slot_subscribe().await.unwrap();
        // while let Some(info) = stream.next().await {
        //     log::info!("external slot {}", info.slot);
        // }
        let (mut stream, _unsubscribe) = client.slot_updates_subscribe().await.unwrap();
        while let Some(update) = stream.next().await {
            if let solana_client::rpc_response::SlotUpdate
            ::FirstShredReceived { slot, timestamp } = update {
                log::info!("external shred {} at {}",
                    slot, chrono::DateTime::<chrono::Utc>::from(
                       std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(timestamp)
                    ).format("%Y-%m-%d %H:%M:%S%.3f"),
                );
            }
        }
    });

    { // listen adjacent
        let adjacent_pubsub = "ws://localhost:8900";
        let client = PubsubClient::new(&adjacent_pubsub)
            .await.unwrap();
        log::info!("adjacent connected");
        // let (mut stream, _unsubscribe) = client.slot_subscribe().await.unwrap();
        // while let Some(info) = stream.next().await {
        //     log::info!("adjacent slot {}", info.slot);
        // }
        let (mut stream, _unsubscribe) = client.slot_updates_subscribe().await.unwrap();
        while let Some(update) = stream.next().await {
            if let solana_client::rpc_response::SlotUpdate
            ::FirstShredReceived { slot, timestamp } = update {
                log::info!("adjacent shred {} at {}",
                    slot, chrono::DateTime::<chrono::Utc>::from(
                       std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(timestamp)
                    ).format("%Y-%m-%d %H:%M:%S%.3f"),
                );
            }
        }
    }
}