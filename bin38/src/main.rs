use lib1::{get, Struct1, Struct2, Struct3};

fn main() {
    let s1 = Struct1::default();
    let s2 = Struct2::default();
    let s3 = Struct3::default();

    get();
    println!("Hello, world! {} {} {}", s1.f1, s2.f2, s3.f1);
}
