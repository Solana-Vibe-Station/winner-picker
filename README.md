# Solana Winner Picker 🎲

A simple Rust program that selects a random winner from a list of participants using a future Solana blockchain slot's blockhash as the source of randomness.

## How it Works

1. Reads a list of participants from a text file
2. Waits for a predetermined Solana slot to be produced
3. Uses the blockhash of that slot as a source of randomness
4. Applies SHA256 to the blockhash and uses the result to select a winner

This approach ensures the selection is:
- **Transparent**: Anyone can verify the result using the same blockhash
- **Unpredictable**: The blockhash cannot be known until the slot is produced
- **Fair**: No one can manipulate the outcome

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- A Solana RPC endpoint (default uses the public Solana Vibe Station endpoint)

## Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd winner-picker
```

2. Build the project:
```bash
cargo build --release
```

## Usage

1. Edit the `participants.txt` file in the project root with one participant per line:
```
athena
boreas
cerberus
daedalus
```

2. Determine a future slot number for the drawing. You can calculate this based on:
   - ~216,000 slots per day
   - ~9,000 slots per hour
   - ~150 slots per minute

3. Update the configuration in `main.rs`:
```rust
let rpc_url = "https://public.rpc.solanavibestation.com";  // Your RPC endpoint
let slot_number: u64 = 336533075;  // Your chosen future slot
let participants_file = "participants.txt";  // Path to participants file
```

4. Run the program after the slot has been produced:
```bash
cargo run
```

## Example Output

```
✅ Blockhash for slot 336533075: 8xKEgKPx7BZ3GvZMxFMooKnLwY5p6WFhJYTwMd3pbAiG
🏆 Winner: Charlie
```

## Verifying the Result

Anyone can verify the selection by:
1. Getting the blockhash for the specified slot from any Solana RPC
2. Running the same algorithm with the same participant list
3. Confirming the same winner is selected

## Notes

- The slot must be in the future when announced to ensure fairness
- The program will fail if run before the slot is produced
- Empty lines in the participants file are ignored
- The RPC endpoint must support versioned transactions

## How Randomness Works

The selection process:
1. Takes the blockhash (32 bytes) from the specified slot
2. Applies SHA256 to create a uniform distribution
3. Uses the first 8 bytes as a number
4. Performs modulo operation with the number of participants
5. Selects the winner at that index

## License

MIT
