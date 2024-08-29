use los::Struct1;
use los2::Struct2;
use los3::Struct3;

fn main() {
    let s1 = Struct1::default();
    let s2 = Struct2::default();
    let s3 = Struct3::default();
    println!("Hello, world! {} {} {}", s1.f1, s2.f2, s3.f1);
}
