use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GridCommit {
    commitinfo : CommitInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CommitInfo {
    operation : String, //Enum??
    timestamp : u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GridProtocol {
    protocol : Protocol,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Protocol {
    //minReaderVersion
    minreaderversion : i16,
    // minwriterVersion
    minwriterversion : i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GridMetaData {
    metadata : MetaData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MetaData {
    id : String,
    format : MetaFormat,
    // schemaString
    schemastr : String,
    // partitionColumns
    partitioncol : Vec<String>,
    // configuration : MetaConfig,
    // createdTime
    createdtime : u128,
    payload : String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MetaFormat {
    provider : String,
    options : String,  //MetaOptions
}

fn main() {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let c = CommitInfo {
        operation : "INSERT".to_string(),
        timestamp,
    };
    let p = Protocol {
        minreaderversion : 1,
        minwriterversion : 1,
    };
    let m = MetaData {
        id : Uuid::new_v4().to_string(),
        format : MetaFormat { provider : "JSON".to_string(), options : "".to_string(), },
        schemastr : "".to_string(),
        partitioncol : vec![],
        createdtime : timestamp,
        payload : "just a try".to_string(),
    };

    let gridmsg = format!("{}\n{}\n{}",
        serde_json::to_string(&GridCommit { commitinfo : c } ).unwrap(),
        serde_json::to_string(&GridProtocol { protocol : p } ).unwrap(),
        serde_json::to_string(&GridMetaData { metadata : m } ).unwrap()
    );
    println!("{gridmsg}");
}
