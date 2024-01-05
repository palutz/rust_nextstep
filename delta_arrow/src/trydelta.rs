use futures::executor::block_on;
use deltalake::datafusion::prelude::*;
use deltalake::operations::update::UpdateBuilder;
use std::sync::Arc;

pub fn read_delta_file() {
    let table_path = "./data/thegrid.delta";
    let table = block_on(deltalake::open_table(table_path)).unwrap();
    // println!("{table}");
    // println!("files: {:?}", table.get_files());
    // let schema = table.get_schema().unwrap();
    // // println!("schema: {:?}", schema);

    let ctx = SessionContext::new();
    //ctx.register_table("project1", Arc::new(table)).unwrap();
    let res = block_on(async{
        let df : DataFrame = ctx
                        .read_parquet(table_path, ParquetReadOptions::default()).await.unwrap() 
                        .select_columns(&["Project Name", "Project Type", "URL to project", "Founded year (project)"]).unwrap();
                        //.filter(col("Project Name").eq(lit("SAP Green token"))).unwrap();
                        //.filter(col("Founded year (project)").gt_eq(lit(2018))).unwrap();
        df.collect().await
    }).unwrap();

    println!("{:?}", res);
}

fn update_with_delta() {
    let table_path = "./data/thegrid.delta";
    let (table, metrics) = block_on(
        async {
            // let ops = DeltaOps::try_from_uri(table_path).await.unwrap();
            // ops.update()
            //    .with_predicate(col("Project Name").eq(lit("SAP Green token")))
            //    .with_update("URL to project", col("URL to project") + lit("www.SAP.com"))

            let table = deltalake::open_table(table_path).await.unwrap();
            UpdateBuilder::new(table.object_store(), table.state)
                .with_predicate(col("Project Name")
                                .is_not_null()
                                .and(col("Project Name").eq(lit("SAP Green token"))))
                //.with_update("URL to project", col("URL to project").eq(lit("www.SAP.com")))
                .with_update("URL to project", lit("www.SAP.com"))
                .await
        }).unwrap();

    println!("{table}");
}