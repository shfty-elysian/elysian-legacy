fn _combine_union(lhs, rhs) {
    if lhs.distance < rhs.distance {
        lhs
    } else {
        rhs
    }
}

fn union(array) {
    fold(array, Fn("_combine_union"))
}

