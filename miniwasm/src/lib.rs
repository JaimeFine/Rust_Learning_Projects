// Import the necessary attribute from wasm-bindgen:
use wasm_bindgen::prelude::*;

/// A simple function exposed to JS.
/// Takes a name as a string slice and returns a customized greeting string.
/// The #[wasm_bindgen] attribute is crucial for making this function accessible.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    // Using Rust's standard formatting macro to create the string.
    let greeting = format!("Hello, {}! This message came from Rust Wasm!!!", name);

    // Log the action in the browser console (Rust side)
    web_sys::console::log_1(&greeting.as_str().into());

    greeting
}




/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
