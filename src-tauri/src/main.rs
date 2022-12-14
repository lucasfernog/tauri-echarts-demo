#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, sync::Mutex};

fn main() {
    let fake_nebula = include_bytes!("../../public/fake-nebula.bin").to_vec();

    let mut map = HashMap::new();
    map.insert("/fake-nebula.bin", fake_nebula);

    let last_points = Mutex::new(HashMap::<String, usize>::new());

    tauri::Builder::default()
        .register_uri_scheme_protocol("plot", move |app, req| {
            use tauri::http::ResponseBuilder;

            let url = url::Url::parse(req.uri()).unwrap();

            let data = map
                .get(url.path())
                .unwrap_or_else(|| panic!("data not found for {}", url.path()))
                .clone();

            let mut last_points_ = last_points.lock().unwrap();
            let mut skip = last_points_.get(url.path()).copied().unwrap_or_default();
            let mut take = data.len() / 8;

            for (k, v) in url.query_pairs() {
                if k == "points" {
                    if let Ok(points) = v.parse::<usize>() {
                        take = points * 8;
                        break;
                    }
                }
            }

            if take as isize > data.len() as isize - skip as isize {
                skip = 0;
            }

            last_points_.insert(url.path().to_string(), skip + take * 8);

            ResponseBuilder::new()
                .header("Content-Type", "application/octet-stream")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Method", "*")
                .body(data.into_iter().skip(skip).take(take).collect())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
