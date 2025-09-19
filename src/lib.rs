use std::fmt::{Display, Formatter};
use tracing::debug;
use crate::errors::AOCError;

pub mod part_1;
pub mod part_2;
pub mod errors;

pub const PROJECT_NAME: &str = "{{ project-name }}";

pub type AOCResult<T> = Result<T, AOCError>;


pub struct AOCSolution {
    pub part1: Option<AOCResult<String>>,
    pub part2: Option<AOCResult<String>>,
}

impl Default for AOCSolution {
    fn default() -> Self {
        Self {
            part1: None,
            part2: None
        }
    }
}

impl Display for AOCSolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {


        match &self.part1 {
            None => { debug!(project=PROJECT_NAME, part=2, "No solution present"); }
            Some(result) => {
                writeln!(f, "{PROJECT_NAME}, Part 1:")?;
                match result {
                    Ok(s) => writeln!(f, "{}", s)?,
                    Err(e) => writeln!(f, "ERROR: {}", e)?,
                }
            }
        }

        match &self.part2 {
            None => { debug!(project=PROJECT_NAME, part=2, "No solution present"); }
            Some(result) => {
                writeln!(f, "{PROJECT_NAME}, Part 2:")?;
                match result {
                    Ok(s) => writeln!(f, "{}", s)?,
                    Err(e) => writeln!(f, "ERROR: {}", e)?,
                }
            }
        }

        Ok(())
    }
}

impl AOCSolution {
    pub fn new() -> Self {
        Self::default()
    }
}