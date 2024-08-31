use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg_attr(feature = "tests", mockall::automock)]
pub trait Trait1 {
    fn fn1();

    fn fn2(a: u16) -> String;

    fn fn3(a: String) -> u16;
}

#[cfg_attr(feature = "tests", mockall::automock)]
pub trait Trait2 {
    fn fn1();

    fn fn2(a: u16) -> String;

    fn fn3(a: String) -> u16;
}

#[cfg_attr(feature = "tests", mockall::automock)]
pub trait Trait3 {
    fn fn1();

    fn fn2(a: u16) -> String;

    fn fn3(a: String) -> u16;
}

#[cfg_attr(feature = "tests", mockall::automock)]
pub trait Trait4 {
    fn fn1();

    fn fn2(a: u16) -> String;

    fn fn3(a: String) -> u16;
}

#[cfg_attr(feature = "tests", mockall::automock)]
pub trait Trait5 {
    fn fn1();

    fn fn2(a: u32) -> String;

    fn fn3(a: String) -> u32;
}

#[cfg_attr(feature = "tests", mockall::automock)]
pub trait Trait6 {
    fn fn1();

    fn fn2(a: u64) -> String;

    fn fn3(a: String) -> u64;
}

#[derive(Serialize, Default)]
#[cfg_attr(feature = "tests", derive(Deserialize, PartialEq, Eq, Debug))]
pub struct Struct1 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Serialize, Default)]
#[cfg_attr(feature = "tests", derive(Deserialize, PartialEq, Eq, Debug))]
pub struct Struct2 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Serialize, Default)]
#[cfg_attr(feature = "tests", derive(Deserialize, PartialEq, Eq, Debug))]
pub struct Struct3 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Serialize, Default)]
#[cfg_attr(feature = "tests", derive(Deserialize, PartialEq, Eq, Debug))]
pub struct Struct4 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Serialize, Default)]
#[cfg_attr(feature = "tests", derive(Deserialize, PartialEq, Eq, Debug))]
pub struct Struct5 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Serialize, Default)]
#[cfg_attr(feature = "tests", derive(Deserialize, PartialEq, Eq, Debug))]
pub struct Struct6 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Serialize, Default)]
#[cfg_attr(feature = "tests", derive(Deserialize, PartialEq, Eq, Debug))]
pub struct Struct7 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Serialize, Default)]
#[cfg_attr(feature = "tests", derive(Deserialize, PartialEq, Eq, Debug))]
pub struct Struct8 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Deserialize, Default)]
#[cfg_attr(feature = "tests", derive(Serialize, PartialEq, Eq, Debug))]
pub struct Struct9 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Deserialize, Default)]
#[cfg_attr(feature = "tests", derive(Serialize, PartialEq, Eq, Debug))]
pub struct Struct10 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Deserialize, Default)]
#[cfg_attr(feature = "tests", derive(Serialize, PartialEq, Eq, Debug))]
pub struct Struct11 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Deserialize, Default)]
#[cfg_attr(feature = "tests", derive(Serialize, PartialEq, Eq, Debug))]
pub struct Struct12 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

#[derive(Deserialize, Default)]
#[cfg_attr(feature = "tests", derive(Serialize, PartialEq, Eq, Debug))]
pub struct Struct13 {
    pub f1: u8,
    pub f2: String,
    pub f3: HashMap<String, String>,
    pub f4: HashSet<String>,
    pub f5: Vec<String>,
}

pub fn get() {
    let resp = reqwest::blocking::get("http://google.com".parse::<Url>().unwrap());
    println!("{:?}", resp.unwrap())
}
