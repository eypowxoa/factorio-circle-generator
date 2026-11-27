use std::env::{Args, args};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json, to_string_pretty};
use zlib_rs::{DeflateConfig, ReturnCode, compress_bound, compress_slice};

type N = i64;

#[derive(Eq, PartialEq)]
struct Point {
    x: N,
    y: N,
}

impl Point {
    fn new(x: N, y: N) -> Self {
        Self { x, y }
    }
}

struct DiameterPoints {
    diameter: N,
    points: Vec<Point>,
}

fn next_arg(ags: &mut Args, def: N) -> N {
    N::from_str_radix(ags.next().unwrap_or(def.to_string()).as_str(), 10).unwrap()
}

fn generate(mode: N, min: N, max: N, fraction: N, precision: N, step: N) -> Vec<DiameterPoints> {
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
                if mode < 20 {
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
                    if mode < 20 {
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

fn main() {
    let mut args = args();
    let program = args.next().unwrap();
    let mode = next_arg(&mut args, 0);
    if 0 == mode {
        eprintln!("{}", format!(
"Circle Blueprint Generator for Factorio.

Prints a blueprint book with a circle blueprints. Copy the output and paste it into the game using the Factorio's string import function.

Usage:
{program} [mode=0] [min=10] [max=120] [fraction=8] [precision=22] [step=1] [entity=stome-wall]

- mode - output format, 14 or maybe 24 is good.
- min - minimal diameter, choose what you want.
- max - maximal diameter, choose what you want.
- fraction - subpixel circle position or 0 to generate all subpixel blueprints, 8 is good.
- precision - subpixel resolution, 22 is good.
- step - skip diameters between steps, 1 is good.
- entity - stone-wall, land-mine or any other but not tile.

Mode:
- 0 - this text.
- 11 - pretty json.
- 12 - json.
- 13 - base64.
- 14 - blueprint.
- 2* - same but only quarter of circle.

Examples:

Circle of walls with diameters from 10 to 120:
{program} 14 > out.txt
Most likely the output will not fit to the console, so you need to save it to a file.

Circle of mines with diameters 8, 11 and 14:
{program} 14 8 14 8 22 3 land-mine
",
        ));
        return;
    };
    let min = next_arg(&mut args, 10);
    let max = next_arg(&mut args, 120);
    let fraction = next_arg(&mut args, 8);
    let precision = next_arg(&mut args, 22);
    let step = next_arg(&mut args, 1);
    let entity = args.next().unwrap_or(String::from("stone-wall"));
    let circles = generate(mode, min, max, fraction, precision, step);
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
        println!("{}", to_string_pretty(&json).unwrap());
        return;
    };
    if 12 == mode || 22 == mode {
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
    if 13 == mode || 23 == mode {
        println!("{b64}");
        return;
    };
    if 14 == mode || 24 == mode {
        println!("0{b64}");
        return;
    };
    panic!("Unknown mode.");
}
