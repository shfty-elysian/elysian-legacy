fn _elongate_position(_delta, position) {
    #{ position: position.sign() * (position.abs() - vec(_delta)).max(vec([])) }
}

/// Elongate the shape by the given delta
fn elongate(delta) {
    channels(#{
        position: lift_context_input(curry(Fn("_elongate_position"), delta))
    })
}
