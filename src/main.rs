use std::{env::args, ops::Div};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json, to_string_pretty};
use zlib_rs::{DeflateConfig, ReturnCode, compress_bound, compress_slice};

fn intersect(r: i32, x: i32) -> i32 {
    if x.abs() > r.abs() {
        panic!("Too big");
    };

    r.pow(2) - x.pow(2)
}

enum Cmp {
    Before,
    Start,
    In,
    End,
    After,
}

fn compare(r: i32, x: i32, m: i32) -> Cmp {
    let i = intersect(r, x);
    let a = m.pow(2);
    let b = (m.abs() + 1).pow(2);
    if i < a {
        Cmp::Before
    } else if i == a {
        Cmp::Start
    } else if i < b {
        Cmp::In
    } else if i == b {
        Cmp::End
    } else {
        Cmp::After
    }
}

fn main() {
    let mut args = args();
    args.next().unwrap();
    let mode = i32::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
    let min = i32::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
    let max = i32::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
    let step = i32::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
    let mut blueprints: Vec<Value> = Vec::new();
    for r in min..(max + 1) {
        let r = r * step;
        let mut p: Vec<(i32, i32)> = Vec::new();
        let mut x = r - 1;
        let mut y = 0;
        while x >= 0 {
            match compare(r, y + 1, x) {
                Cmp::Before | Cmp::Start => x -= 1,
                Cmp::In | Cmp::End | Cmp::After => y += 1,
            };
            if 0 == mode {
                println!("◶ {x} {y}");
            };
            p.push((x, y));
        }
        while y >= 0 {
            match compare(r, x, y) {
                Cmp::Before | Cmp::Start => y -= 1,
                Cmp::In | Cmp::End | Cmp::After => x -= 1,
            };
            if 0 == mode {
                println!("◵ {x} {y}");
            };
            p.push((x, y));
        }
        while x <= -1 {
            match compare(r, y, x + 1) {
                Cmp::Before | Cmp::Start => x += 1,
                Cmp::In | Cmp::End | Cmp::After => y -= 1,
            };
            if 0 == mode {
                println!("◴ {x} {y}");
            };
            p.push((x, y));
        }
        while y <= -1 {
            match compare(r, x + 1, y + 1) {
                Cmp::Before | Cmp::Start => y += 1,
                Cmp::In | Cmp::End | Cmp::After => x += 1,
            };
            if 0 == mode {
                println!("◷ {x} {y}");
            };
            p.push((x, y));
        }
        let s1 = match r < 100 {
            true => format!("shape-curve-2"),
            false => format!("signal-{}", r.div(100)),
        };
        let s2 = match r < 10 {
            true => format!("shape-curve-3"),
            false => format!("signal-{}", r.div(10)),
        };
        let s3 = format!("signal-{}", r % 10);
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
                "entities": p
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
                "label": format!("[virtual-signal=shape-circle] {r}"),
                "version": 562954249961473u64,
            },
            "index": blueprints.len(),
        }));
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
