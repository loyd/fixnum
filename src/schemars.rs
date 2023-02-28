pub use schemars::{JsonSchema, gen::SchemaGenerator, schema::{Schema, SchemaObject, InstanceType}};

use crate::{FixedPoint};

impl<I, P> JsonSchema for FixedPoint<I, P> {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        "String".to_owned()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}