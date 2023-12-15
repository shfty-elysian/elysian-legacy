fn _set_ctx(data, ctx) {
    ctx += data;
    ctx
}

fn set(data) {
    curry(Fn("_set_ctx"), data)
}

fn _get_ctx(key, ctx) {
    ctx[key]
}

fn get(key) {
    curry(Fn("_get_ctx"), key)
}
