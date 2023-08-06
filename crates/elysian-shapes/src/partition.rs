use elysian_core::{
    ir::{
        ast::Identifier,
        module::{FunctionIdentifier, NumericType, Type},
    },
    property,
};

pub const PARTITION: FunctionIdentifier = FunctionIdentifier::new("partition", 1015123493751899330);

pub const PARTITION_ID: Identifier = Identifier::new("partition_id", 1485962089216017275);
property!(
    PARTITION_ID,
    PARTITION_ID_PROP,
    Type::Number(NumericType::Float)
);
