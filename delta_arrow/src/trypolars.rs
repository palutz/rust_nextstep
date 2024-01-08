use polars::prelude::*;

fn csv2dataframe(path: &str) -> DataFrame {
    let q = LazyCsvReader::new(path)
        .has_header(true)
        .finish()
        .unwrap()
        .filter(
            col("Project Type")
                .is_not_null()
                .and(col("Project Type").eq(lit("Corporate"))),
        );
    // .group_by(vec![col("species")])
    // .agg([col("*").sum()]);

    let df = q.collect().unwrap();
    println!("Df shape = {:?}", df.shape());

    df
}

// fn read_parquet(path: str) {
//     let lf1 = LazyFrame::scan_parquet("myfile_1.parquet", Default::default())?
//         .group_by([col("ham")])
//         .agg([
//             // expressions can be combined into powerful aggregations
//             col("foo")
//                 .sort_by([col("ham").rank(Default::default(), None)], [false])
//                 .last()
//                 .alias("last_foo_ranked_by_ham"),
//             // every expression runs in parallel
//             col("foo").cum_min(false).alias("cumulative_min_per_group"),
//             // every expression runs in parallel
//             col("foo").reverse().implode().alias("reverse_group"),
//         ]);
//
// let lf2 = LazyFrame::scan_parquet("myfile_2.parquet", Default::default())?
//     .select([col("ham"), col("spam")]);

// let df = lf1
//     .join(lf2, [col("reverse")], [col("foo")], JoinArgs::new(JoinType::Left))
//     // now we finally materialize the result.
//     .collect()?;
// }

// fn create_parquetfile(mut df : DataFrame) {
//     let mut file = std::fs::File::create("data/thegrid.parquet").unwrap();
//     ParquetWriter::new(&mut file). finish(&mut df).unwrap();
// }
//

pub fn query_delta_with_polar() {
    let args = ScanArgsParquet::default();
    // let lf = LazyFrame::scan_parquet("./data/thegrid.parquet", args).unwrap();
    let lf = LazyFrame::scan_parquet(
        "./data/thegrid.delta/0-2df34050-7f4f-4194-aace-05069d96d19e-0.parquet",  // reading directly the delta table
        args,
    )
    .unwrap();
    let df = lf
        .filter(
            //col("URL to project").is_null()
            //.and(
            col("Project Name").eq(lit("SAP Green token")))
        //)
        .collect()
        .unwrap();
    println!("Dataframe: \n {}", df);
}
