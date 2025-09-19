use crate::{AOCError, AOCSolution};

const PART_NUM:i8 = 1;
pub fn run(solution: &mut AOCSolution) {
    solution.part1 = Some(Err(AOCError::PartError(PART_NUM, format!("Part {PART_NUM} not yet implemented"))))
}