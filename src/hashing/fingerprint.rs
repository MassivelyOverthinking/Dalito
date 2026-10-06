//====================================================================================
// IMPORTS & MODULES
//====================================================================================

use crate::utility::types::HashContent;

//====================================================================================
// DATASET FINGERPRINT
//====================================================================================

pub struct DatasetFingerprint {
    schema_hash: HashContent,
    content_hash: HashContent,
    metadata_hash: HashContent,
    memory_size: f32,
    row_counts: u32,
}