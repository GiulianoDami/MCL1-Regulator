use std::env;
use std::process;

mod analyzer;
mod predictor;
mod interactive;

use analyzer::MCL1Analyzer;
use predictor::PathwayPredictor;
use interactive::InteractiveAnalyzer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [arguments]", args[0]);
        eprintln!("Commands:");
        eprintln!("  analyze <input_file> - Analyze MCL1 interactions");
        eprintln!("  predict -i <input_file> -o <output_file> - Predict metabolic pathways");
        eprintln!("  interactive - Launch interactive mode");
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
            let analyzer = MCL1Analyzer::new();
            analyzer.analyze_interactions(input_file);
        }
        "predict" => {
            let mut input_file = String::new();
            let mut output_file = String::new();
            
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "-i" => {
                        if i + 1 < args.len() {
                            input_file = args[i + 1].clone();
                            i += 2;
                        } else {
                            eprintln!("Missing argument for -i");
                            process::exit(1);
                        }
                    }
                    "-o" => {
                        if i + 1 < args.len() {
                            output_file = args[i + 1].clone();
                            i += 2;
                        } else {
                            eprintln!("Missing argument for -o");
                            process::exit(1);
                        }
                    }
                    _ => {
                        eprintln!("Unknown option: {}", args[i]);
                        process::exit(1);
                    }
                }
            }

            if input_file.is_empty() || output_file.is_empty() {
                eprintln!("Usage: {} predict -i <input_file> -o <output_file>", args[0]);
                process::exit(1);
            }

            let predictor = PathwayPredictor::new();
            predictor.predict_pathways(&input_file, &output_file);
        }
        "interactive" => {
            let interactive = InteractiveAnalyzer::new();
            interactive.run();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Use 'analyze', 'predict', or 'interactive'");
            process::exit(1);
        }
    }
}