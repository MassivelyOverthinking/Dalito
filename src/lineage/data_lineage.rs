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

    pub fn initiate(&mut self, name: &'a str, input: InputResult<'a>, transformation: TransformationResult<'a>) -> Result<(), DalitoError> {
        if self.active.is_none() {
            return Err(DalitoError::interrupted_transformation(
                "Active transformation in progress cannot be terminated - Try '.finish()' method"
            ));
        };

        self.active = Some(ActiveTransformation::new(
            name, 
            input, 
            transformation
        ));

        Ok(())
    }

    pub fn finish(&mut self, output: OutputResult) -> Result<(), DalitoError> {
        let active = self.active.take().ok_or(
            DalitoError::no_active_transformation(
                "No active transformation record currently found"
            )
        )?;

        let entry = LineageEntry::new(
            *active.get_name(),
            *active.get_input(),
            output,
            *active.get_transformation(),
            *active.get_started_at(),
            *output.get_finished_at(),
        );

        self.add_entry(entry);

        Ok(())
    }

    pub fn history(&self) -> Result<Vec<LineageEntry<'a>>, ()> {
        Ok(self.entries.clone())
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

#[derive(Debug, Clone)]
pub struct LineageEntry<'a> {
    entry_id: Uuid,
    name: Option<&'a str>,
    started_at: SystemTime,
    finished_at: SystemTime,
    duration: Duration,
    input: InputResult<'a>,
    output: OutputResult,
    transformation: TransformationResult<'a>,
}

impl<'a> LineageEntry<'a> {
    pub fn new(
        name: Option<&'a str>, 
        input: InputResult<'a>,
        output: OutputResult, 
        transformation: TransformationResult<'a>,
        started_at: SystemTime,
        finished_at: SystemTime,
    ) -> Self {
        Self { 
            entry_id: Uuid::new_v4(),
            name: name,
            started_at: started_at,
            finished_at: finished_at,
            duration: Self::calculate_duration(&started_at, &finished_at), 
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
        writeln!(f, "Duration:          {:?}", self.duration)?;
        writeln!(f, "Input:             {}", self.input)?;
        writeln!(f, "Output:            {}", self.output)?;
        writeln!(f, "Transformation:    {}", self.transformation)?;

        Ok(())
    }
}