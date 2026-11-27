use std::env::{Args, args};

use factorio_circle_generator::{N, blueprint};

fn next_arg(ags: &mut Args, def: N) -> N {
    N::from_str_radix(ags.next().unwrap_or(def.to_string()).as_str(), 10).unwrap()
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
    println!(
        "{}",
        blueprint(mode, min, max, fraction, precision, step, &entity)
    )
}
