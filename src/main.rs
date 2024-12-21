use rand::prelude::*;
use sha2::{Digest, Sha256};
use std::fmt;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH}; // Add this to your dependencies
const DIFFICULTY: usize = 2;

#[derive(Clone, Debug)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: f64,
    signature: String,
    timestamp: u64,
}

#[derive(Clone)] // Add Clone trait to Block
struct Block {
    index: u32,
    previous_hash: String,
    timestamp: u64,
    transactions: Vec<Transaction>,
    nonce: u64,
    hash: String,
    difficulty: usize,
}
impl Transaction {
    fn new(sender: String, recipient: String, amount: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Simple signature simulation
        let signature = format!("SIG_{}_{}", sender, timestamp);

        Transaction {
            sender,
            recipient,
            amount,
            signature,
            timestamp,
        }
    }
}

impl Block {
    fn new(
        index: u32,
        previous_hash: String,
        transactions: Vec<Transaction>,
        difficulty: usize,
    ) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            nonce: 0,
            hash: String::new(),
            difficulty,
        }
    }
    fn calculate_hash(&self) -> String {
        let transactions_str: String = self
            .transactions
            .iter()
            .map(|tx| format!("{}{}{}", tx.sender, tx.recipient, tx.amount))
            .collect();

        let data = format!(
            "{}{}{}{}{}",
            self.index, &self.previous_hash, self.timestamp, transactions_str, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    fn mine_block_with_visual_effects(&mut self) {
        let mut iterations = 0;
        loop {
            self.hash = self.calculate_hash();
            iterations += 1;
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                println!("Block Mined: {}", self.index);
                break;
            }
            if iterations > 100 {
                print!("MINING IN PROGRESS...");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated hash:{}", self.hash);
                break;
            }
            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = chrono::DateTime::from_timestamp(self.timestamp as i64, 0)
            .expect("Invalid timestamp");
        write!(f, "Block {} at {}", self.index, datetime)
    }
}
struct Wallet {
    address: String,
    balance: f64,
    transactions: Vec<Transaction>,
}
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    const MINING_REWARD: f64 = 50.0;
    const DIFFICULTY_ADJUSTMENT_INTERVAL: u32 = 10;
    const TARGET_TIME_PER_BLOCK: u64 = 10; // seconds

    fn mine_block(&mut self, miner: String, pending_transactions: Vec<Transaction>) -> Block {
        let mut transactions = pending_transactions;

        // Add mining reward transaction
        let reward_tx = Transaction::new(String::from("NETWORK"), miner, Self::MINING_REWARD);
        transactions.push(reward_tx);

        let difficulty = self.calculate_difficulty();

        let mut block = Block::new(
            self.chain.len() as u32,
            self.chain.last().unwrap().hash.clone(),
            transactions,
            difficulty,
        );

        block.mine_block_with_visual_effects();
        block
    }
    fn calculate_difficulty(&self) -> usize {
        if self.chain.len() <= Self::DIFFICULTY_ADJUSTMENT_INTERVAL as usize {
            return DIFFICULTY;
        }

        let last_block = self.chain.last().unwrap();
        let adjustment_block =
            &self.chain[self.chain.len() - Self::DIFFICULTY_ADJUSTMENT_INTERVAL as usize];

        let time_taken = last_block.timestamp - adjustment_block.timestamp;
        let expected_time =
            Self::TARGET_TIME_PER_BLOCK * Self::DIFFICULTY_ADJUSTMENT_INTERVAL as u64;

        if time_taken < expected_time / 2 {
            last_block.difficulty + 1
        } else if time_taken > expected_time * 2 {
            std::cmp::max(last_block.difficulty - 1, 1)
        } else {
            last_block.difficulty
        }
    }
    fn new() -> Blockchain {
        let genesis_block = Block::new(
            0,
            String::new(),
            Vec::new(), // empty transactions for genesis
            DIFFICULTY,
        );

        Blockchain {
            chain: vec![genesis_block],
        }
    }
    fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }
    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}
impl Wallet {
    fn new(address: String) -> Self {
        Wallet {
            address,
            balance: 1000.0, // Starting balance for testing
            transactions: Vec::new(),
        }
    }

    fn create_transaction(&mut self, recipient: String, amount: f64) -> Option<Transaction> {
        if amount <= self.balance {
            let tx = Transaction::new(self.address.clone(), recipient, amount);
            self.balance -= amount;
            self.transactions.push(tx.clone());
            Some(tx)
        } else {
            println!("Insufficient funds! Balance: {}", self.balance);
            None
        }
    }
}
fn main() {
    println!("Welcome to Bekcoin Mining Simulator v2.0!");
    println!("Enter your miner name!");

    let mut miner_name = String::new();
    std::io::stdin()
        .read_line(&mut miner_name)
        .expect("Failed to read input");
    miner_name = miner_name.trim().to_string();

    let mut blockchain = Blockchain::new();
    let mut wallets: Vec<Wallet> = Vec::new();

    // Create wallets for all traders
    let trader_names = vec![
        "Bob", "Linda", "John", "Omar", "Eve", "Svetlana", "Grace", "Jiro",
    ];

    for name in trader_names.iter() {
        wallets.push(Wallet::new(name.to_string()));
    }

    // Add miner's wallet
    let miner_wallet = Wallet::new(miner_name.clone());
    wallets.push(miner_wallet);

    println!("Starting mining simulation!");

    let mut rng = rand::thread_rng();

    for i in 0..10 {  // Simulate 10 blocks
        let mut pending_transactions = Vec::new();
        
        let num_transactions = rng.gen_range(1..=3);
        for _ in 0..num_transactions {
            let sender_idx = rng.gen_range(0..wallets.len());
            let mut recipient_idx = rng.gen_range(0..wallets.len());
            while recipient_idx == sender_idx {
                recipient_idx = rng.gen_range(0..wallets.len());
            }
            
            let amount = rng.gen_range(1.0..100.0);
            
            // Get recipient address before transaction
            let recipient_address = wallets[recipient_idx].address.clone();
            
            // Now create the transaction
            if let Some(tx) = wallets[sender_idx].create_transaction(
                recipient_address,
                amount
            ) {
                pending_transactions.push(tx);
            }
        }

        println!("Mining block {} with {} transactions...", i + 1, pending_transactions.len());
        let block = blockchain.mine_block(miner_name.clone(), pending_transactions);
        blockchain.add_block(block);
    }
    // Print final statistics
    println!("\nMining completed!");
    println!("Total blocks: {}", blockchain.get_total_blocks());
    println!("Final blockchain state:");
    for block in blockchain.chain.iter() {
        println!("\nBlock {}:", block.index);
        println!("Hash: {}", block.hash);
        println!("Transactions:");
        for tx in block.transactions.iter() {
            println!("  {} sent {} BEK to {}", tx.sender, tx.amount, tx.recipient);
        }
    }
}
