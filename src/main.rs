use lambda_runtime::{service_fn, Error, LambdaEvent};
use lingua::{IsoCode639_1, Language, LanguageDetectorBuilder};
use serde_json::{json, Value};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    println!("{}", serde_json::to_string(&event).unwrap());
    let body_json = event["body"].as_str().unwrap();
    let body: Value = serde_json::from_str(body_json).unwrap();
    let text = body["text"].as_str().unwrap();

    let empty_vec: Vec<Value> = vec![];
    let languages_json = match body["languages"].as_array() {
        Some(languages_json) => languages_json,
        None => &empty_vec,
    };

    let languages: Vec<Language> = languages_json
        .iter()
        .map(|language| IsoCode639_1::from_str(language.as_str().unwrap()).unwrap())
        .map(|iso_code| Language::from_iso_code_639_1(&iso_code))
        .collect();

    let detector = if languages.len() > 1 {
        LanguageDetectorBuilder::from_languages(&languages).build()
    } else {
        LanguageDetectorBuilder::from_all_languages()
            .with_minimum_relative_distance(0.1)
            .build()
    };

    let confidence_values: Vec<(Language, f64)> = detector.compute_language_confidence_values(text);

    let return_values: Vec<Value> = confidence_values
        .iter()
        .map(|(language, confidence)| {
            json!({
                "language": language.iso_code_639_1().to_string(),
                "language_name": language.to_string(),
                "confidence": confidence
            })
        })
        .collect();

    let return_body = json!({
        "detected_languages": return_values
    });

    Ok(json!({
        "statusCode": 200,
        "body": return_body.to_string()
    }))
}
