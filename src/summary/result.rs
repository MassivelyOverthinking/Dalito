//====================================================================================
// IMPORTS & MODULES
//====================================================================================

use std::time::SystemTime;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{hashing::fingerprint::DatasetFingerprint};

//====================================================================================
// DATA SUMMARY: Results
//====================================================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
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

impl<'a> Display for InputResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct OutputResult {
    finished_at: SystemTime,
    fingerprint: DatasetFingerprint,
}

impl OutputResult {
    pub fn get_finished_at(&self) -> &SystemTime {
        &self.finished_at
    }
}

impl Display for OutputResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransformationResult<'a> {
    name: &'a str,
    version: &'a str,
    parameters: Vec<ParameterEntry<'a>>,
}

impl<'a> Display for TransformationResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")
    }
}

#[derive(Debug, Clone)]
pub struct ActiveTransformation<'a> {
    name: &'a str, 
    input: InputResult<'a>,
    transformation: TransformationResult<'a>,
    started_at: SystemTime,
}

impl<'a> ActiveTransformation<'a> {
    pub fn new(name: &'a str, input: InputResult<'a>, transformation: TransformationResult<'a>) -> Self {
        Self {  
            name: name,
            input: input, 
            transformation: transformation, 
            started_at: SystemTime::now(), 
        }
    }

    pub fn get_name(&self) -> &'a str {
        &self.name
    }

    pub fn get_input(&self) -> &InputResult<'a> {
        &self.input
    }

    pub fn get_started_at(&self) -> &SystemTime {
        &self.started_at
    }

    pub fn get_transformation(&self) -> &TransformationResult<'a> {
        &self.transformation
    }
}

//====================================================================================
// DATA SUMMARY: Data Entries
//====================================================================================

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
struct ColumnEntry<'a> {
    name: &'a str,
    dtype: &'a str,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
struct ParameterEntry<'a> {
    name: &'a str,
    dtype: &'a str,
    value: &'a str,
}