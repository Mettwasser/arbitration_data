use crate::Result;
use std::collections::HashMap;

pub type LanguageDict<'a> = HashMap<&'a str, &'a str>;

fn make_to_language_dict(s: &str) -> Result<LanguageDict<'_>> {
    Ok(serde_json::from_str(s)?)
}
