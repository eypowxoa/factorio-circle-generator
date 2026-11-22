use std::env::{Args, args};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json, to_string_pretty};
use zlib_rs::{DeflateConfig, ReturnCode, compress_bound, compress_slice};

type N = i64;

fn next_arg(ags: &mut Args, def: N) -> N {
    N::from_str_radix(ags.next().unwrap_or(def.to_string()).as_str(), 10).unwrap()
}

fn main() {
    let mut args = args();
    args.next().unwrap();
    let mode = next_arg(&mut args, 4);
    let min = next_arg(&mut args, 10);
    let max = next_arg(&mut args, 120);
    let fraction = next_arg(&mut args, 8);
    let precision = next_arg(&mut args, 22);
    let step = next_arg(&mut args, 1);
    let entity = args.next().unwrap_or(String::from("stone-wall"));
    let mut blueprints: Vec<Value> = Vec::new();
    for diameter in min..(max + 1) {
        let is_odd = diameter % 2 != 0;
        if (step > 1) && (diameter < max) && (((diameter - min) % step) != 0) {
            continue;
        };
        for substep in 1..=precision {
            if fraction > 0 && substep != fraction {
                continue;
            };
            let radius = (((diameter / 2 - 1) * precision) + substep) * 2;
            let mut points: Vec<(N, N)> = Vec::new();
            let mut at: (N, N) = (radius, 0);
            let mut previous = (N::MIN, N::MIN);
            if is_odd {
                points.push((diameter / 2 - 1, -1));
                points.push((-diameter / 2 - 1, -1));
                points.push((-1, diameter / 2 - 1));
                points.push((-1, -diameter / 2 - 1));
            };
            while at.0 >= 0 {
                match (radius.pow(2) - (at.1 + 1).pow(2)) <= at.0.pow(2) {
                    true => at.0 -= 1,
                    false => at.1 += 1,
                };
                if 0 == mode {
                    println!("◶ {at:?}");
                };
                let s = N::from(is_odd);
                let current = ((at.0 - s) / (precision * 2), (at.1 - s) / (precision * 2));
                if current != previous {
                    points.push((current.0, current.1));
                    points.push((-current.0 - 1 - s, current.1));
                    points.push((-current.0 - 1 - s, -current.1 - 1 - s));
                    points.push((current.0, -current.1 - 1 - s));
                };
                previous = current;
            }
            blueprints.push(json!({
                "blueprint": {
                    "snap-to-grid": {"x": 1, "y": 1},
                    "absolute-snapping": true,
                    "icons": [
                        {"signal": {"type": "virtual", "name": "shape-curve"}, "index": 1},
                        {"signal": {"type": "virtual", "name": match diameter < 100 {
                            true => format!("shape-curve-2"),
                            false => format!("signal-{}", diameter / 100),
                        }}, "index": 2},
                        {"signal": {"type": "virtual", "name": match diameter < 10 {
                            true => format!("shape-curve-3"),
                            false => format!("signal-{}", diameter / 10),
                        }}, "index": 3},
                        {"signal": {"type": "virtual", "name": format!("signal-{}", diameter % 10)}, "index": 4},
                    ],
                    "entities": points.iter().enumerate().map(|(i, (x, y))| json!({
                        "entity_number": i + 1,
                        "name": entity,
                        "position": {"x": (*x as f64) + 0.5, "y": (*y as f64) + 0.5},
                    })).collect::<Vec<Value>>(),
                    "item": "blueprint",
                    "label": format!("[virtual-signal=shape-circle] {}", diameter),
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
            "icons": [{"signal": {"type": "virtual", "name": "shape-circle"}, "index": 1}],
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
