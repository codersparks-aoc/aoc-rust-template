use tracing::{Level, debug};
use clap::{Parser, ValueEnum};
use aoc_{{ project-name  | snake_case }}::{part_1, part_2, AOCResult, AOCSolution};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
enum Part {
    /// Run and output only part 1 of the solution
    Part1,
    /// Run and output only part 2 of the solution
    Part2,
    /// Run all parts of the solution
    #[default]
    All,
}

/// A command line runner to test each part of the solution for {{ project-name }}
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Which part to run
    #[arg(value_enum, default_value_t=Part::All)]
    part: Part,
    /// The level of verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,
}

fn main() -> AOCResult<()> {

    let args = CliArgs::parse();

    let log_level = match args.verbosity {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        _ => Level::DEBUG,
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();
    debug!("args: {:?}", args);

    let mut solution = AOCSolution::new();

    if args.part == Part::Part1 || args.part == Part::All {
        debug!("Running part 1");
        part_1::run(&mut solution);
    }

    if args.part == Part::Part2 || args.part == Part::All {
        debug!("Running part 2");
        part_2::run(&mut solution);
    }

    println!("{}", solution);

    Ok(())
}
