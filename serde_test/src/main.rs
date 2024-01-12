use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InnerData {
    id: String,
    otherid: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    x: String,
    y: Vec<InnerData>,
}

fn main() {
    let mut v: Vec<InnerData> = vec![];
    for i in 1..10 {
        let ii: i32 = 0 + i;
        v.push(InnerData {
            id: ii.to_string(),
            otherid: format!("data_{ii}"),
        });
    }
    let mydate: MyData = MyData {
        x: "dataX".to_string(),
        y: v.clone(),
    };

    let serializ = serde_json::to_string(&mydate).unwrap();

    println!("serialization {serializ}");
}
