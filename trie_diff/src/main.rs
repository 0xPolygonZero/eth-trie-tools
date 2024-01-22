use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use protocol_decoder::{
    compact::compact_prestate_processing::process_compact_prestate_debug,
    trace_protocol::TrieCompact,
};

use eth_trie_utils::{
    debug_tools::create_diff_between_tries,
    partial_trie::{HashedPartialTrie, PartialTrie},
};

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

fn read_input_from_file(p: &Path) -> HashedPartialTrie {
    match p
        .extension()
        .map(|s| s.to_str().unwrap_or_default())
        .unwrap_or_default()
    {
        "compact" => {
            let out = process_compact_prestate_debug(read_compact_from_file(p)).unwrap();
            out.witness_out.tries.state
        }
        "json" => read_json_trie_from_file(p),
        _ => panic!(
            "Input file must either be a `*.json` or a `*.compact` (got a {:?}).",
            p
        ),
    }
}

fn read_compact_from_file(f_name: &Path) -> TrieCompact {
    let buf = fs::read_to_string(f_name).unwrap();
    let payload = buf.trim();
    let compact_bytes = hex::decode(payload).unwrap();

    TrieCompact(compact_bytes)
}

fn read_json_trie_from_file(f_name: &Path) -> HashedPartialTrie {
    let buf = fs::read_to_string(f_name).unwrap();
    let payload = buf.trim();
    serde_json::from_str(payload).unwrap()
}
