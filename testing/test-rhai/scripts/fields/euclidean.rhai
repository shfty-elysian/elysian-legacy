fn _euclidean_distance(position) {
    #{ distance: position.length() }
}

fn _euclidean_gradient(position) {
    #{ gradient: position.normalize() }
}

// Cartesian distance metric
fn euclidean() {
    channels(#{
        distance: lift_context_input(Fn("_euclidean_distance")),
        gradient: lift_context_input(Fn("_euclidean_gradient")),
    });
}

