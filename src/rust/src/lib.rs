use extendr_api::prelude::*;

/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// This function adds two numbers.
/// @export
#[extendr]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// This function subtracts two numbers.
/// @export
#[extendr]
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// This function multiplies two numbers.
/// @export
#[extendr]
#[deprecated(
    since = "0.1",
    note = "We can't multiply, so we've deprecated this function."
)]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// This function divides two numbers.
/// @export
#[extendr]
fn divide(a: i32, b: i32) -> i32 {
    a / b
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rpackage;
    fn hello_world;
    fn add;
    fn subtract;
    fn multiply;
    fn divide;
}
