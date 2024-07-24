// src/lib.rs
#[cfg(feature = "x")]
pub fn feature_x() {
    println!("Feature X is enabled!");
}

#[cfg(not(feature = "x"))]
pub fn feature_x() {
    println!("Feature X is disabled!");
}