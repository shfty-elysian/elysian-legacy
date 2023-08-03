#[macro_export]
macro_rules! elysian_function_arg {
    ($target:expr => mut $arg:ident, $($next:tt)*) => {
        $crate::elysian_function_arg!(
            $target.chain([elysian_core::ir::module::InputDefinition {
                id: $arg.clone().into(),
                mutable: true,
            }]) => $($next)*
        )
    };
    ($target:expr => $arg:ident, $($next:tt)*) => {
        $crate::elysian_function_arg!(
        $target.chain([elysian_core::ir::module::InputDefinition {
            id: $arg.clone().into(),
            mutable: false,
        }]) => $($next)*)
    };
    ($target:expr => mut $arg:ident) => {
        $target.chain([elysian_core::ir::module::InputDefinition {
            id: $arg.clone().into(),
            mutable: true,
        }])
    };
    ($target:expr => $arg:ident) => {
        $target.chain([elysian_core::ir::module::InputDefinition {
            id: $arg.clone().into(),
            mutable: false,
        }])
    };
}

#[macro_export]
macro_rules! elysian_function_args {
    ($($args:tt)*) => {
        $crate::elysian_function_arg!($($args)*).collect()
    };
}

#[macro_export]
macro_rules! elysian_function_inner {
    ($pub:tt | $ident:ident | [$($args:tt)*] | $ret:ident | $body:tt) => {
        FunctionDefinition {
            id: $ident,
            public: $pub,
            inputs: $crate::elysian_function_args!([].into_iter() => $($args)*),

            output: $ret.into(),
            block: elysian_proc_macros::elysian_block! $body,
        }
    };
}

#[macro_export]
macro_rules! elysian_function {
    (pub fn $ident:ident($($args:tt)*) -> $ret:ident $body:tt) => {
        $crate::elysian_function_inner!(true | $ident | [$($args)*] | $ret | $body)
    };
    (fn $ident:ident($($args:tt)*) -> $ret:ident $body:tt) => {
        $crate::elysian_function_inner!(false | $ident | [$($args)*] | $ret | $body)
    };
}
