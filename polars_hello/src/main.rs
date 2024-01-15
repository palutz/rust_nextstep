use serde;
use polars::prelude::*;

#[derive(Serialize, Deserialize)]
struct Project{
    name : String,
    prjtype : String, 
    url : String,
    found_year : i16,
    industries : String,
    usecases : String,
    review : String,
    description : String,
}

fn main() {
    let df_csv = CsvReader::from_path("data_csv/thegrid.csv").unwrap()
        .infer_schema(None)
        .has_header(true)
        .with_try_parse_dates(true)
        .finish().unwrap();
    println!("Whole dataset shape: {:?}", df_csv.shape());

    let mut out = df_csv.clone().lazy()
                .select([col("*")])
                // .filter(col("URL to project").is_null())
                .filter(col("Project Name").eq(lit("SAP Green token")))
                .collect().unwrap();

    
    // println!("{}", out);

    // out = out.clone().lazy()
    //         // .select([col("Project Name"), col("Project Type"), col("URL to project")])
    //         .filter(col("Project Name").eq(lit("SAP Green token")))
    //         .collect().unwrap();

    let row = out.get(1).unwrap();
    println!("{:?}", row);
}
