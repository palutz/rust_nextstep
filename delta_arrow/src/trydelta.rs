use deltalake::arrow::record_batch::RecordBatch;
use deltalake::datafusion::error::DataFusionError;
// use deltalake::{DeltaTable, SchemaTypeStruct};
use deltalake::datafusion::prelude::*;
use deltalake::arrow::{
    array::{Int64Array, StringArray, TimestampMicrosecondArray},
    datatypes::{DataType as ArrowDataType, Field, Schema as ArrowSchema, TimeUnit},
};
use deltalake::parquet::{
    basic::{Compression, ZstdLevel},
    file::properties::WriterProperties,
};  
//use deltalake::operations::update::UpdateBuilder;
// use deltalake::operations::merge::{ MergeBuilder, InsertBuilder };
use deltalake::DeltaOps;
use deltalake::writer::{DeltaWriter, RecordBatchWriter};
use std::sync::Arc;

async fn table_stat(table_path : &str) {
    let table = deltalake::open_table(table_path).await.unwrap();
    println!("table uri = {}",table.table_uri());
    println!("{table}");
    println!("files: {:?}", table.get_files());
    let schema = table.get_schema().unwrap();
    println!("schema: {:?}", schema);
}

// pub async fn read_delta_file() -> DataFrame {
//     let table_path = "./data/thegrid.delta";
//     // let _ = table_stat(table_path).await;
//     // let mut table = deltalake::open_table_with_version(table_path, 0).await.unwrap(); 
//     let table = deltalake::open_table(table_path).await.unwrap(); 
//     // table.load_version(0).await.unwrap();
//     let data_uri = table.get_file_uris().next().unwrap();
//     println!("**************************");
//     println!("{}", data_uri);
//     println!("*************************");
//     let ctx = SessionContext::new();
//     //ctx.register_table("project1", Arc::new(table)).unwrap();
    //df.show().await.unwrap();
    // df.collect().await.unwrap();
    // println!("{:?}", df);
// }

async fn get_project_by_name(prj: &str, file_uris : Vec<&str>, columns : &[&str]) -> Result<DataFrame, DataFusionError> {
    let ctx = SessionContext::new();
    let df = ctx.read_parquet(file_uris, ParquetReadOptions::default()).await?;
    // df = df.select_columns(&["Project Name", "Project Type", "URL to project", "Founded year (project)"]).unwrap()
    if columns.len() > 0 { 
        df.select_columns(columns)?
    } else {
        df
    }.filter(col("Project Name").eq(lit(prj)))
}

pub async fn get_project(table_paths: Vec<&str>, prj_name: &str) -> Result<DataFrame, DataFusionError> {
    get_project_by_name(prj_name, table_paths, &[]).await
}

pub async fn update_with_delta() {
    let table_path = "./data/thegrid.delta";
    //let table = deltalake::open_table(table_path).await.unwrap(); 
    //let source = get_project_by_name("SAP Green token", 
    //                            vec![table_path], 
    //                            &[]).await;  // take all the columns
                                //&["Project Name", "Project Type", "URL to project", "Founded year (project)"]).await;
    let mut table = deltalake::open_table(table_path).await.unwrap(); 
    let schema = Arc::new(ArrowSchema::new(vec![
            Field::new("Project Name", ArrowDataType::Utf8, true),
            Field::new("Project Type", ArrowDataType::Utf8, true),
            Field::new("URL to project", ArrowDataType::Utf8, true),
            Field::new("Founded year (project)", ArrowDataType::Int64, true),
            Field::new("Industries", ArrowDataType::Utf8, true),
            Field::new("Usecases", ArrowDataType::Utf8, true),
            Field::new("5 word review", ArrowDataType::Utf8, true),
            Field::new("Description (from online)", ArrowDataType::Utf8, true),
    ]));
    let batch = RecordBatch::try_new(
         schema,
        vec![
            Arc::new(StringArray::from(vec!("SAP Green token"))),
            Arc::new(StringArray::from(vec!("Corporate"))),
            Arc::new(StringArray::from(vec!("www.sap.com"))),
            Arc::new(Int64Array::from(vec!(2021))),
            Arc::new(StringArray::from(vec!("EnergyTech"))),
            Arc::new(StringArray::from(vec!("Supply chain"))),
            Arc::new(StringArray::from(vec!("Traceability and supply chain for materials"))),
            Arc::new(StringArray::from(vec!("RE - Not using EVM by the looks of it.   Does not explicity state what blockchain it uses but uses \"proof of authority\" as outlined here 
                            GreenToken by SAP is a system enabled by blockchain technology which ensures transparency of raw material usage which occurs before some products enter a supply chain. 
                            Companies are now subject to Environmental, Social and Governance legislation, like having to ensure some materials are sustainably sourced. 
                            On top of this companies may be motivated to ensure sustainable sources of materials as many consumers are keen to only buy products which are compliant with these values: whether they are written into law or not.")))
        ]
    ).unwrap();

    let writer_properties = 
            WriterProperties::builder()
            .set_compression(Compression::ZSTD(ZstdLevel::try_new(3).unwrap()))
            .build();
  
    //let table = DeltaOps(table)
    //    .write(vec![batch.clone()])
    //    .with_writer_properties(writer_properties)
    //    .await.unwrap();
    let mut writer = RecordBatchWriter::for_table(&table)
                            .expect("Error writing")
                            .with_writer_properties(writer_properties);

    writer.write(batch).await.unwrap();

    let adds = writer
        .flush_and_commit(&mut table)
        .await
        .expect("Failed to flush write");
    
    println!("{} adds written", adds);

    println!("table:   ************************");
    println!("{}", table);
  
  
}
    // answer.show().await.unwrap();
    // let (table, metrics) = 
//            let ops = DeltaOps::try_from_uri(table_path).await.unwrap();
//            ops.update()
//                .with_update("Founded year (project)".to_string(),  lit(2021))
//                .await
//                .unwrap()
//
    // let mut table = deltalake::open_table(table_path).await.unwrap();
    // let (table, metrics) = 
    // UpdateBuilder::new(table.object_store(), table.state)
    // //InsertBuilder::new(table.object_store(), table.load_version(0))
    //     .with_predicate(col("Project Name").eq(lit("SAP Green token")))
    //     .with_update("URL to project", lit("www.SAP.com"))
    //     .await
    //     .unwrap()


    // let source = read_delta_file();

    // let table = deltalake::open_table(table_path).await.unwrap();
    // let (table, metrics) = DeltaOps(table)
    //     .merge(source, col("target.id").eq(col("source.id")))
    //     .with_source_alias("source")
    //     .with_target_alias("target")
    //     .when_matched_update(|update| {
    //         update
    //             .update("value", col("source.value") + lit(1))
    //             .update("modified", col("source.modified"))
    //     })?
    //     .when_not_matched_insert(|insert| {
    //         insert
    //             .set("id", col("source.id"))
    //             .set("value", col("source.value"))
    //             .set("modified", col("source.modified"))
    //     })?
    //     .await?
    // println!("{table}");
// }