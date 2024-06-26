use std::{fs, path::Path};

use mpt_trie::partial_trie::HashedPartialTrie;
use trace_decoder::{
    compact::compact_prestate_processing::process_compact_prestate_debug,
    trace_protocol::TrieCompact,
};

pub const TRIE_PATH_DOC_STR: &str = r#"
Each path must point to a file that is either:
- (`*.compact`) Compact encoding (hex string) of an MPT trie (spec: https://gist.github.com/mandrigin/ff7eccf30d0ef9c572bafcb0ab665cff).
- (`*.json`) A serialized `HashedPartialTrie`.
"#;

pub fn read_input_from_file(p: &Path) -> HashedPartialTrie {
    match p
        .extension()
        .map(|s| s.to_str().unwrap_or_default())
        .unwrap_or_default()
    {
        // For now, if we ever get compact, we're going to just use the state trie.
        "compact" => {
            let out = process_compact_prestate_debug(read_compact_from_file(p)).unwrap();
            out.witness_out.state_trie
        }
        "json" => read_json_trie_from_file(p),
        _ => panic!(
            "Input file must either be a `*.json` or a `*.compact` (got a {:?}).",
            p
        ),
    }
}

pub fn read_compact_from_file(f_name: &Path) -> TrieCompact {
    let buf = fs::read_to_string(f_name).unwrap();
    let payload = buf.trim();
    let compact_bytes = hex::decode(payload).unwrap();

    TrieCompact(compact_bytes)
}

pub fn read_json_trie_from_file(f_name: &Path) -> HashedPartialTrie {
    let buf = fs::read_to_string(f_name).unwrap();
    let payload = buf.trim();
    serde_json::from_str(payload).unwrap()
}
