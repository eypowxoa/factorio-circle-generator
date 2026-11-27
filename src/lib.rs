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
    mode: N,
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
