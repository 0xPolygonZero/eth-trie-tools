use std::path::PathBuf;

use clap::Parser;
use common::{read_input_from_file, TRIE_PATH_DOC_STR};
use mpt_trie::{
    debug_tools::query::{get_path_from_query, DebugQueryParamsBuilder},
    nibbles::Nibbles,
};

const QUERY_CONFIG_STR: &str = "Query Config";

/// Displays a "trace" of a key query for a given key, where the trace contains the nodes encountered.
#[derive(Debug, Parser)]
#[command(after_help = TRIE_PATH_DOC_STR)]
struct ProgArgs {
    /// Path to the trie to query against.
    trie_path: PathBuf,

    /// The key to query in hex format (with or without the "0x").
    key: Nibbles,

    /// Config for the query output.
    #[command(flatten)]
    config: QueryConfig,
}

#[derive(Parser, Debug)]
struct QueryConfig {
    /// Include the key piece (if the node contains a piece of the key) for each node.
    #[arg(short = 'k', long, default_value_t = true, help_heading = QUERY_CONFIG_STR)]
    include_key_piece_per_node: bool,

    /// Include the node types in the trace.
    #[arg(short = 't', long, default_value_t = true, help_heading = QUERY_CONFIG_STR)]
    include_node_type: bool,

    /// Include additional info that is specific to a given node type.
    #[arg(short = 's', long, default_value_t = false, help_heading = QUERY_CONFIG_STR)]
    include_node_specific_values: bool,
}

impl From<QueryConfig> for DebugQueryParamsBuilder {
    fn from(v: QueryConfig) -> Self {
        DebugQueryParamsBuilder::default()
            .print_key_pieces(v.include_key_piece_per_node)
            .print_node_type(v.include_node_type)
            .print_node_specific_values(v.include_node_specific_values)
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let p_args = ProgArgs::parse();
    let trie = read_input_from_file(&p_args.trie_path);

    let query = DebugQueryParamsBuilder::from(p_args.config).build(p_args.key);
    let res = get_path_from_query(&trie, query);

    println!("{}", res);

    Ok(())
}
