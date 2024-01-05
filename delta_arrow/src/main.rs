use polars::{prelude::*, lazy::dsl::col};
use futures::executor::block_on;

fn csv2dataframe(path : &str) -> DataFrame {
    let q =  LazyCsvReader::new(path)
                .has_header(true)
                .finish().unwrap()
                .filter(
                    col("Project Type")
                    .is_not_null()
                    .and(col("Project Type").eq(lit("Corporate"))));
        // .group_by(vec![col("species")])
        // .agg([col("*").sum()]);

    let df = q.collect().unwrap();
    println!("Df shape = {:?}", df.shape());

    df
}

fn create_parquetfile(mut df : DataFrame) {
    let mut file = std::fs::File::create("data/thegrid.parquet").unwrap();
    ParquetWriter::new(&mut file). finish(&mut df).unwrap();
}

fn read_delta_file() {
    let table_path = "./data/thegrid.delta";
    let table = block_on(deltalake::open_table(table_path)).unwrap();
    println!("{table}");
}


fn main() {
    // let df = csv2dataframe("data_csv/thegrid.csv");
    // create_parquetfile(df);
    read_delta_file();
}
