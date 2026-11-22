use std::{env::args};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json, to_string_pretty};
use zlib_rs::{DeflateConfig, ReturnCode, compress_bound, compress_slice};

type N = i64;

fn main() {
    let mut args = args();
    args.next().unwrap();
    let mode = N::from_str_radix(args.next().unwrap_or(String::from("4")).as_str(), 10).unwrap();
    let min = N::from_str_radix(args.next().unwrap_or(String::from("5")).as_str(), 10).unwrap();
    let max = N::from_str_radix(args.next().unwrap_or(String::from("60")).as_str(), 10).unwrap();
    let fraction = N::from_str_radix(args.next().unwrap_or(String::from("7")).as_str(), 10).unwrap();
    let precision = N::from_str_radix(args.next().unwrap_or(String::from("22")).as_str(), 10).unwrap();
    let step = N::from_str_radix(args.next().unwrap_or(String::from("1")).as_str(), 10).unwrap();
    let entity = args.next().unwrap_or(String::from("stone-wall"));
    let mut blueprints: Vec<Value> = Vec::new();
    for radius in min..(max + 1) {
        if (step > 1) && (radius < max) && (((radius - min) % step) != 0) {
            continue;
        };
        for substep in 0..precision {
            if fraction > 0 && substep != fraction {
                continue;
            };
            let radius = ((radius - 1) * precision) + substep + 1;
            let mut points: Vec<(N, N)> = Vec::new();
            let mut at: (N, N) = (radius, 0);
            let mut previous = (N::MIN, N::MIN);
            while at.0 >= 0 {
                match (radius.pow(2) - (at.1 + 1).pow(2)) <= at.0.pow(2) {
                    true => at.0 -= 1,
                    false => at.1 += 1,
                };
                if 0 == mode {
                    println!("◶ {at:?}");
                };
                let current = (at.0 / precision, at.1 / precision);
                if current != previous {
                    points.push((current.0, current.1));
                    points.push((-current.0 - 1, current.1));
                    points.push((-current.0 - 1, -current.1 - 1));
                    points.push((current.0, -current.1 - 1));
                };
                previous = current;
            }
            let radius = radius / precision;
            let s1 = match radius < 100 {
                true => format!("shape-curve-2"),
                false => format!("signal-{}", radius / 100),
            };
            let s2 = match radius < 10 {
                true => format!("shape-curve-3"),
                false => format!("signal-{}", radius / 10),
            };
            let s3 = format!("signal-{}", radius % 10);
            blueprints.push(json!({
                "blueprint": {
                    "snap-to-grid": {
                        "x": 1,
                        "y": 1
                    },
                    "absolute-snapping": true,
                    "icons": [
                        {
                            "signal": {
                                "type": "virtual",
                                "name": "shape-curve",
                            },
                            "index": 1,
                        },
                        {
                            "signal": {
                                "type": "virtual",
                                "name": s1,
                            },
                            "index": 2,
                        },
                        {
                            "signal": {
                                "type": "virtual",
                                "name": s2,
                            },
                            "index": 3,
                        },
                        {
                            "signal": {
                                "type": "virtual",
                                "name": s3,
                            },
                            "index": 4,
                        },
                    ],
                    "entities": points
                        .iter()
                        .enumerate()
                        .map(|(i, (x, y))| json!({
                            "entity_number": i + 1,
                            "name": entity,
                            "position": {
                                "x": (*x as f64) + 0.5,
                                "y": (*y as f64) + 0.5,
                            },
                        }))
                        .collect::<Vec<Value>>(),
                    "item": "blueprint",
                    "label": format!("[virtual-signal=shape-circle] {}", radius + 1),
                    "version": 562954249961473u64,
                },
                "index": blueprints.len(),
            }));
        }
    }
    let json = json!({
        "blueprint_book": {
            "blueprints": blueprints,
            "item": "blueprint-book",
            "label": "[virtual-signal=shape-circle]",
            "icons": [
                {
                    "signal": {
                        "type": "virtual",
                        "name": "shape-circle",
                    },
                    "index": 1,
                },
            ],
            "active_index": 0,
            "version": 562954249961473u64,
        },
    });
    if 0 == mode {
        return;
    };
    if 1 == mode {
        println!("{}", to_string_pretty(&json).unwrap());
        return;
    };
    if 2 == mode {
        println!("{json}");
        return;
    };
    let json = json.to_string();
    let mut buffer = vec![0u8; compress_bound(json.len())];
    let (compressed, rc) = compress_slice(
        &mut buffer,
        json.as_bytes(),
        DeflateConfig::best_compression(),
    );
    if let ReturnCode::Ok = rc {
    } else {
        panic!("Can not compress.");
    }
    let b64 = STANDARD.encode(compressed);
    if 3 == mode {
        println!("{b64}");
        return;
    };
    if 4 == mode {
        println!("0{b64}");
        return;
    };
}
