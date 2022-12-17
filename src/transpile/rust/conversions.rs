use pendora_base::types::{RequestType, Type, Value};

pub fn rust_type_conversion(pendora_type: Type) -> String {
    match pendora_type {
        Type::Integer => "i64",
        Type::String => "String",
        Type::Boolean => "bool",
        Type::NullableInteger => "Option<i64>",
        Type::NullableString => "Option<String>",
        Type::NullableBoolean => "Option<bool>",
    }
    .to_string()
}

pub fn rust_request_type_conversion(request_type: RequestType) -> String {
    match request_type {
        RequestType::GET => "\"GET\"",
        RequestType::POST => "\"POST\"",
        RequestType::PATCH => "\"PATCH\"",
        RequestType::DELETE => "\"DELETE\"",
    }
    .to_string()
}

pub fn rust_request_content_conversion(request_content: Value, is_global_context: bool) -> String {
    if is_global_context {
        match request_content {
            Value::Global(i) => format!("self.{}", i),
            Value::Parent(i) => format!("self.{}", i),
            Value::Argument(i) => i.to_string(),
        }
    } else {
        match request_content {
            Value::Global(i) => format!("global.{}", i),
            Value::Parent(i) => format!("self.{}", i),
            Value::Argument(i) => i.to_string(),
        }
    }
}

pub fn rust_surround_quotes(string: String) -> String {
    format!("\"{}\"", string)
}
