// src/lib.rs
#[cfg(feature = "x")]
pub fn feature_x() {
    println!("Feature X is enabled!");
}

#[cfg(not(feature = "x"))]
pub fn feature_x() {
    println!("Feature X is disabled!");
}
/*
pub struct Foo;

impl Foo {
    pub fn foo() {
        println!("Foo");
    }
}*/

pub struct Foo;
impl Foo {
    pub fn foo(){
        println!("Foo");
    }
}

pub struct Bar;
impl Bar {
    pub fn bar() {
        println!("Bar");
    }
}

struct Baz;
impl Baz {
    fn baz() {
        println!("Baz");
    }
}