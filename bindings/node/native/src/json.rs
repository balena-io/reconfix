
use neon::scope::Scope;
use neon::vm::{VmResult, JsResult};
use neon::js::{JsValue, JsObject, JsArray, Object, Variant};
use neon::js::Value as ManagedValue;
use neon::js::error::{JsError, Kind};
use neon::mem::Handle;

use serde_json::{Value, Number};
use serde_json::map::Map;

pub fn from_managed<'a, S: Scope<'a>>(scope: &mut S, managed: Handle<JsValue>) -> VmResult<Value>
{
    match managed.variant() {
        Variant::Null(_) => Ok(Value::Null),
        Variant::Boolean(b) => Ok(Value::Bool(b.value())),
        Variant::Number(n) => Number::from_f64(n.value())
                                .map(Value::Number)
                                .ok_or(())
                                .or_else(|_| JsError::throw(Kind::TypeError, "invalid float value")),
        Variant::String(s) => Ok(Value::String(s.value())),
        Variant::Object(o) => convert_managed_obj(scope, o).map(Value::Object),
        Variant::Array(a) => convert_managed_arr(scope, a).map(Value::Array),
        _ => return JsError::throw(Kind::TypeError, "unsuported data type"),
    }
}

fn convert_managed_obj<'a, S: Scope<'a>>(scope: &mut S, obj: Handle<JsObject>) -> VmResult<Map<String, Value>>
{
    let mut map = Map::new();
    let keys = obj.get_own_property_names(scope)?
        .to_vec(scope)?;
    
    for key in keys {
        let value = obj.get(scope, key)?;
        let name = key.to_string(scope)?.value();
        let converted = from_managed(scope, value)?;
        map.insert(name, converted);
    }

    Ok(map)
}

fn convert_managed_arr<'a, S: Scope<'a>>(scope: &mut S, arr: Handle<JsArray>) -> VmResult<Vec<Value>>
{
    let mut vec = Vec::new();
    let items = arr.to_vec(scope)?;

    for item in items {
        let converted = from_managed(scope, item)?;
        vec.push(converted);
    }

    Ok(vec)
}

pub fn from_native<'a, S: Scope<'a>>(scope: &mut S, native: Value) -> JsResult<'a, JsValue>
{
    use neon::js::{JsNull, JsBoolean, JsNumber, JsString};

    match native {
        Value::Null => Ok(JsNull::new().upcast()),
        Value::Bool(b) => Ok(JsBoolean::new(scope, b).upcast()),
        Value::Number(n) => {
            n.as_f64()
                .map(|f| JsNumber::new(scope, f).upcast())
                .ok_or(())
                .or_else(|_| JsError::throw(Kind::TypeError, "invalid float value"))
        },
        Value::String(s) => JsString::new_or_throw(scope, &s).map(|s| s.upcast()),
        Value::Object(o) => convert_native_obj(scope, o).map(|o| o.upcast()),
        Value::Array(a) => convert_native_arr(scope, a).map(|a| a.upcast()),
    }
}

fn convert_native_obj<'a, S: Scope<'a>>(scope: &mut S, map: Map<String, Value>) -> JsResult<'a, JsObject>
{
    let obj = JsObject::new(scope);

    for (key, value) in map {
        let converted = from_native(scope, value)?;
        obj.set(key.as_ref(), converted)?;
    }

    Ok(obj)
}

fn convert_native_arr<'a, S: Scope<'a>>(scope: &mut S, vec: Vec<Value>) -> JsResult<'a, JsArray>
{
    let arr = JsArray::new(scope, vec.len() as u32);

    for (index, value) in vec.into_iter().enumerate() {
        let converted = from_native(scope, value)?;
        arr.set(index as u32, converted)?;
    }

    Ok(arr)
}

