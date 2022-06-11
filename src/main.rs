use clap::Parser;
use reconnect::csv_api::{CsvApi, CsvParams};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'l', long)]
    left_csv_path: String,

    #[clap(short = 'r', long)]
    right_csv_path: String,
}

fn main() {
    let args = Args::parse();

    let params = CsvParams {
        left_csv_path: args.left_csv_path,
        right_csv_path: args.right_csv_path,
    };

    use std::time::Instant;
    let now = Instant::now();

    {
        let differences = CsvApi::compare_csv(params).unwrap();
        CsvApi::write_differences_to_file("diff.csv".to_string(), &differences).expect("Unable to write differences to file.");
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
