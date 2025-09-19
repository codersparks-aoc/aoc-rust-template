use crate::{AOCError, AOCSolution};

const PART_NUM:i8 = 2;
pub fn run(solution: &mut AOCSolution) {
    solution.part2 = Some(Err(AOCError::PartError(PART_NUM, format!("Part {PART_NUM} not yet implemented"))))
}