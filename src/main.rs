// main.rs

#[no_mangle] // Ensure function name is preserved in the compiled WASM
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Optional: a main function (useful if targeting emscripten)
fn main() {
    // This will print to the Emscripten JS console
    #[cfg(target_os = "emscripten")]
    {
        println!("Hello from Rust compiled with Emscripten!");
        println!("2 + 3 = {}", add(2, 3));
    }
}