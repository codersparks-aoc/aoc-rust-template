
use crate::AOCError;
use crate::AOCResult;

const PART_NUM:i8 = 1;
pub fn run() -> AOCResult<String> {
    Err(AOCError::PartError(PART_NUM, format!("Part {PART_NUM} not yet implemented")))
}