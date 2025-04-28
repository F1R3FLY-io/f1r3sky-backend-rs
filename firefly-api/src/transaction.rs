use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Represents a transaction with timestamp, name, and arguments
#[derive(Debug, Serialize)]
pub struct Transaction {
    pub id: String,
    pub date_time: DateTime<Utc>,
    pub name: String,
    pub arguments: Vec<String>,
}

impl Transaction {
    /// Creates a new Transaction from a tuple of DateTime and arguments
    ///
    /// # Arguments
    /// * `data` - Tuple containing (DateTime<Utc>, Vec<String>)
    ///
    /// # Returns
    /// * `Result<Self>` - Result containing the Transaction or an error
    pub fn new(data: (String, DateTime<Utc>, Vec<String>)) -> Result<Self> {
        let (id, date_time, arguments) = data;

        if arguments.len() < 2 {
            return Err(anyhow!(
                "Operation signature requires at least 2 arguments, but got {}",
                arguments.len()
            ));
        }

        Ok(Self {
            id,
            date_time,
            name: arguments[1].clone(),
            arguments: arguments[2..].to_vec(),
        })
    }
}
