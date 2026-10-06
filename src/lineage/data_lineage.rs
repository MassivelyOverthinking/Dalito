//====================================================================================
// IMPORTS & MODULES
//====================================================================================

use std::time::{Duration, SystemTime};

use uuid::Uuid;

use crate::summary::result::{InputResult, OutputResult, TransformationResult};

//====================================================================================
// DATA LINEAGE TOOL: Main
//====================================================================================


pub struct DataLineage<'a> {
    name: String,
    entries: Vec<LineageEntry<'a>>,
    count: u8,
}

impl<'a> DataLineage<'a> {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            entries: Vec::new(),
            count: 0,
        }
    }

    pub fn initiate(&mut self) {
        todo!()
    }

    pub fn finish(&mut self) {
        todo!()
    }

    pub fn history(&self) -> Result<Vec<LineageEntry<'a>>, ()> {
        todo!()
    }

    pub fn transformations(&self) -> Result<Vec<TransformationResult<'a>>, ()> {
        todo!()
    }

    pub fn display(&self) -> Result<(), ()> {
        todo!()
    }

    pub fn query(&self) -> Result<Option<Vec<LineageEntry<'a>>>, ()> {
        todo!()
    }

    pub fn to_json(&self) -> serde_json::Result<()> {
        todo!()
    }

    fn add_entry(&mut self, entry: LineageEntry<'a>) {
        self.entries.push(entry);
        self.count += 1;
    }
}

//====================================================================================
// DATA LINEAGE TOOL: Lineage Entry
//====================================================================================

#[derive(Debug)]
pub struct LineageEntry<'a> {
    entry_id: Uuid,
    name: Option<&'a str>,
    durations_ms: Duration,
    input: InputResult<'a>,
    output: OutputResult,
    transformation: Option<TransformationResult<'a>>,
}

impl<'a> LineageEntry<'a> {
    pub fn new(name: Option<&'a str>, input: InputResult<'a>, output: OutputResult, transformation: Option<TransformationResult<'a>>) -> Self {
        Self { 
            entry_id: Uuid::new_v4(),
            name: name,
            durations_ms: Self::calculate_duration(input.get_timestamp(), output.get_timestamp()), 
            input: input, 
            output: output, 
            transformation: transformation, 
        }
    }

    fn calculate_duration(input: &SystemTime, output: &SystemTime) -> Duration {
        let duration_ms = output.duration_since(*input).unwrap();
        return duration_ms;
    }
}