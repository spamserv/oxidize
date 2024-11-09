pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


pub mod blockchain {
    use std::time::SystemTime;
    use sha256::digest;

    struct Blockchain {
        blocks: Vec<Block>
    }

    struct Block {
       header: BlockHeader,
       body: BlockBody
    }

    struct BlockHeader {
        timestamp: SystemTime,
        previous_hash: String,
        current_hash: String,
        nonce: String,
        difficulty: u8,
    }

    struct BlockBody {
        transactions: Vec<BlockTransaction>
    }

    struct BlockTransaction {
        inputs: String,
        outputs: String,
        metadata: TransactionMetadata,
    }

    struct TransactionMetadata {
        sender: String,
        receiver: String,
        value: String
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
