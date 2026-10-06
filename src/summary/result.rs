//====================================================================================
// IMPORTS & MODULES
//====================================================================================

use crate::{hashing::fingerprint::DatasetFingerprint, utility::types::HashContent};

//====================================================================================
// DATA SUMMARY: Results
//====================================================================================

pub struct InputResult<'a> {
    fingerprint: DatasetFingerprint,
    columns: Vec<ColumnEntry<'a>>,
    parameters: Vec<ParameterEntry<'a>>,
}

pub struct OutputResult {
    fingerprint: DatasetFingerprint,
}

pub struct TransformationResult<'a> {
    name: &'a str,
    version: &'a str,
    parameters: Vec<ParameterEntry<'a>>,
}

//====================================================================================
// DATA SUMMARY: Data Entries
//====================================================================================

struct ColumnEntry<'a> {
    name: &'a str,
    dtype: &'a str,
}

struct ParameterEntry<'a> {
    name: &'a str,
    dtype: &'a str,
    value: &'a str,
}