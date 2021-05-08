use std::fs;

use parser::parse_html;

pub mod parser;

fn main() {
    let data = fs::read_to_string("test/index.html").unwrap();
    parse_html(data);
}
