use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Intercepted {
    leaves: Vec<Leaf>,
}

#[derive(Debug)]
enum Primitive {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
}

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Null => write!(f, "null")?,
            Primitive::Bool(b) => write!(f, "{}", b)?,
            Primitive::Number(n) => write!(f, "{}", n)?,
            Primitive::String(s) => write!(f, r#""{}""#, s)?,
        };
        Ok(())
    }
}

#[derive(Debug)]
struct Leaf {
    path: String,
    value: Primitive,
}

use Primitive as P;

fn flatten(prefix: String, mut acc: &mut Vec<Leaf>, json_value: Value) {
    match json_value {
        Value::Null => acc.push(Leaf {
            path: prefix.clone(),
            value: P::Null,
        }),
        Value::Bool(b) => acc.push(Leaf {
            path: prefix.clone(),
            value: P::Bool(b),
        }),
        Value::Number(n) => acc.push(Leaf {
            path: prefix.clone(),
            value: P::Number(n),
        }),
        Value::String(s) => acc.push(Leaf {
            path: prefix.clone(),
            value: P::String(s),
        }),
        Value::Array(arr) => {
            for (i, ele) in arr.into_iter().enumerate() {
                flatten(format!("{}[{}]", prefix, i), &mut acc, ele);
            }
        }
        Value::Object(o) => {
            for (k, v) in o {
                flatten(format!("{}.{}", prefix, k), &mut acc, v);
            }
        }
    }
}
impl<'de> Deserialize<'de> for Intercepted {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        let prefix = String::new();
        let mut acc = vec![];
        flatten(prefix, &mut acc, value);
        Ok(Self { leaves: acc })
    }
}
fn main() {
    let match_with = std::env::args().last().unwrap();
    let matcher = SkimMatcherV2::default();
    let input = std::io::stdin().lock();
    let val: Intercepted = serde_json::from_reader(input).unwrap();
    for leaf in val.leaves {
        if let Some(_) = matcher.fuzzy_match(leaf.path.as_str(), match_with.as_str()) {
            println!("{} : {}", leaf.path, leaf.value)
        }
    }
}
