use firefly_api::write_node_client::BlocksClient;
use tokio;

#[tokio::main]
async fn main() {
    let block_node_url = "http://localhost:40403".to_string(); // Replace with your actual block node URL
    let client = BlocksClient::new(block_node_url);

    match client.get_transactions().await {
        Ok(transactions) => {
            for (timestamp, csv_values) in transactions {
                println!("Timestamp: {}", timestamp);
                println!("CSV Values: {:?}", csv_values);
                println!("-----------------------------");
            }
        }
        Err(e) => {
            eprintln!("Error fetching test-transactions-list: {}", e);
        }
    }
}
