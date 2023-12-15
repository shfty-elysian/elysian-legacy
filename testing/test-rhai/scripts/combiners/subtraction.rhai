fn _combine_subtraction(lhs, rhs) {
    rhs.distance = -rhs.distance;
    if lhs.distance > rhs.distance {
        lhs
    } else {
        rhs
    }
}

fn subtraction(array) {
    fold(array, Fn("_combine_subtraction"))
}
