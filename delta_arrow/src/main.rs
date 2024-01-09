mod trydelta;
// mod trypolars;

use crate::trydelta::*;
// use crate::trypolars::*;

#[tokio::main]
async fn main() {
    // let df = csv2dataframe("data_csv/thegrid.csv");
    // create_parquetfile(df);
    // query_delta_with_polar();
    get_project(vec!("./data/thegrid.delta"), "SAP Green token").await.unwrap().show().await.unwrap();
    //update_with_delta().await;
    //let df = read_delta_file().await;
    //df.show().await.unwrap();

}
