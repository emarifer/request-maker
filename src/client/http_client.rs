use isahc::http::{Error, Method as IsaMethod};
use isahc::Request as IsaRequest;

use super::{Request, RequestMethod};

impl From<&RequestMethod> for IsaMethod {
    fn from(value: &RequestMethod) -> Self {
        match value {
            RequestMethod::Delete => IsaMethod::DELETE,
            RequestMethod::Get => IsaMethod::GET,
            RequestMethod::Head => IsaMethod::HEAD,
            RequestMethod::Options => IsaMethod::OPTIONS,
            RequestMethod::Patch => IsaMethod::PATCH,
            RequestMethod::Post => IsaMethod::POST,
            RequestMethod::Put => IsaMethod::PUT,
        }
    }
}

pub fn build_request(req: &Request) -> Result<IsaRequest<String>, Error> {
    IsaRequest::builder()
        .uri(&req.url)
        .method(&req.method)
        .body(req.body.clone())
}
