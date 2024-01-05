use polars::{prelude::*, lazy::dsl::col};


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

// fn create_parquetfile(mut df : DataFrame) {
//     let mut file = std::fs::File::create("data/thegrid.parquet").unwrap();
//     ParquetWriter::new(&mut file). finish(&mut df).unwrap();
// }
//

// fn query_delta_with_polar() {
//     let args = ScanArgsParquet::default();
//     let lf = LazyFrame::scan_parquet("./data/thegrid.parquet", args).unwrap();
//     let df = lf.
//             filter(col("URL to project").is_null())
//             .collect().unwrap();
//     println!("Dataframe: \n {}",df);
// }