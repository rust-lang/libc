#![allow(dead_code)]

mod cargo_expand;
mod ir;
mod skip;
mod symbol_table;
mod translation;

// TODO: Implement proper error types instead of Box<dyn Error>.
// TODO: Add documentation.
