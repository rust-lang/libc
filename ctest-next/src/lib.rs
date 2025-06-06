#![allow(dead_code)]

mod cargo_expand;
mod converter;
mod ir;
mod skip;
mod symbol_table;

// TODO: Implement proper error types instead of Box<dyn Error>.
// TODO: Add documentation.
