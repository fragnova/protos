use parity_scale_codec::Encode;
use protos::traits::Trait;
use std::env;

fn main() {
  let json_file = env::args().skip(1).next();
  let json_file = match json_file {
    Some(file) => file,
    None => {
      println!("Usage: make_trait <json_file>");
      return;
    }
  };

  let json = std::fs::read_to_string(json_file).unwrap();

  println!("JSON: {}", json);

  let mut t: Trait = serde_json::from_str(&json).unwrap();

  // THIS IS the way we reprocess the trait declaration before sorting it on chain and hashing it
  t.records = t
    .records
    .into_iter()
    .map(|r| (r.name.to_lowercase(), r.types).into())
    .collect();
  t.records.dedup_by(|a, b| a.name == b.name);
  t.records.sort_by(|a, b| a.name.cmp(&b.name));

  let binary_trait = t.encode();
  println!("SCALE encoded trait: 0x{}", hex::encode(&binary_trait));
}
