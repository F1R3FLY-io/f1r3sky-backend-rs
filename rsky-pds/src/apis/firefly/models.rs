use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy)]
pub struct Stringified<T>(pub T);

impl<T> Serialize for Stringified<T>
where
    T: ToString,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de, T> Deserialize<'de> for Stringified<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<T>().map(Self).map_err(serde::de::Error::custom)
    }
}

impl<T> From<T> for Stringified<T> {
    fn from(value: T) -> Self {
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
    pub amount: Stringified<u128>,
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
    pub amount: Stringified<u128>,
    pub post: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transfer {
    pub id: String,
    pub direction: Direction,
    pub date: u64,
    pub amount: Stringified<u128>,
    pub to_address: String,
    pub cost: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WalletStateAndHistory {
    pub address: String,
    pub balance: Stringified<u128>,
    pub requests: Vec<Request>,
    pub exchanges: Vec<Exchange>,
    pub boosts: Vec<Boost>,
    pub transfers: Vec<Transfer>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransferRequest {
    pub amount: Stringified<u128>,
    pub description: String,
    pub user_handle: String,
}
