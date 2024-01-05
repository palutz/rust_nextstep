mod trydelta;
mod trypolars;

use crate::trydelta::*;

fn main() {
    // let df = csv2dataframe("data_csv/thegrid.csv");
    // create_parquetfile(df);
    read_delta_file();
    // query_delta_with_polar();
    //update_with_delta();
}
