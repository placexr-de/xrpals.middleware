extern crate tokio;
extern crate warp;

use bytes::BufMut;
use chrono::Utc;
use futures::{StreamExt, TryStreamExt};
use std::convert::Infallible;
use warp::{http::StatusCode, multipart::FormData, Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

type PointsMap = BTreeMap<u32, Point>;

#[tokio::main]
async fn main() {
    println!("Starting XR-PALS - LPS server...");

    let index = warp::path::end()
        .map(|| warp::reply::html("XR-PALS - LPS Server"));

    let upload = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(upload);

    // Combine the routes
    let routes = index.or(upload).recover(handle_rejection);

    // Start the server and listen on port 8080
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let mut parts = form.into_stream();
    while let Some(Ok(p)) = parts.next().await {
        if p.name() == "file" {
            let file_ending = "yaml";

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    println!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
            let file_name = format!("./uploads/{}.{}", timestamp, file_ending);
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("Error writing file: {}", e);
                warp::reject::reject()
            })?;
            println!("Created file: {}", file_name);

            // Create File 2 version
            let converted_name = format!("./uploads/{}.xrpals.yaml", timestamp);
            if let Err(e) = convert_file_format(&file_name, &converted_name).await {
                println!("Conversion failed: {}", e);
            } else {
                println!("Created converted file: {}", converted_name);
            }
        }
    }

    Ok("success")
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        println!("Unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}

async fn convert_file_format(
    input_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read original YAML
    let yaml_str = tokio::fs::read_to_string(input_path).await?;

    // Parse YAML format
    let points: PointsMap = serde_yaml::from_str(&yaml_str)?;

    // Serialize in compact (inline) style
    let mut out = String::new();
    for (k, v) in points {
        out.push_str(&format!(
            "{}: {{x: {}, y: {}, z: {}}}\n",
            k, v.x, v.y, v.z
        ));
    }
    tokio::fs::write(output_path, out).await?;
    Ok(())
}
