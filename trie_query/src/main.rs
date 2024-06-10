use std::path::PathBuf;

use clap::Parser;
use common::{read_input_from_file, TRIE_PATH_DOC_STR};
use mpt_trie::{
    debug_tools::query::{get_path_from_query, DebugQueryParamsBuilder},
    nibbles::Nibbles,
};

/// Displays a "trace" of a key query for a given key, where the trace contains the nodes encountered.
#[derive(Debug, Parser)]
#[command(after_help = TRIE_PATH_DOC_STR)]
struct ProgArgs {
    /// Path to the trie to query against.
    trie_path: PathBuf,

    /// The key to query in hex format (with or without the "0x").
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
