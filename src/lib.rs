use std::collections::HashMap;

// Name Assignment (variables and constants)
// TODO: Assign the current bitcoin mining reward
pub const MINING_REWARD: f64 = 3.125;
// TODO: Assign the current block height
pub const CURRENT_BLOCK_HEIGHT: u64 = 850_000;
// TODO: Assign the number of satoshis in one Bitcoin
pub const BTC_TO_SATS: u64 = 100_000_000;

#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}

/// Calculate the total Bitcoin reward for a given number of mined blocks.
pub fn calculate_total_reward(blocks_mined: u64) -> f64 {
    // TODO: Multiply blocks_mined by MINING_REWARD and return result
    blocks_mined as f64 * MINING_REWARD
}

/// Return true if the transaction fee is between 0.00001 and 0.01 BTC.
pub fn is_valid_tx_fee(fee: f64) -> bool {
    // TODO: Check if fee is between 0.00001 and 0.01 BTC (inclusive)
    (0.00001..=0.01).contains(&fee)
}

/// Return true if the wallet balance is greater than 50.0 BTC.
pub fn is_large_balance(balance: f64) -> bool {
    // TODO: Compare balance to 50.0 and return result
    balance > 50.0
}

/// Return the priority of a transaction ("high", "medium", "low") based on fee rate.
pub fn tx_priority(size_bytes: u64, fee_btc: f64) -> &'static str {
    // TODO: Calculate fee rate (fee_btc / size_bytes) and use if/else if/else
    // High: > 0.00005, Medium: > 0.00001, otherwise Low
    let fee_rate = fee_btc / size_bytes as f64;
    if fee_rate > 0.00005 {
        "high"
    } else if fee_rate > 0.00001 {
        "medium"
    } else {
        "low"
    }
}

/// Return true if the network string equals "mainnet" (case-insensitive).
pub fn is_mainnet(network: &str) -> bool {
    // TODO: Convert network to lowercase and compare with "mainnet"
    network.to_lowercase() == "mainnet"
}

/// Return true if value is in the inclusive range 100..=200.
pub fn is_in_range(value: i64) -> bool {
    // TODO: Check if 100 <= value <= 200
    (100..=200).contains(&value)
}

/// Return true if both references point to the exact same object in memory.
pub fn is_same_wallet<T>(wallet1: &T, wallet2: &T) -> bool {
    // TODO: Use std::ptr::eq to compare reference identity
    std::ptr::eq(wallet1, wallet2)
}

/// Normalize a Bitcoin address by trimming whitespace and lowercasing.
pub fn normalize_address(address: &str) -> String {
    // TODO: Trim leading/trailing whitespace and convert to lowercase
    address.trim().to_lowercase()
}

/// Append a new UTXO to the list and return the updated list.
pub fn add_utxo(mut utxos: Vec<Utxo>, new_utxo: Utxo) -> Vec<Utxo> {
    // TODO: Push new_utxo into utxos and return it
    utxos.push(new_utxo);
    utxos
}

/// Find the first transaction with a fee greater than 0.005 BTC.
pub fn find_high_fee(fee_list: &[f64]) -> Option<(usize, f64)> {
    // TODO: Iterate with enumerate and return the first (index, fee) where fee > 0.005
    fee_list
        .iter()
        .enumerate()
        .find(|(_, &fee)| fee > 0.005)
        .map(|(i, &fee)| (i, fee))
}

/// Return basic wallet details as a tuple of (name, balance).
pub fn get_wallet_details() -> (String, f64) {
    // TODO: Return a tuple with wallet name and balance
    ("satoshi_wallet".to_string(), 50.0)
}

/// Get the status of a transaction from the mempool or "not found".
pub fn get_tx_status(tx_pool: &HashMap<String, String>, txid: &str) -> String {
    // TODO: Look up txid in tx_pool, returning the status or "not found"
    tx_pool
        .get(txid)
        .cloned()
        .unwrap_or_else(|| "not found".to_string())
}

/// Destructure wallet_info and format a status string.
pub fn unpack_wallet_info(wallet_info: (String, f64)) -> String {
    // TODO: Destructure the tuple into (name, balance) and format the result
    // Expected format: "Wallet <name> has balance: <balance> BTC"
    let (name, balance) = wallet_info;
    format!("Wallet {} has balance: {} BTC", name, balance as u64)
}

/// Convert BTC to satoshis (1 BTC = 100,000,000 sats).
pub fn calculate_sats(btc: f64) -> u64 {
    // TODO: Multiply btc by BTC_TO_SATS and return as u64
    (btc * BTC_TO_SATS as f64) as u64
}

/// Generate a mock Bitcoin address of length 32 with the given prefix.
pub fn generate_address(prefix: &str) -> String {
    // TODO: Build a random suffix of (32 - prefix.len()) chars from [a-z0-9]
    // TODO: Concatenate prefix + suffix and return
    use rand::Rng;
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let suffix_len = 32 - prefix.len();
    let mut rng = rand::thread_rng();
    let suffix: String = (0..suffix_len)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();
    format!("{}{}", prefix, suffix)
}

/// Validate a Bitcoin block height. Returns (is_valid, message).
pub fn validate_block_height(height: i64) -> (bool, String) {
    // TODO: Check that height is not negative
    // TODO: Check that height is within a realistic range (<= 1_000_000)
    // TODO: Return (true, "Valid block height") otherwise
    if height < 0 {
        (false, "Block height cannot be negative".to_string())
    } else if height > 1_000_000 {
        (false, "Block height is unrealistic".to_string())
    } else {
        (true, "Valid block height".to_string())
    }
}

/// Compute the block reward (in sats) for each block height based on the halving schedule.
pub fn halving_schedule(blocks: &[u64]) -> HashMap<u64, u64> {
    // TODO: Base reward is 50 * 100_000_000 sats; halving interval is 210_000 blocks
    // TODO: For each block: halvings = block / 210_000; reward = base >> halvings
    // TODO: Insert (block, reward) into the result HashMap
    let base_reward: u64 = 50 * 100_000_000;
    let halving_interval: u64 = 210_000;
    let mut result = HashMap::new();

    for &block in blocks {
        let halvings = block / halving_interval;
        let reward = base_reward >> halvings;
        result.insert(block, reward);
    }

    result
}

/// Find the UTXO with the smallest value that meets or exceeds target.
pub fn find_utxo_with_min_value(utxos: &[Utxo], target: u64) -> Option<Utxo> {
    // TODO: Filter UTXOs to those with value >= target
    // TODO: Return the one with the smallest value, or None if none qualify
    utxos
        .iter()
        .filter(|utxo| utxo.value >= target)
        .min_by_key(|utxo| utxo.value)
        .cloned()
}

/// Create a UTXO map from txid, vout, and arbitrary extra string fields.
pub fn create_utxo(
    txid: &str,
    vout: u32,
    extra: HashMap<String, String>,
) -> HashMap<String, String> {
    // TODO: Build a base map with "txid" and "vout" (as string)
    // TODO: Merge extra into the base map and return
    let mut result = HashMap::new();
    result.insert("txid".to_string(), txid.to_string());
    result.insert("vout".to_string(), vout.to_string());

    for (key, value) in extra {
        result.insert(key, value);
    }

    result
}

// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let version_hex = &raw_tx_hex[0..8];
    let bytes = hex::decode(version_hex).map_err(|_| "Hex decode error".to_string())?;

    let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    Ok(version)
}
