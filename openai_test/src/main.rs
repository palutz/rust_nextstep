mod lib_openai;

use std::time::Instant;
// use std::sync::Arc;
use lib_openai::*;


fn main() {
    let start = Instant::now();

    let api_token = std::env::var("OPENAI_API_KEY").expect("expected there to be an openai api token");
    // let embedding_url = "https://api.openai.com/v1/embeddings".to_owned();
    let bearer = format!("Bearer {api_token}");

    //let embeds = Embeddings {
    //    q  : "What is the forecast for next week?".to_string(),
    //    d  : "Economics, Macroeconomic, Finance".to_string(),
    //    ha : "Crisis will be worse".to_string(),
    //    aia: "Crisis will be worse".to_string(),
    //};

    //let calemb = CalcEmbeddings {
    //    embeds,
    //    bearer,
    //    ai_url : embedding_url.to_string(),
    //};

    let askq = AskQuestionSchema {
        question : "What is the forecast for next week?".to_string(),
        domain   : "Economics, Macroeconomic, Finance".to_string(),
        hanswer  : "Crisis will be worse".to_string(),
    };
    //let params = Arc::new(calemb);
    //let res = calculate_embeddings(params);
    let res = open_ai_entry(bearer, askq);

    for v in res.iter() {
        println!("{:?}",v);
    }
    let duration = start.elapsed();
    println!("*********************************");
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
