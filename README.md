# factorio-circle-generator
Circle Blueprint Generator for Factorio.

Prints a blueprint book with a circle blueprints. Copy the output and paste it into the game using the Factorio's string import function.

## Usage

```sh
factorio-circle-generator [mode=0] [min=10] [max=120] [fraction=8] [precision=22] [step=1] [entity=stome-wall]
```

- `mode` - output format, 14 or maybe 24 is good.
- `min` - minimal diameter, choose what you want.
- `max` - maximal diameter, choose what you want.
- `fraction` - subpixel circle position or 0 to generate all subpixel blueprints, 8 is good.
- `precision` - subpixel resolution, 22 is good.
- `step` - skip diameters between steps, 1 is good.
- `entity` - stone-wall, land-mine or any other but not tile.

## Mode
- `0` - this text.
- `10` - debug points.
- `11` - pretty json.
- `12` - json.
- `13` - base64.
- `14` - blueprint.
- `2*` - same but only quarter of circle.

## Examples:

### Circle of walls with diameters from 10 to 120

```sh
factorio-circle-generator 14 > out.txt
```

Most likely the output will not fit to the console, so you need to save it to a file.

### Circle of mines with diameters 110, 113, 116, 119 and 122

```sh
factorio-circle-generator 24 110 122 8 22 3 land-mine
```
