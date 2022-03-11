use std::str::FromStr;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use lingua::{Language, LanguageDetectorBuilder, IsoCode639_1};
use serde_json::{json, Value};

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

    // FIXME: make languages optional so it will default to use all languages
    let languages_json = body["languages"].as_array().unwrap();

    let languages: Vec<Language> = languages_json.iter()
        .map(|language| IsoCode639_1::from_str(language.as_str().unwrap()).unwrap())
        .map(|iso_code| Language::from_iso_code_639_1(&iso_code))
        .collect();

    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    let confidence_values: Vec<(Language, f64)> = detector.compute_language_confidence_values(text);

    let return_values: Vec<Value> = confidence_values
        .iter()
        .map(|(language, confidence)| {
            json!({
                "language": language.iso_code_639_1().to_string(),
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
