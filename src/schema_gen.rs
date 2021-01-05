use crate::Schema;

pub trait Schematized {
    fn schematize(namespace: Option<String>) -> Schema;
}

impl Schematized for bool {
    fn schematize(_namespace: Option<String>) -> Schema {
        Schema::Boolean
    }
}

impl Schematized for i32 {
    fn schematize(_namespace: Option<String>) -> Schema {
        Schema::Int
    }
}

impl Schematized for u32 {
    fn schematize(_namespace: Option<String>) -> Schema {
        // see discussion here: http://apache-avro.679487.n3.nabble.com/unsigned-types-td4028701.html
        Schema::Long
    }
}

impl Schematized for i64 {
    fn schematize(_namespace: Option<String>) -> Schema {
        Schema::Long
    }
}

impl Schematized for f32 {
    fn schematize(_namespace: Option<String>) -> Schema {
        Schema::Float
    }
}

impl Schematized for f64 {
    fn schematize(_namespace: Option<String>) -> Schema {
        Schema::Double
    }
}

impl Schematized for String {
    fn schematize(_namespace: Option<String>) -> Schema {
        Schema::String
    }
}

impl Schematized for Vec<u8> {
    fn schematize(_namespace: Option<String>) -> Schema {
        Schema::Bytes
    }
}
