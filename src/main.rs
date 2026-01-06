use std::env;
use std::process;

mod analyzer;
mod predictor;
mod interactive;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [arguments...]", args[0]);
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "analyze" => {
            if args.len() < 3 {
                eprintln!("Usage: {} analyze <input_file>", args[0]);
                process::exit(1);
            }
            let input_file = &args[2];
            let analyzer = analyzer::MCL1Analyzer::new();
            analyzer.analyze_interactions(input_file);
        }
        "predict" => {
            if args.len() < 3 {
                eprintln!("Usage: {} predict <input_file> <output_file>", args[0]);
                process::exit(1);
            }
            let input_file = &args[2];
            let output_file = &args[3];
            let predictor = predictor::PathwayPredictor::new();
            predictor.predict_pathways(input_file, output_file);
        }
        "interactive" => {
            interactive::launch_interactive_session();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Available commands: analyze, predict, interactive");
            process::exit(1);
        }
    }
}