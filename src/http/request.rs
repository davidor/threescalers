use crate::api_call::{Kind::*, *};
use crate::application::*;
use crate::user::*;
use crate::version::*;
use crate::ToParams;

use super::RequestParameters;

use http::{Request, Method};

#[derive(Clone, Debug)]
pub struct Req {
    pub method: Method,
    pub path: &'static str,
    pub rp: RequestParameters,
}

// TODO make it ok for any T, also () which means no body!!!
impl From<Req> for Request<String> {
    fn from(r: Req) -> Self {
        let body = match r.rp.body() {
            Some(s) => s.to_string(),
            None => String::new(),
        };
        let mut rb = Request::builder();
        rb.header("User-Agent", USER_AGENT)
            .method(r.method)
            .uri(r.rp.path_and_query(r.path).as_ref())
            .body(body)
            .unwrap()
    }
}

impl From<&ApiCall<'_, '_, '_, '_, '_>> for Req {
    fn from(i: &ApiCall) -> Self {
        use super::endpoints::*;

        let (method, path) = match (i.kind(), i.application(), i.user()) {
            (Authorize, Application::OAuthToken(_), _) => OAUTH_AUTHORIZE_ENDPOINT,
            (Authorize, _, Some(&User::OAuthToken(_))) => OAUTH_AUTHORIZE_ENDPOINT,
            (Authorize, _, _) => AUTHORIZE_ENDPOINT,
            (AuthRep, Application::OAuthToken(_), _) => OAUTH_AUTHREP_ENDPOINT,
            (AuthRep, _, Some(&User::OAuthToken(_))) => OAUTH_AUTHREP_ENDPOINT,
            (AuthRep, _, _) => AUTHREP_ENDPOINT,
            (Report, _, _) => REPORT_ENDPOINT,
        };
        let mut params = Vec::new();
        i.to_params(&mut params);
        let rp = RequestParameters::new(&method, &params);

        Req { method, path, rp }
    }
}

impl From<&ApiCall<'_, '_, '_, '_, '_>> for Request<String> {
    fn from(i: &ApiCall) -> Self {
        Req::from(i).into()
    }
}
