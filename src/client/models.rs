use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Head,
}

impl TryFrom<&str> for RequestMethod {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "get" => Ok(RequestMethod::Get),
            "post" => Ok(RequestMethod::Post),
            "put" => Ok(RequestMethod::Put),
            "patch" => Ok(RequestMethod::Patch),
            "delete" => Ok(RequestMethod::Delete),
            "options" => Ok(RequestMethod::Options),
            "head" => Ok(RequestMethod::Head),
            _ => Err("Invalid HTTP method"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub method: RequestMethod,
    pub headers: HashMap<String, String>,
    pub body: String,
}

/* Testing the conversion */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_convert_str_to_method() {
        assert!(RequestMethod::try_from("GET").is_ok_and(|x| x == RequestMethod::Get));
        assert!(RequestMethod::try_from("post").is_ok_and(|x| x == RequestMethod::Post));
        assert!(RequestMethod::try_from("Patch").is_ok_and(|x| x == RequestMethod::Patch));
        assert!(RequestMethod::try_from("Henry").is_err());
    }
}

/* REFERENCES:
https://dev.to/peterblockman/quick-guide-to-rusts-frominto-and-tryfromtryinto-traits-3gf1
*/
