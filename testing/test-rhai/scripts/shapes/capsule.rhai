fn capsule(delta, radius) {
    sequence([
        line(delta),
        isosurface(radius),
    ])
}

