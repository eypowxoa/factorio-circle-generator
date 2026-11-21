use std::{env::args, ops::Div};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json, to_string_pretty};
use zlib_rs::{DeflateConfig, ReturnCode, compress_bound, compress_slice};

type N = i64;

fn main() {
    let mut args = args();
    args.next().unwrap();
    let mode = N::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
    let min = N::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
    let max = N::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
    let fraction = N::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
    let precision = N::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
    let step = N::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
    let mut previous_x = N::MIN;
    let mut previous_y = N::MIN;
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
            let mut x: N = radius;
            let mut y: N = 0;
            while x >= 0 {
                match (radius.pow(2) - (y + 1).pow(2)) <= x.pow(2) {
                    true => x -= 1,
                    false => y += 1,
                };
                if 0 == mode {
                    println!("◶ {x} {y}");
                };
                let current_x = x / precision;
                let current_y = y / precision;
                if current_x != previous_x || current_y != previous_y {
                    for a in [
                        (1, 0, 1, 0),
                        (-1, -1, 1, 0),
                        (-1, -1, -1, -1),
                        (1, 0, -1, -1),
                    ] {
                        points.push((current_x * a.0 + a.1, current_y * a.2 + a.3));
                    }
                };
                previous_x = current_x;
                previous_y = current_y;
            }
            let radius = radius / precision;
            let s1 = match radius < 100 {
                true => format!("shape-curve-2"),
                false => format!("signal-{}", radius.div(100)),
            };
            let s2 = match radius < 10 {
                true => format!("shape-curve-3"),
                false => format!("signal-{}", radius.div(10)),
            };
            let s3 = format!("signal-{}", radius % 10);
            blueprints.push(json!({
                "blueprint": {
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
                            "name": "heat-pipe",
                            "position": {
                                "x": (*x as f64) + 0.5,
                                "y": (*y as f64) + 0.5,
                            },
                        }))
                        .collect::<Vec<Value>>(),
                    "item": "blueprint",
                    "label": format!("[virtual-signal=shape-circle] {radius}"),
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
