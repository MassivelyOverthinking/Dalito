//====================================================================================
// IMPORTS & MODULES
//====================================================================================

use std::time::SystemTime;

use uuid::Uuid;

use crate::summary::result::{InputResult, OutputResult, TransformationResult};

//====================================================================================
// DATA LINEAGE TOOL: Main
//====================================================================================


pub struct DataLineage<'a> {
    entries: Vec<LineageEntry<'a>>,
}

impl DataLineage {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn initiate(&mut self) {
        todo!()
    }

    pub fn finish(&mut self) {
        todo!()
    }
}

//====================================================================================
// DATA LINEAGE TOOL: Lineage Entry
//====================================================================================

pub struct LineageEntry<'a> {
    entry_id: Uuid,
    timestamp: SystemTime,
    input: InputResult<'a>,
    output: OutputResult,
    transformation: Option<TransformationResult<'a>>,
}

impl LineageEntry<'a> {
    pub fn new(input: InputResult, output: OutputResult, transformation: Option<TransformationResult>) -> Self {
        Self { 
            entry_id: Uuid::new_v4(), 
            timestamp: SystemTime::now(), 
            input: input, 
            output: output, 
            transformation: transformation, 
        }
    }
}