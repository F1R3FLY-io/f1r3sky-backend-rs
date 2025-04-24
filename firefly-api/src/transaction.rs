use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};

/// Represents a transaction with timestamp, name, and arguments
#[derive(Debug)]
pub struct Transaction {
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
    pub fn new(data: (DateTime<Utc>, Vec<String>)) -> Result<Self> {
        let (date_time, mut arguments) = data;

        if arguments.len() < 2 {
            return Err(anyhow!("Insufficient arguments, expected at least 2"));
        }

        // Remove and discard the last argument
        arguments
            .pop()
            .ok_or_else(|| anyhow!("Failed to get last argument"))?;

        // Get the name from the last remaining argument
        let name = arguments
            .pop()
            .ok_or_else(|| anyhow!("Failed to get name argument"))?;

        Ok(Self {
            date_time,
            name,
            arguments,
        })
    }
}
