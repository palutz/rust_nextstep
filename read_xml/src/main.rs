use std::fs::File;
use std::io::{BufReader, Read};
use serde::Deserialize;
use quick_xml::de::from_str;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Result {
    TotalMatches: i32,
    TotalPages: i32,
    PageNumber: i32,
    item: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    mid: String,
    merchantname: String,
    linkid: String,
    createdon: String,
    sku: String,
    productname: String,
    category: Category,
    price: Price,
    saleprice: Price,
    upccode: String,
    description: Description,
    keywords: String,
    linkurl: String,
    imageurl: String,
}

#[derive(Debug, Deserialize)]
struct Category {
    primary: String,
    secondary: String,
}

#[derive(Debug, Deserialize)]
struct Price {
    #[serde(rename = "@currency")]
    currency: String,
}

#[derive(Debug, Deserialize)]
struct Description {
    short: String,
    long: String,
}

fn main() {
    let mut file: BufReader<File> = BufReader::new(File::open("file.xml")
                                    .expect("Error reading file"));
    let mut str = String::new();
    file.read_to_string(&mut str).expect("cannot read string");

    let result: Result = from_str(&str).unwrap();

    println!("{:?}", result);

}
