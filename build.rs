fn main() {
    println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libcoastragui.so"); // Set the shared object name
    println!("cargo:rustc-link-arg=-Wl,--as-needed"); // Link only necessary libraries
    println!("cargo:rustc-link-arg=-Wl,--strip-all"); // Strip all symbols to minimize size

    println!("cargo:rustc-link-arg=-C opt-level=3"); // Enable maximum optimizations
    println!("cargo:rustc-link-arg=-C target-cpu=native"); // Optimize for the current CPU architecture
    println!("cargo:rustc-link-arg=-C lto"); // Enable Link-Time Optimization
    println!("cargo:rustc-link-arg=-C codegen-units=1"); // Reduce codegen units for better optimization
}
