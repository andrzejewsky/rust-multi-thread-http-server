use std::collections::HashMap;
use std::fmt::{ Display, Debug, Formatter, Result as FmtResult };

#[derive(Debug)]
pub struct QueryString<'a > {
  param_bag: HashMap<&'a str, Value<'a>>
}

#[derive(Debug)]
pub enum Value<'a > {
  Single(&'a str),
  Multiple(Vec<&'a str>)
}

impl<'a> QueryString<'a> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.param_bag.get(key)
  }
}

impl<'a> From<&'a str> for QueryString<'a > {
  fn from(s: &'a str) -> Self {
    let mut param_bag = HashMap::new();

    for sub in s.split('&') {
      let mut key = sub;
      let mut val = "";

      if let Some(i) = s.find('=') {
        key = &sub[..i];
        val = &sub[i + 1..];
      }

      param_bag.entry(key)
        .and_modify(|e_value: &mut Value| match e_value {
          Value::Single(prev_val) => {
            *e_value = Value::Multiple(vec![prev_val, val]);
          },
          Value::Multiple(vec) => vec.push(val)
        })
        .or_insert(Value::Single(val));
    }

    QueryString { param_bag }
  }
}


impl<'a> Display for QueryString<'a> {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{:?}", self.param_bag)
  }
}
