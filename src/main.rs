use clap::Parser;
use std::{fs, time};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The letters of the spelling bee, center letter first, all in a string, e.g. 'abcdefg'
    #[clap(value_parser)]
    letters: String,
}

fn main() {
    let start_time = time::Instant::now();
    let cli = Cli::parse();
    let letters: Vec<char> = cli.letters.chars().collect();
    let letters: [char; 7] = letters.try_into().expect("Needed 7 letters");
    let center_letter = letters[0];
    println!("Parsing input: {:.3} us", start_time.elapsed().as_micros());
    println!("Using letters: {:?}", letters);

    let read_time = time::Instant::now();
    let file_path = "american_english_dictionary.txt";
    let file = fs::read_to_string(file_path).expect("Unable to read file");
    let words: Vec<_> = file
        .lines()
        .filter(|&w| w.chars().all(char::is_lowercase))
        .collect();
    println!(
        "Reading dictionary file: {:.3} us",
        read_time.elapsed().as_micros()
    );

    let filter_time = time::Instant::now();
    let mut words: Vec<_> = words
        .iter()
        .filter(|&w| !w.ends_with("'s"))
        // At least four letters
        .filter(|&w| w.len() >= 4)
        // Contains the center letter
        .filter(|&w| w.contains(center_letter))
        .filter(|&w| w.chars().all(|c| letters.contains(&c)))
        .collect();
    words.sort_unstable();
    words.dedup();

    println!("Filtering: {:.3} us", filter_time.elapsed().as_micros());

    println!("Found {} words\n", words.len());
    for &&w in &words {
        println!("{}", w);
    }

    println!("\nTotal time: {:.3} s", start_time.elapsed().as_secs_f32());
}
