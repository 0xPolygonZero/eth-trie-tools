use std::path::PathBuf;

use clap::Parser;
use common::{read_input_from_file, TRIE_PATH_DOC_STR};
use mpt_trie::debug_tools::stats::get_trie_stats_with_name;

/// Analyze one or two tries and output stats.
#[derive(Debug, Parser)]
#[command(after_help = TRIE_PATH_DOC_STR)]
struct ProgArgs {
    /// The primary trie to compare against.
    trie_path: PathBuf,

    /// If a second trie is provided, a comparison will be made between the two tries along with printout out their individual stats.
    other_trie_path: Option<PathBuf>,
}

fn main() {
    pretty_env_logger::init();

    let p_args = ProgArgs::parse();
    let trie = read_input_from_file(&p_args.trie_path);
    let trie_name = p_args
        .trie_path
        .file_name()
        .map(|os_str| os_str.to_string_lossy())
        .unwrap_or("Trie".into());

    let stats = get_trie_stats_with_name(&trie, trie_name.to_string());
    println!("{}", stats);

    if let Some(other_trie_path) = p_args.other_trie_path {
        // A second trie was passed in. Print its stats and also do a comparison.
        let other_trie = read_input_from_file(&other_trie_path);
        let other_trie_name = other_trie_path
            .file_name()
            .map(|os_str| os_str.to_string_lossy())
            .unwrap_or("Trie".into());
        let other_trie_stats = get_trie_stats_with_name(&other_trie, other_trie_name.into());

        println!("{}", other_trie_stats);

        let comparison = stats.compare(&other_trie_stats);

        println!("{}", comparison);
    }
}
