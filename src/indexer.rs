use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::{collections::HashMap, str::FromStr};
use yellowstone_grpc_proto::prelude::subscribe_update::UpdateOneof;
use {
    crate::constants::YOUR_DISCRIMINATOR, yellowstone_grpc_client::GeyserGrpcClient,
    yellowstone_grpc_proto::prelude::*,
};

#[derive(BorshDeserialize)]
struct MyInstruction {
    discriminant: u8,
    // your instruction fields
}

async fn process_transaction(tx: SubscribeUpdateTransactionInfo) -> anyhow::Result<()> {
    let program_id = Pubkey::from_str("Your_Program_ID")?;

    for ix in tx.transaction.unwrap().message.unwrap().instructions {
        if ix.program_id_index == program_id {
            let instruction_data = ix.data;

            // Check discriminator (first byte in this example)
            if instruction_data[0] == YOUR_DISCRIMINATOR {
                let instruction = MyInstruction::try_from_slice(&instruction_data)?;

                // Process instruction
                // Store in database, etc.
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = GeyserGrpcClient::connect("your_endpoint").await?;

    let mut transactions = HashMap::new();
    transactions.insert(
        "client".to_string(),
        SubscribeRequestFilterTransactions {
            account_include: vec!["Your_Program_ID".to_string()],
            ..Default::default()
        },
    );

    let request = SubscribeRequest {
        transactions,
        commitment: Some(CommitmentLevel::Finalized as i32),
        ..Default::default()
    };

    let (_tx, mut rx) = client.subscribe_with_request(Some(request)).await?;

    while let Some(message) = rx.next().await {
        match message {
            Ok(msg) => {
                if let Some(UpdateOneof::Transaction(tx)) = msg.update_oneof {
                    // Mirar tipos de tx
                    if let Err(e) = process_transaction(tx).await {
                        eprintln!("Error processing transaction: {}", e);
                    }
                }
            }
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }

    Ok(())
}
