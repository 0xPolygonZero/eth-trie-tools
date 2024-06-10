use std::path::PathBuf;

use clap::Parser;
use common::{read_input_from_file, TRIE_PATH_DOC_STR};
use mpt_trie::debug_tools::diff::create_diff_between_tries;
use mpt_trie::partial_trie::PartialTrie;

#[derive(Debug, Parser)]
#[command(after_help = TRIE_PATH_DOC_STR)]
/// Attempts to find the lowest possible point where two given tries differ.
///
/// Takes in two paths to tries that we want to include in the diff.
struct ProgArgs {
    a_path: PathBuf,
    b_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let p_args = ProgArgs::parse();

    let a_trie = read_input_from_file(&p_args.a_path);
    let b_trie = read_input_from_file(&p_args.b_path);

    println!(
        "Trie hashes: a: {:?}, b: {:?}",
        a_trie.hash(),
        b_trie.hash()
    );

    let diff = create_diff_between_tries(&a_trie, &b_trie);

    println!("{}", diff);

    Ok(())
}
