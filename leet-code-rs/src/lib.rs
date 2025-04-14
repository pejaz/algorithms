// src/lib.rs

const CURRENT: &str = "sdfsdfsd.rs"; // trigger rust-analyser recheck

pub mod solutions {
    automod::dir!("src/solutions");
}
mod sort;
