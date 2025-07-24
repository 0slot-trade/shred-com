# Solana Shreds Speed Comparator

This is a simple Rust utility that compares the responsiveness of a **self-hosted Solana node** to a public Solana **RPC endpoint** (like Helius). It does so by:

1. Measuring slot retrieval latency using `get_slot()`.
2. Subscribing to slot updates (`slotUpdatesSubscribe`) via WebSocket for both the external RPC and local node.
3. Logging the timestamps of the first shred received for each slot.

---

## ✨ Features

* Measures and logs latency of `get_slot()` on external RPC.
* Subscribes to `slotUpdatesSubscribe` for both:

  * External RPC (e.g. Helius)
  * Local node (`ws://localhost:8900`)
* Logs `slot` and `timestamp` of first shred received from both sources.
* Can be used to visually compare propagation speed and health of Solana nodes.

---

## 📦 Build & Run

### Build

```bash
cargo build --release
```

### Run

```bash
RUST_LOG=info cargo run --release
```

---

## 🔧 Configuration

### Step 1. replace the default values in the source code:

```rust
const RPC: &str = "https://mainnet.helius-rpc.com/?api-key=your-api-key";
const PUBSUB: &str = "wss://mainnet.helius-rpc.com/?api-key=your-api-key";
```

Replace `"your-api-key"` with your actual Helius or RPC provider key.

Set adjacent_pubsub to the corrent rpc port. Assume the local solana node's rpc port is 8900:
```rust
let adjacent_pubsub = "ws://localhost:8900";
```
Replace 8900 with your actual rpc port.

Run this tool and get the output:
```
[2025-07-22 13:49:52.604020 INFO src/main.rs:66] adjacent shred 355004229 at 2025-07-22 13:49:52.603
[2025-07-22 13:49:52.612685 INFO src/main.rs:44] external shred 355004229 at 2025-07-22 13:49:52.601
[2025-07-22 13:49:52.956191 INFO src/main.rs:66] adjacent shred 355004230 at 2025-07-22 13:49:52.955
[2025-07-22 13:49:52.958656 INFO src/main.rs:44] external shred 355004230 at 2025-07-22 13:49:52.954
[2025-07-22 13:49:53.312734 INFO src/main.rs:66] adjacent shred 355004231 at 2025-07-22 13:49:53.312
[2025-07-22 13:49:53.321156 INFO src/main.rs:44] external shred 355004231 at 2025-07-22 13:49:53.311
```
It shows adjacent (our node) is slower.

### Step 2. Firewall settings:
Block the shreds from solana main net. And only allow the shreds from shred-zdn running in the same server with our solana node.
```
# First, allow local loopback access to tvu port 8001 as default  
sudo ufw allow from 127.0.0.1 to any port 8001 proto udp

# Then, deny all other access to tvu port 8001 as default  
sudo ufw deny 8001/udp

# Check rules to confirm  
sudo ufw status numbered
```

To find your Solana TVU port, run solana-validator contact-info (or agave-validator contact-info, depending on the client).

### Step 3. Run shred-com
You can only see external shred due to shreds from solana main net have been blocked int step 2.
```
[2025-07-22 13:55:10.377981 INFO src/main.rs:44] external shred 355005231 at 2025-07-22 13:55:10.371
[2025-07-22 13:55:10.821683 INFO src/main.rs:44] external shred 355005232 at 2025-07-22 13:55:10.813
[2025-07-22 13:55:11.294355 INFO src/main.rs:44] external shred 355005233 at 2025-07-22 13:55:11.289
```

### Step 4. Run shred-zdn
Make sure your Solana validator is running locally with tvu port set to 8001 (by default):

sudo ./shred-zdn \
  --auth YOUR_AUTH_KEY \
  --port 18888 \
  --interface lo \
  --sniffer-port 8001 \
  --forwards 127.0.0.1:8001

### check the output of Solana Shreds Speed Comparator

Sample Output

```
[2025-07-22 14:21:27.807704 INFO src/main.rs:66] adjacent shred 355009049 at 2025-07-22 14:21:27.807
[2025-07-22 14:21:27.815859 INFO src/main.rs:44] external shred 355009049 at 2025-07-22 14:21:27.811
[2025-07-22 14:21:28.166906 INFO src/main.rs:66] adjacent shred 355009050 at 2025-07-22 14:21:28.164
[2025-07-22 14:21:28.177193 INFO src/main.rs:44] external shred 355009050 at 2025-07-22 14:21:28.168
```

This helps you assess which node receives slot updates faster. In the example above, it shows that the adjacent node (our node) receives shreds faster, benefiting from shred-zdn.

---

## 📃 License

MIT License