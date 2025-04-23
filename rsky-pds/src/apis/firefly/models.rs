#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RequestStatus {
    DONE,
    ONGOING,
    CANCELLED,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Request {
    pub id: String,
    pub date: u64,
    pub amount: u128,
    pub status: RequestStatus,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Exchange {}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    INCOMING,
    OUTGOING,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Boost {
    pub id: String,
    pub username: String,
    pub direction: Direction,
    pub date: u64,
    pub amount: u128,
    pub post: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transfer {
    pub id: String,
    pub direction: Direction,
    pub date: u64,
    pub amount: u128,
    pub to_address: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WalletStateAndHistory {
    pub address: String,
    pub balance: u128,
    pub requests: Vec<Request>,
    pub exchanges: Vec<Exchange>,
    pub boosts: Vec<Boost>,
    pub transfers: Vec<Transfer>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransferRequest {
    pub amount: u128,
    pub description: String,
    pub user_handle: String,
}
