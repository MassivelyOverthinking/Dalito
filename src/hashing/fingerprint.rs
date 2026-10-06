//====================================================================================
// IMPORTS & MODULES
//====================================================================================

use serde::{Deserialize, Serialize};

use crate::utility::types::HashContent;

//====================================================================================
// DATASET FINGERPRINT
//====================================================================================

#[derive(Debug, Deserialize, Serialize)]
pub struct DatasetFingerprint {
    schema_hash: HashContent,
    content_hash: HashContent,
    metadata_hash: HashContent,

    memory_size: u64,
    row_count: u64,
}