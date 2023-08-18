#![no_main]

use libfuzzer_sys::fuzz_target;

use pineapple_json::parser::json_value;

fuzz_target!(|data: &str| {
    let _json = json_value(data);
});
