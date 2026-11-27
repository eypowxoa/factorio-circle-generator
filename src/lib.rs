use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json, to_string_pretty};
use zlib_rs::{DeflateConfig, ReturnCode, compress_bound, compress_slice};

pub type N = i64;

#[derive(Eq, PartialEq)]
pub struct Point {
    pub x: N,
    pub y: N,
}

impl Point {
    pub fn new(x: N, y: N) -> Self {
        Self { x, y }
    }
}

pub struct DiameterPoints {
    pub diameter: N,
    pub points: Vec<Point>,
}

pub fn generate(
    quarter: bool,
    min: N,
    max: N,
    fraction: N,
    precision: N,
    step: N,
) -> Vec<DiameterPoints> {
    let mut circles = Vec::new();
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
            let mut points = Vec::new();
            let mut at = Point::new(radius, 0);
            let mut previous = Point::new(N::MIN, N::MIN);
            if is_odd {
                points.push(Point::new(diameter / 2 - 1, -1));
                if !quarter {
                    points.push(Point::new(-diameter / 2 - 1, -1));
                    points.push(Point::new(-1, diameter / 2 - 1));
                    points.push(Point::new(-1, -diameter / 2 - 1));
                };
            };
            while at.x >= 0 {
                match (radius.pow(2) - (at.y + 1).pow(2)) <= at.x.pow(2) {
                    true => at.x -= 1,
                    false => at.y += 1,
                };
                let s = N::from(is_odd);
                let current =
                    Point::new((at.x - s) / (precision * 2), (at.y - s) / (precision * 2));
                if current != previous {
                    points.push(Point::new(current.x, current.y));
                    if !quarter {
                        points.push(Point::new(-current.x - 1 - s, current.y));
                        points.push(Point::new(-current.x - 1 - s, -current.y - 1 - s));
                        points.push(Point::new(current.x, -current.y - 1 - s));
                    };
                };
                previous = current;
            }
            circles.push(DiameterPoints { diameter, points });
        }
    }
    circles
}

pub fn blueprint(
    mode: N,
    min: N,
    max: N,
    fraction: N,
    precision: N,
    step: N,
    entity: &str,
) -> String {
    let circles = generate(mode >= 20, min, max, fraction, precision, step);
    let mut blueprints: Vec<Value> = Vec::new();
    for DiameterPoints { diameter, points } in circles {
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
                "entities": points.iter().enumerate().map(|(i, Point{ x, y })| json!({
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
    if 11 == mode || 21 == mode {
        return format!("{}", to_string_pretty(&json).unwrap());
    };
    if 12 == mode || 22 == mode {
        return format!("{json}");
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
    if 13 == mode || 23 == mode {
        return format!("{b64}");
    };
    if 14 == mode || 24 == mode {
        return format!("0{b64}");
    };
    panic!("Unknown mode.");
}
