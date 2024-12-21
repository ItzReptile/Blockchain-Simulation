Bekcoin Mining Simulator
A simple blockchain implementation in Rust that simulates mining operations and basic transactions between traders.
Features

SHA-256 based block hashing
Proof of Work (PoW) mining simulation with difficulty adjustment
Visual mining progress indicators
Basic transaction simulation between traders
Genesis block generation
Timestamp tracking for blocks
Block rewards system (137 Bekcoins per block)

Requirements

Rust (Latest Stable Version)
Cargo

Dependencies
tomlCopy[dependencies]
sha2 = "0.10.8"
chrono = "0.4"
Installation

Clone the repository:

bashCopygit clone https://github.com/yourusername/bekcoin-mining-simulator
cd bekcoin-mining-simulator

Build the project:

bashCopycargo build --release
Usage
Run the simulator:
bashCopycargo run
The program will:

Ask for your miner name
Create a genesis block
Simulate transactions between predefined traders
Display mining progress and transaction details
Show final statistics including total blocks and Bekcoins traded

Technical Details

Block Structure:

Index (block number)
Previous block's hash
Timestamp
Transaction data
Nonce (for mining)
Current block's hash


Mining Difficulty: Set to require 2 leading zeros in block hash
Block Time: Approximately 3 seconds between blocks
Maximum Mining Iterations: 100 per block

Future Improvements

Add proper transaction validation
Implement wallet system
Add network simulation
Persist blockchain to disk
Add more complex consensus mechanisms
