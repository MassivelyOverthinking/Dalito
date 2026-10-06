//====================================================================================
// IMPORTS & MODULES
//====================================================================================

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::{hashing::fingerprint::DatasetFingerprint};

//====================================================================================
// DATA SUMMARY: Results
//====================================================================================

#[derive(Debug, Deserialize, Serialize)]
pub struct InputResult<'a> {
    timestamp: SystemTime,
    fingerprint: DatasetFingerprint,
    columns: Vec<ColumnEntry<'a>>,
    parameters: Vec<ParameterEntry<'a>>,
}

impl<'a> InputResult<'a> {
    pub fn get_timestamp(&self) -> &SystemTime {
        &self.timestamp
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputResult {
    timestamp: SystemTime,
    fingerprint: DatasetFingerprint,
}

impl OutputResult {
    pub fn get_timestamp(&self) -> &SystemTime {
        &self.timestamp
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransformationResult<'a> {
    name: &'a str,
    version: &'a str,
    parameters: Vec<ParameterEntry<'a>>,
}

//====================================================================================
// DATA SUMMARY: Data Entries
//====================================================================================

#[derive(Debug, Deserialize, Serialize)]
struct ColumnEntry<'a> {
    name: &'a str,
    dtype: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
struct ParameterEntry<'a> {
    name: &'a str,
    dtype: &'a str,
    value: &'a str,
}