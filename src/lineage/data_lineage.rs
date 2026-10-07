//====================================================================================
// IMPORTS & MODULES
//====================================================================================

use std::time::{Duration, SystemTime};
use std::fmt::Display;

use uuid::Uuid;

use crate::summary::result::{InputResult, OutputResult, TransformationResult, ActiveTransformation};
use crate::error::DalitoError;

//====================================================================================
// DATA LINEAGE TOOL: Main
//====================================================================================


pub struct DataLineage<'a> {
    name: String,
    entries: Vec<LineageEntry<'a>>,
    active: Option<ActiveTransformation<'a>>,
    count: u8,
}

impl<'a> DataLineage<'a> {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            entries: Vec::new(),
            active: None,
            count: 0,
        }
    }

    pub fn initiate(&mut self, input: InputResult<'a>, transformation: Option<TransformationResult<'a>>) -> Result<(), DalitoError> {
        if self.active.is_none() {
            return Err(DalitoError::interrupted_transformation(
                "Active transformation in progress cannot be terminated - Try '.finish()' method"
            ));
        };

        self.active = Some(ActiveTransformation::new(
            input, 
            transformation
        ));

        Ok(())
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
        output.duration_since(*input).unwrap()
    }
}

impl<'a> Display for LineageEntry<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Lineage Entry")?;
        writeln!(f, "-----------------------------")?;
        writeln!(f, "ID:                {}", self.entry_id)?;
        writeln!(f, "Name:              {}", self.name.unwrap_or("None"))?;
        writeln!(f, "Duration:          {:?}", self.durations_ms)?;
        writeln!(f, "Input:             {}", self.input)?;
        writeln!(f, "Output:            {}", self.output)?;
        writeln!(
            f, 
            "Transformation:    {}", 
            self.transformation
                .as_ref()
                .map(|transform| transform.to_string())
                .unwrap_or_else(|| "None".to_string())
            )?;

        Ok(())
    }
}