extern crate ewasm_api;
extern crate ethereum_bn128;
extern crate parity_bytes as bytes;

use bytes::BytesRef;

#[no_mangle]
pub extern fn main() {
  let length = ewasm_api::calldata_size();
  let input = ewasm_api::calldata_copy(0, length);

  // NOTE: this validation will also be done by bn128_pairing

  if length % 192 != 0 {
    ewasm_api::revert();
  }

  // charge a base fee plus a word fee for every element
  let base_fee = 100000;
  let element_fee = 80000;
  let total_cost = base_fee + (length / 192) * element_fee;

  ewasm_api::consume_gas(total_cost as u64);

  let mut output = vec![0u8; 32];
  match ethereum_bn128::bn128_pairing(&input[..], &mut BytesRef::Fixed(&mut output[..])) {
    Ok(_) => {
      ewasm_api::finish_data(&output.to_vec());
    }
    Err(_) => {
      ewasm_api::revert();
    }
  }
}
