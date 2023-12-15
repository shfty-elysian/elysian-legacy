import "scripts/util" as util;
import "scripts/fields/mod" as fields;
import "scripts/modifiers/mod" as modifiers;
import "scripts/shapes/mod" as shapes;
import "scripts/combiners/mod" as combiners;

fn dist_to_color(ctx) {
    let dist = 1.0 - ctx.distance.abs().max(0.0).min(1.0);

    if ctx.distance.sign() < 0.0 {
        [px, 0.0, 0.0]
    }
    else {
        [0.0, 0.0, px]
    }
}

fn grad_to_color(ctx) {
    let dist = ctx.distance;
    let fac = 1.0 - dist.abs().max(0.0).min(1.0);
    let grad = ctx.gradient * 0.5 + 0.5;
    [
        fac * grad[0],
        fac * grad[1],
        1.0 - dist.sign().max(0.0).min(1.0)
    ]
}

let shape = sequence([
    set(#{ channels: ["position", "distance", "gradient"] }),
    elongate([0.0, 0.2]),
    subtraction([
        capsule([0.5, 0.0], 0.25),
        circle(0.5),
    ]),
    grad_to_color,
]);

let i = image_rgb8(64, 48);

i.map(shape);

viuer(i);

shape
