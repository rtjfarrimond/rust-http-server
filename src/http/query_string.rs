use std::collections::HashMap;
use std::convert::From;
use std::string::ToString;
use std::vec::Vec;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

impl<'buf> ToString for QueryString<'buf> {
    fn to_string(&self) -> String {
        let mut s = String::from("?");
        for (k, v) in self.data.iter() {
            s.push_str(k);
            s.push('=');
            s.push_str(&v.to_string());
        }
        s
    }
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> ToString for Value<'buf> {
    fn to_string(&self) -> String {
        match self {
            Self::Single(s) => s.to_string(),
            Self::Multiple(vec) => {
                let mut concatenated = String::new();
                for s in vec {
                    concatenated.push_str(s);
                    concatenated.push(',');
                }
                concatenated
            }
        }
    }
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut value = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                value = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing| match existing {
                    Value::Single(prev_val) => *existing = Value::Multiple(vec![prev_val, value]),
                    Value::Multiple(vec) => vec.push(value),
                })
                .or_insert(Value::Single(value));
        }

        QueryString { data }
    }
}
