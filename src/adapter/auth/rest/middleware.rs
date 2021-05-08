use crate::adapter::auth::rest::models::*;
use crate::adapter::auth::rest::ACCESS_TOKEN_HEADER_NAME;
use crate::adapter::auth::rest::ACCESS_TOKEN_PREFIX;
use crate::adapter::auth::rest::REFRESH_TOKEN_COOKIE_NAME;
use crate::adapter::rest_prelude::*;
use crate::port::auth::service as auth_service;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, Error as ActixError, HttpMessage};
use futures::future::LocalBoxFuture;
use futures::{future, FutureExt};
use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::task;

struct Inner {
    exclude_fn: Option<Box<dyn Fn(&ServiceRequest) -> bool>>,
    exclude: HashMap<String, Option<HashSet<http::Method>>>,
    exclude_regex: Vec<ExcludeRegexRule>,
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtAuth(Rc<Inner>);

impl Default for JwtAuth {
    fn default() -> Self {
        Self(Rc::new(Inner {
            exclude_fn: None,
            exclude: Default::default(),
            exclude_regex: vec![],
        }))
    }
}

pub struct ExcludeRule {
    pub path: String,
    pub methods: Option<HashSet<http::Method>>,
}

impl From<String> for ExcludeRule {
    fn from(f: String) -> Self {
        Self {
            path: f,
            methods: Default::default(),
        }
    }
}

impl<'a> From<&'a str> for ExcludeRule {
    fn from(f: &'a str) -> Self {
        Self {
            path: f.to_owned(),
            methods: Default::default(),
        }
    }
}

impl<S, M> From<(S, M)> for ExcludeRule
where
    S: Into<String>,
    M: Into<http::Method>,
{
    fn from(f: (S, M)) -> Self {
        let mut methods = HashSet::new();
        methods.insert(f.1.into());
        Self {
            path: f.0.into(),
            methods: Some(methods),
        }
    }
}

pub struct ExcludeRegexRule {
    pub path: Regex,
    pub methods: Option<HashSet<http::Method>>,
}

impl From<String> for ExcludeRegexRule {
    fn from(f: String) -> Self {
        Self {
            path: Regex::new(&f).unwrap(),
            methods: Default::default(),
        }
    }
}

impl<'a> From<&'a str> for ExcludeRegexRule {
    fn from(f: &'a str) -> Self {
        Self {
            path: Regex::new(f).unwrap(),
            methods: Default::default(),
        }
    }
}

impl<'a> From<(&'a str, http::Method)> for ExcludeRegexRule {
    fn from(f: (&'a str, http::Method)) -> Self {
        let mut methods = HashSet::new();
        methods.insert(f.1);
        Self {
            path: Regex::new(f.0).unwrap(),
            methods: Some(methods),
        }
    }
}

impl From<(String, http::Method)> for ExcludeRegexRule {
    fn from(f: (String, http::Method)) -> Self {
        let mut methods = HashSet::new();
        methods.insert(f.1);
        Self {
            path: Regex::new(&f.0).unwrap(),
            methods: Some(methods),
        }
    }
}

impl From<(Regex, http::Method)> for ExcludeRegexRule {
    fn from(f: (Regex, http::Method)) -> Self {
        let mut methods = HashSet::new();
        methods.insert(f.1);
        Self {
            path: f.0,
            methods: Some(methods),
        }
    }
}

impl JwtAuth {
    /// Ignore and do not check if `f` returns true
    pub fn exclude_fn(mut self, f: Box<dyn Fn(&ServiceRequest) -> bool>) -> Self {
        Rc::get_mut(&mut self.0).unwrap().exclude_fn = Some(f);
        self
    }

    /// Ignore and do not check auth for specified path.
    pub fn exclude<T>(mut self, rule: T) -> Self
    where
        T: Into<ExcludeRule>,
    {
        let rule = rule.into();
        Rc::get_mut(&mut self.0)
            .unwrap()
            .exclude
            .insert(rule.path, rule.methods);
        self
    }

    /// Ignore and do not check auth for paths that match regex
    pub fn exclude_regex<T>(mut self, rule: T) -> Self
    where
        T: Into<ExcludeRegexRule>,
    {
        Rc::get_mut(&mut self.0)
            .unwrap()
            .exclude_regex
            .push(rule.into());
        self
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for JwtAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(JwtAuthMiddleware {
            inner: Rc::clone(&self.0),
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct JwtAuthMiddleware<S> {
    inner: Rc<Inner>,
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for JwtAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let exclude = {
            let exclude_fn_res = match &self.inner.exclude_fn {
                Some(f) => f(&req),
                None => false,
            };

            let exclude_res = match self.inner.exclude.get(req.path()) {
                None => false,
                Some(None) => true,
                Some(Some(methods)) => methods.contains(req.method()),
            };
            let exclude_regexp_res = self.inner.exclude_regex.iter().fold(false, |acc, rule| {
                let match_path = rule.path.is_match(req.path());
                let match_method = match &rule.methods {
                    None => true,
                    Some(methods) => methods.contains(req.method()),
                };

                let m = match_path && match_method;
                acc || m
            });

            exclude_fn_res || exclude_res || exclude_regexp_res
        };

        if !exclude {
            let service = Rc::clone(&self.service);
            async move {
                auth(&req).await?;
                service.borrow_mut().call(req).await
            }
            .boxed_local()
        } else {
            self.service.call(req).boxed_local()
        }
    }
}

async fn auth(req: &ServiceRequest) -> Result<(), ActixError> {
    let ctx = req.app_data::<web::Data<State>>().expect("no state");

    let auth_service = &ctx.auth_service;

    // Get access token from header
    let access_token_encoded = req
        .headers()
        .get(ACCESS_TOKEN_HEADER_NAME)
        .ok_or_else(|| {
            actix_web::error::ErrorUnauthorized(format!(
                r#""{}" header not found"#,
                ACCESS_TOKEN_HEADER_NAME
            ))
        })?
        .to_str()
        .map_err(actix_web::error::ErrorBadRequest)?
        .trim_start_matches(ACCESS_TOKEN_PREFIX);

    // Get refresh token from cookie
    let refresh_token_encoded = req.cookie(REFRESH_TOKEN_COOKIE_NAME).ok_or_else(|| {
        actix_web::error::ErrorUnauthorized(format!(
            r#""{}" cookie not found"#,
            REFRESH_TOKEN_COOKIE_NAME
        ))
    })?;

    // Decode access token
    let access_token_decoded =
        AccessTokenDecoded::decode(auth_service.secret(), access_token_encoded)
            .map_err(actix_web::error::ErrorBadRequest)?;

    // Decode refresh token
    let refresh_token_decoded =
        RefreshTokenDecoded::decode(auth_service.secret(), refresh_token_encoded.value())
            .map_err(actix_web::error::ErrorBadRequest)?;

    // Authorize
    let auth_req = auth_service::AuthorizeRequest {
        jwt: auth_service::Jwt {
            access_token: access_token_decoded.into(),
            refresh_token: refresh_token_decoded.into(),
        },
    };
    let auth_res = auth_service
        .authorize(auth_req)
        .await
        .map_err(actix_web::error::ErrorUnauthorized)?;

    let jwt: Jwt = auth_res.jwt.into();

    req.extensions_mut().insert(jwt);

    Ok(())
}
