use std::path::PathBuf;

use clap::Parser;
use common::read_input_from_file;
use mpt_trie::{
    debug_tools::query::{get_path_from_query, DebugQueryParamsBuilder},
    nibbles::Nibbles,
};

#[derive(Debug, Parser)]
struct ProgArgs {
    trie_path: PathBuf,
    key: Nibbles,
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let p_args = ProgArgs::parse();
    let trie = read_input_from_file(&p_args.trie_path);

    let query = DebugQueryParamsBuilder::default()
        .print_node_specific_values(true)
        .build(p_args.key);
    let res = get_path_from_query(&trie, query);

    println!("{}", res);

    Ok(())
}
