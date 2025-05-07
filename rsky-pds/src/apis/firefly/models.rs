use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy)]
pub struct U128Stringified(pub u128);

impl Serialize for U128Stringified {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for U128Stringified {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u128>()
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}

impl From<u128> for U128Stringified {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

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
    pub amount: U128Stringified,
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
    pub amount: U128Stringified,
    pub post: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transfer {
    pub id: String,
    pub direction: Direction,
    pub date: u64,
    pub amount: U128Stringified,
    pub to_address: String,
    pub cost: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WalletStateAndHistory {
    pub address: String,
    pub balance: U128Stringified,
    pub requests: Vec<Request>,
    pub exchanges: Vec<Exchange>,
    pub boosts: Vec<Boost>,
    pub transfers: Vec<Transfer>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransferRequest {
    pub amount: U128Stringified,
    pub description: String,
    pub user_handle: String,
}
