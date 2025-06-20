#![no_main]
#![warn(unused_imports)]
use libfuzzer_sys::fuzz_target;
use scroll_core::parser::parse_scroll;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_scroll(s);
    }
});
