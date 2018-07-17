extern crate ewasm_api;
extern crate blake2;
use blake2::{Blake2b, Digest};

#[no_mangle]
pub extern fn main() {
  let length = ewasm_api::calldata_size();
  let data = ewasm_api::calldata_copy(0, length);

  // charge a base fee plus a word fee for every 256-bit word
  let base_fee = 60;
  let word_fee = 12;
  let total_cost = base_fee + ((length + 31) / 32) * word_fee;

  ewasm_api::consume_gas(total_cost as u64);

  let mut hasher = Sha256::default();
  hasher.input(&data);
  let hash = hasher.result();

  ewasm_api::finish_data(hash.to_vec());
}
