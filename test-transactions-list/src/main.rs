use dotenv::dotenv;
use firefly_api::write_node_client::BlocksClient;
use rsky_pds::apis::firefly::providers::get_firefly_provider;
use tokio;

fn get_provider() {}

#[tokio::main]
async fn main() {
    let block_node_url = "http://localhost:40403".to_string(); // Replace with your actual block node URL
    let client = BlocksClient::new(block_node_url);

    match client.get_transactions().await {
        Ok(transactions) => {
            for (id, timestamp, csv_values) in transactions {
                println!("Timestamp: {}, id: {}", timestamp, id);
                println!("CSV Values: {:?}", csv_values);
                println!("-----------------------------");
            }
        }
        Err(e) => {
            eprintln!("Error fetching raw transaction list: {}", e);
        }
    }

    dotenv().ok();

    let firefly_provider = get_firefly_provider().unwrap();

    let firefly = firefly_provider.firefly();

    match firefly.get_transactions().await {
        Ok(transactions) => {
            for transaction in transactions {
                println!("Transaction: {:?}", transaction);
            }
        }
        Err(e) => {
            eprintln!("Error fetching transaction list: {}", e);
        }
    }
}
