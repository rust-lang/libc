// Ensure that our crate is able to handle definitions spread across many files

mod bar;
mod foo;

use bar::*;
use foo::*;
