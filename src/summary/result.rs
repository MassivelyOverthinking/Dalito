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
    row_count: u32,
    digest: HashContent,
    memory_size: f32,
    null_count: u8,
    parameters: Vec<ParameterEntry<'a>>,
}

pub struct OutputResult {
    fingerprint: DatasetFingerprint,
}

pub struct TransformationResult {
    fingerprint: DatasetFingerprint,
}

//====================================================================================
// DATA SUMMARY: Column Entry
//====================================================================================

struct ColumnEntry<'a> {
    name: &'a str,
    dtype: &'a str,
}

//====================================================================================
// DATA SUMMARY: Parameter Entry
//====================================================================================

struct ParameterEntry<'a> {
    name: &'a str,
    dtype: &'a str,
    value: &'a str,
}