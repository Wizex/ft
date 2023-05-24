use clap::ValueEnum;
use serde::{Serialize, Deserialize};

/// File transferring type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
pub enum TransferType {
    /// Send a file to a server. 
    To,

    /// Download a file from a server.
    From
}