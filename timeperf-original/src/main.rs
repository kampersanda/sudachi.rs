use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use sudachi::analysis::stateful_tokenizer::StatefulTokenizer;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::subset::InfoSubset;
use sudachi::prelude::*;

use clap::Parser;

mod timer;
use timer::Timer;

const RUNS: usize = 10;
const TRIALS: usize = 10;

#[derive(Parser, Debug)]
#[clap(name = "main", about = "A program.")]
struct Args {
    #[clap(short = 'd', long)]
    dict_filename: String,

    #[clap(short = 's', long)]
    sentence_filename: String,

    #[clap(short = 'r', long)]
    resources_filename: Option<String>,
}

fn main() {
    let args = Args::parse();

    let config = Config::new(
        None,
        args.resources_filename.map(|s| PathBuf::from(s)),
        Some(PathBuf::from(&args.dict_filename)),
    )
    .unwrap();
    let lines = load_file(&args.sentence_filename);

    let dict = JapaneseDictionary::from_cfg(&config).unwrap();
    let mut tokenizer = StatefulTokenizer::new(&dict, Mode::C);
    tokenizer.set_subset(InfoSubset::empty());
    let mut morphemes = MorphemeList::empty(&dict);

    let mut measure = |t: &mut Timer| {
        let mut n_words = 0;
        for _ in 0..RUNS {
            t.start();
            for line in &lines {
                tokenizer.reset().push_str(line);
                tokenizer.do_tokenize().unwrap();
                morphemes.collect_results(&mut tokenizer).unwrap();
                n_words += morphemes.len();
            }
            t.stop();
        }
        dbg!(n_words);
    };

    let mut t = Timer::new();

    // Warmup
    t.reset();
    measure(&mut t);
    println!("Warmup: {}", t.average());

    let (mut min, mut max, mut avg) = (0.0, 0.0, 0.0);

    for _ in 0..TRIALS {
        t.reset();
        measure(&mut t);
        t.discard_min();
        t.discard_max();
        min += t.min();
        avg += t.average();
        max += t.max();
    }

    min = min / TRIALS as f64;
    avg = avg / TRIALS as f64;
    max = max / TRIALS as f64;

    println!("Number_of_sentences: {}", lines.len());
    println!(
        "Elapsed_seconds_to_tokenize_all_sentences: [{},{},{}]",
        min, avg, max
    );
}

fn load_file<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    buf.lines().map(|line| line.unwrap()).collect()
}
