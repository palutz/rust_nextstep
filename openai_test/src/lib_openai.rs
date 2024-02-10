use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use serde::{Deserialize, Serialize};
use candle_core::{ DType, Device, Tensor};

#[derive(Serialize, Deserialize, Debug)]
pub struct AskQuestionSchema {
    pub question: String,
    pub domain: String,
    pub hanswer: String,
}

#[derive(Default, Clone)]
pub struct Embeddings {
    pub q  : String,
    pub d  : String,
    pub ha : String,
    pub aia: String,
}

#[derive(Default,Clone)]
pub struct CalcEmbeddings {
    pub embeds : Embeddings,
    pub bearer : String,
    pub ai_url : String,
}


// Create an OpenAI request that specifies a system and an user context
fn create_sys_usr_req(
    model: (&str, &str),
    sys_msg: (&str, &str),
    user_msg: (&str, &str),
) -> serde_json::Value {
    serde_json::json!({
        model.0 : model.1,
        "messages": [
            {
                // "role" : sys_msg.get_role(),
                // "content" : sys_msg.get_content()
                "role" : sys_msg.0,
                "content" : sys_msg.1,
            },
            {
                "role" : user_msg.0,
                "content" : user_msg.1,
            }
        ],
        "temperature" : 0.7,
        "max_tokens" : 64,
        "top_p" : 1,
    })
}

fn create_embeeding_req(sentence: &str) -> serde_json::Value {
    serde_json::json!({
        "model": "text-embedding-3-small",
        "input" : sentence,
        "encoding_format": "float",
    })
}

fn ask_openai<'a>(api_url: &'a str, bearer: &'a str, request: serde_json::Value) -> String {
    // println!("request: {request}");
    // println!("**************************");
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", bearer)
        // .bearer_auth(api_token)
        .json(&request)
        .send()
        .unwrap();

    let res_json = res.text_with_charset("utf-8").unwrap();
    res_json
}

fn resp_clean_json(resp: &str) -> serde_json::Value {
    let mut res_clean = resp.replace(r#"\\n"#, "");
    res_clean = res_clean.replace(r#"\n\n"#, " ");
    res_clean = res_clean.replace(r#"\n"#, " ");

    let valjson: serde_json::Value = serde_json::from_str(&res_clean).unwrap();
    valjson
}

// "Models" for OpenAI

fn translate2eng(sentence: &str) -> serde_json::Value {
    let model = ("model", "gpt-4-turbo-preview");
    let sys_msg = ("system","You will be provided with a sentence in Italian, and your task is to translate it into English.");
    let user_msg = ("user", sentence);

    create_sys_usr_req(model, sys_msg, user_msg)
}

///
/// Model for openAI question
///
fn prepare_ai_question(domain: &str, question: &str) -> serde_json::Value {
    let model = ("model", "gpt-4-turbo-preview");
    let sys_req = format!(
        "You are a candidate for a position on the following domain: {}",
        domain
    );
    let sys_msg = ("system", sys_req.as_str());
    let user_msg = ("user", question);

    create_sys_usr_req(model, sys_msg, user_msg)
}

///
/// Check what OpenAI would answer as a candidate
///
// fn chatgpt_candidate(params: Arc<(AskQuestionSchema, String, String)>) -> HashMap<String, String> {
fn chatgpt_candidate(params: Arc<(AskQuestionSchema, String, String)>) -> String {
    let prs = Arc::clone(&params);
    let qs = &prs.0;
    let bearer = prs.1.as_str();
    let openai_url = prs.2.as_str();
    let q_gpt = prepare_ai_question(&qs.domain, &qs.question);
    let res_q_uk = ask_openai(openai_url,bearer, q_gpt);
    let q_json = resp_clean_json(&res_q_uk);
    let q_content = q_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_else(|| "Error");

    q_content.to_string()
}

//
// Translate all the sentences... in threads
//
// fn translate_all_sentences(
//     qs: &AskQuestionSchema,
//     bearer: &[u8],
//     openai_url: &[u8],
// fn translate_all_sentences(params: Arc<(AskQuestionSchema, [u8],[u8])>) -> HashMap<String, String> {
fn translate_all_sentences(params: Arc<(AskQuestionSchema, String, String)>) -> HashMap<String, String> {
    let mut handles: Vec<JoinHandle<(String, String)>> = vec![];

    for i in 0..3 {
        let arcp = Arc::clone(&params);
        let handle = thread::spawn(move || {
            let aqs = &arcp.0;
            let b = &arcp.1.as_str();
            let oai_url = &arcp.2.as_str(); //std::str::from_utf8(&arcp.2).unwrap();
            if i == 0 {
                let q_uk = translate2eng(aqs.question.as_str());
                let res_q_uk = ask_openai(oai_url, b, q_uk);
                let q_uk_json = resp_clean_json(&res_q_uk);
                let q_uk_content = q_uk_json["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or_else(|| "error!!!");
                return ("Q".to_string(), q_uk_content.to_string());
            } else if i == 1 {
                let d_uk = translate2eng(aqs.domain.as_str());
                let res_d_uk = ask_openai(oai_url, b, d_uk);
                let d_uk_json = resp_clean_json(&res_d_uk);
                let d_uk_content = d_uk_json["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or_else(|| "error!!!");
                return ("D".to_string(), d_uk_content.to_string());
            } else {
                // i == 2
                let ha_uk = translate2eng(aqs.hanswer.as_str());
                let res_ha_uk = ask_openai(oai_url, b, ha_uk);
                let ha_uk_json = resp_clean_json(&res_ha_uk);
                let ha_uk_content = ha_uk_json["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or_else(|| "error!!!");
                return ("HA".to_string(), ha_uk_content.to_string());
            }
        });
        handles.push(handle);
    }

    let mut res: HashMap<String, String> = HashMap::new();
    for h in handles {
        let (k, v) = h.join().unwrap(); // saving the result from the threads
        res.insert(k, v);
    }
    res
}

// curl https://api.openai.com/v1/embeddings \
//   -H "Authorization: Bearer $OPENAI_API_KEY" \
//   -H "Content-Type: application/json" \
//   -d '{
//     "input": "The food was delicious and the waiter...",
//   }'
// }

pub fn calculate_embeddings(
    // sentences: &'static Vec<String>,bearer: &'static str,openai_url: &'static str
    params : Arc<CalcEmbeddings>
) -> HashMap<String, Vec<f64>> {
    let mut handles: Vec<JoinHandle<(String, Vec<f64>)>> = vec![];

    for i in 0..4 {
        let arcp = Arc::clone(&params);
        let handle = thread::spawn(move || {
            let th_sentences : [String; 4] = [arcp.embeds.q.to_owned(), arcp.embeds.d.to_owned(), arcp.embeds.ha.to_owned(), arcp.embeds.aia.to_owned()];
            let b = &arcp.bearer;
            let oai_url = &arcp.ai_url;

            let s = th_sentences[i].as_str();

            let emb_req = create_embeeding_req(s);
            let emb_ai = ask_openai(oai_url, b, emb_req);
            let emb_ai_json = resp_clean_json(&emb_ai);
            let vec_emb: Vec<f64> =
                if let Some(emb_ai_content) = emb_ai_json["data"][0]["embedding"].as_array() {
                    emb_ai_content
                        .iter()
                        .map(|x| serde_json::from_value(x.clone()).unwrap())
                        .collect()
                } else {
                    vec![0f64]
                };

            (s.to_string(), vec_emb)
        });
        handles.push(handle);
    }

    let mut res: HashMap<String, Vec<f64>> = HashMap::new();
    for h in handles {
        let r1 = h.join().unwrap();
        res.insert(r1.0, r1.1);
    }
    res
}


fn calculate_cosine_sim(sentences_values: &HashMap<String, Vec<f64>>) {
    let mut handles: Vec<JoinHandle<(String, Vec<f64>)>> = vec![];

    let arc_hash = Arc::new(sentences_values);
    let mut similarities = Arc::new(Mutex::new(vec![]));
    let sentences : Vec<String>  = Arc::clone(&arc_hash).into_keys().collect();
    let keylength = sentences.len();
    for i in 0..keylength {
        let (k_v, similarities) = (Arc::clone(&arc_hash), Arc::clone(&similarities));
        let h = thread::spawn(move || {
                    let sentences : Vec<String>  = arc_hash.into_keys().collect();
                    if let Some(v_i) = k_v.get(&sentences[i]) {
                        for j in (i + 1)..keylength {
                            if let Some(v_j) = k_v.get(&sentences[j]) {
                                let vi32 : Vec<f32> = v_i.into_iter().map(|x| *x as f32).collect();
                                let e_i = Tensor::new(vi32, &Device::Cpu).unwrap();
                                let vj32 : Vec<f32> = v_j.into_iter().map(|x| *x as f32).collect();
                                let e_j = Tensor::new(vj32, &Device::Cpu).unwrap();
                                let sum_ij = (*e_i * *e_j).unwrap().sum_all().unwrap().to_scalar::<f32>().unwrap().;
                                let sum_i2 = (&e_i * &e_i).unwrap().sum_all().unwrap().to_scalar::<f32>().unwrap().;
                                let sum_j2 = (&e_j * &e_j).unwrap().sum_all().unwrap().to_scalar::<f32>().unwrap().;
                                let cosine_similarity = sum_ij / (sum_i2 * sum_j2).sqrt();
                                let mut similarities = similarities.lock().unwrap();
                                *similarities.push((cosine_similarity, i, j));
                            }
                        }
                    }
        })
    };
    similarities.sort_by(|u, v| v.0.total_cmp(&u.0));
    for &(score, i, j) in similarities[..5].iter() {
        println!("score: {score:.2} '{}' '{}'", sentences[i], sentences[j])
    }
}

// CALCULATE DISTANCES fro the embeddings

// from openai import OpenAI
// from typing import List
// import numpy as np
// import pandas as pd
//
// client = OpenAI(max_retries=5)
//
// EMBEDDING_MODEL = "text-embedding-3-small"
//
//
// def get_embedding(text: str, model="text-embedding-3-small", **kwargs) -> List[float]:
//     text = text.replace("\n", " ")  # Performance reason... Check OpenAPI docs!!!
//     response = client.embeddings.create(input=[text], model=model, **kwargs)
//
//     return response.data[0].embedding
//
// # Implementing only the cosine similarity.. let's KISS
// def cosine_similarity(a, b):
//     return np.dot(a, b) / (np.linalg.norm(a) * np.linalg.norm(b))
//
//
// def distances_from_embeddings(
//     query_embedding: List[float],
//     embeddings: List[List[float]],
//     distance_metric="cosine",
// ) -> List[List]:
//     # """Return the distances between a query embedding and a list of embeddings."""
//     distance_metrics = {
//         "cosine": spatial.distance.cosine,
//         "L1": spatial.distance.cityblock,
//         "L2": spatial.distance.euclidean,
//         "Linf": spatial.distance.chebyshev,
//     }
//     distances = [
//         distance_metrics[distance_metric](query_embedding, embedding)
//         for embedding in embeddings
//     ]
//     return distances
//


///
/// Entry point for the OpenAI workflow
///
#[warn(dead_code)]
pub fn open_ai_entry(bearer: String, askquestion: AskQuestionSchema) -> HashMap<String, Vec<f64>> {
    let openai_url = "https://api.openai.com/v1/chat/completions".to_owned();
    let embedding_url = "https://api.openai.com/v1/embeddings".to_owned();

    let askq = AskQuestionSchema {
        ..askquestion
    };
    // let params = Arc::new((askq, bearer, openai_url));
    let params = Arc::new((askq, bearer, openai_url));
    let translated: HashMap<String, String> =
        // translate_all_sentences(&askq, bearer, openai_url);
        translate_all_sentences(Arc::clone(&params));
    let ai_answer: String = chatgpt_candidate(Arc::clone(&params));
    let embeds = Embeddings {
        q  : translated.get("Q").unwrap().to_owned(),
        d  : translated.get("D").unwrap().to_owned(),
        ha : translated.get("HA").unwrap().to_owned(),
        aia: ai_answer,
    };
    let calc_embed = CalcEmbeddings {
        embeds,
        bearer : params.1.to_owned(),
        ai_url : params.2.to_owned(),
    };

    // let _ = calculate_embeddings(&sentences, &params.1.to_owned(), &params.2.to_owned());
    let hm = calculate_embeddings(Arc::new(calc_embed));

    hm
}
// let body = reqwest::blocking::get("https://www.rust-lang.org").unwrap().text();
// let sys_msg = ("system","You are a poetic assistant, skilled in explaining complex programming concepts with creative flair.");
// let user_msg = ("user","Compose a poem that explains the concept of recursion in programming.");
// let model = ("model", "gpt-3.5-turbo");
// let openai_url = "https://api.openai.com/v1/chat/completions";
// let res_json = ask_openai(openai_url, bearer, req_json);
// clean the result and convert to a Json Object
// let valjson = resp_clean_json(&res_json);
// extract the information we need
// let content = valjson["choices"][0]["message"]["content"].as_str().unwrap_or_else(|| "error!!!");
// content.to_string()
