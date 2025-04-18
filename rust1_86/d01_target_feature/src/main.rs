#[target_feature(enable = "avx2")]
fn requires_avx2() {
    // ... snip
}

#[target_feature(enable = "avx2")]
fn safe_callsite() {
    // Calling `requires_avx2` here is safe as `safe_callsite`
    // requires the `avx2` feature itself.
    requires_avx2();
}

fn unsafe_callsite() {
    // Calling `requires_avx2` here is unsafe, as we must
    // ensure that the `avx2` feature is available first.
    if is_x86_feature_detected!("avx2") {
        unsafe { requires_avx2() };
    }
}

fn main() {
    println!("Hello, world!");
}
