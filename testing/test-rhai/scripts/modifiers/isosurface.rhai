fn _isosurface_distance(_delta, distance) {
    #{ distance: distance - _delta }
}

/// Adjust the shape's isosurface by the given distance
fn isosurface(delta) {
    channels(#{
        distance: lift_context_input(curry(Fn("_isosurface_distance"), delta))
    })
}
