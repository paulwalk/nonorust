use nonorust::puzzle_factory::PuzzleConfig;
use clap::Parser;
use nonorust::cli::Args;
use nonorust::logging::configure_flexi_logger;

// use flexi_logger::Logger;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    configure_flexi_logger(args.debug).expect("Failed to configure logger - quitting");
    
    let puzzle_file_path = args.path;
    let max_iterations = args.max_iterations;

    let puzzle_factory_result = PuzzleConfig::build(puzzle_file_path.parse().unwrap());
    let mut puzzle = match puzzle_factory_result {
        Ok(puzzle) => puzzle,
        Err(e) => {
            eprintln!("Error loading puzzle: {e}");
            return Ok(());
        }
    };

    log::info!(
        "Starting Nonogram solver with file: {puzzle_file_path} and max iterations set to: {max_iterations}");

    let (iterations_needed_to_solve,puzzle_solved) = puzzle.solve(max_iterations);
    puzzle.dump();
    if puzzle_solved {
        println!("Puzzle solved!");
    } else {
        println!("Puzzle NOT solved!");
    }
    println!("Iterations used: {iterations_needed_to_solve}");

    Ok(())
}
