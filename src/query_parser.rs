use std::collections::HashMap;

pub fn parse_query(qry: &str) -> HashMap<String, String> {
  let mut params = HashMap::new();

  for pair in qry.split('&') {
    let p: Vec<&str> = pair.split('=').collect();
    if p.len() != 2 { continue; }
    let k = p[0].to_string();
    let v = p[1].to_string();
    if params.contains_key(&k) { params.remove(&k); }
    params.insert(k.clone(), v.clone());
  }

  params
}

#[allow(dead_code)]
pub fn parse_from_uri(uri: &str) -> HashMap<String, String> {

  let mut is_qry = false;
  let mut query = String::new();

  for c in uri.chars() {
    is_qry = is_qry || c == '?';
    if is_qry { query.push(c.to_owned()); }
  }

  parse_query(query.as_str())
}