use blockchainlib::*;

fn main() {
    /* Define difficulty */
    let difficulty = 0x0000_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

    /* Create a new blockchain */
    let mut blockchain = Blockchain::new();

    /* Create a genesis block with transactions */
    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 25,
                },
            ],
        }],
        difficulty,
    );

    /* Mine that genesis block */
    genesis_block.mine();
    println!("Mined genesis block: {:?}", &genesis_block);

    let last_hash = genesis_block.hash.clone();

    /* Add that genesis block to the blockchain */
    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    /* Create a new block with more transactions */
    let mut new_block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Charlie".to_owned(),
                    value: 5,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 30,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 20,
                    },
                ],
            },
        ],
        difficulty,
    );

    /* Mine that new block */
    new_block.mine();
    println!("Mined a new block: {:?}", &new_block);

    /* Add that new block to the blockchain */
    blockchain
        .update_with_block(new_block)
        .expect("Failed to add new block");
}
