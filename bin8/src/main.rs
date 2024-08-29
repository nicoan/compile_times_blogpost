use lib1::{get as get1, Struct1};
use lib2::{get as get2, Struct2};
use lib3::{get as get3, Struct3};

fn main() {
    let s1 = Struct1::default();
    let s2 = Struct2::default();
    let s3 = Struct3::default();

    get1();
    get2();
    get3();
    println!("Hello, world! {} {} {}", s1.f1, s2.f2, s3.f1);
}
