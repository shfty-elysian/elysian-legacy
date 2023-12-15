fn _combine_intersection(lhs, rhs) {
    if lhs.distance > rhs.distance {
        lhs
    } else {
        rhs
    }
}

fn intersection(array) {
    fold(array, Fn("_combine_intersection"))
}

