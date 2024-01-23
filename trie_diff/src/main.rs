use std::path::PathBuf;

use clap::Parser;
use common::read_input_from_file;

use eth_trie_utils::{debug_tools::diff::create_diff_between_tries, partial_trie::PartialTrie};

#[derive(Debug, Parser)]
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
