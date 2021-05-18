
use serde_yaml::Value as Yaml;
use serde_json::Value as Json;

pub trait ToSerdeJson {
    fn yaml_to_json(self) -> Json;
}

impl ToSerdeJson for serde_yaml::Value {
    fn yaml_to_json(self) -> serde_json::Value {
        match self {
            Yaml::Null => Json::Null,
            Yaml::Bool(b) => Json::Bool(b),
            Yaml::Number(n) => {
                if let Some(i) = n.as_i64() {
                    return Json::Number(i.into());
                }
                if let Some(u) = n.as_u64() {
                    return Json::Number(u.into());
                }
                if let Some(f) = n.as_f64() {
                    return Json::Number(serde_json::Number::from_f64(f).unwrap());
                }
                unreachable!()
            }
            Yaml::String(s) => Json::String(s),
            Yaml::Sequence(seq) => Json::Array(seq.into_iter().map(ToSerdeJson::yaml_to_json).collect()),
            Yaml::Mapping(map) => {
                let mut ret = serde_json::Map::with_capacity(map.len());
                for (key, value) in map {
                    let key = match key {
                        Yaml::Null => "null".to_string(),
                        Yaml::Bool(b) => b.to_string(),
                        Yaml::Number(n) => n.to_string(),
                        Yaml::String(s) => s.to_string(),
                        _ => unreachable!(),
                    };
                    ret.insert(key, ToSerdeJson::yaml_to_json(value));
                }
                Json::Object(ret)
            }
        }
    }
}
