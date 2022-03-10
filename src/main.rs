use lambda_runtime::{service_fn, Error, LambdaEvent};
use lingua::{Language, LanguageDetectorBuilder};
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

    let detector = LanguageDetectorBuilder::from_all_languages().build();
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
