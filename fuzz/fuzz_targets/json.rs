#![no_main]

use libfuzzer_sys::fuzz_target;

use json_nom::parser::json_value;

fuzz_target!(|data: &str| {
    let _json = json_value(data);
});
