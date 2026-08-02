#![allow(unused)]
fn parser(x: &str) -> Vec<&str> {
     x.split_whitespace().collect()
}

fn detect 