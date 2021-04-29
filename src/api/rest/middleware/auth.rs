use crate::api::rest::prelude::*;
use crate::services::auth::AuthService;

use actix_web::dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error as ActixError, Error, FromRequest, HttpMessage};
use futures::future::LocalBoxFuture;
use futures::{future, Future, FutureExt};
use regex::RegexSet;
use std::cell::RefCell;
use std::collections::HashSet;
use std::pin::Pin;
use std::rc::Rc;
use std::task;

struct Inner {
    exclude: HashSet<String>,
    exclude_regex: RegexSet,
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtAuth(Rc<Inner>);

impl Default for JwtAuth {
    fn default() -> Self {
        Self(Rc::new(Inner {
            exclude: Default::default(),
            exclude_regex: RegexSet::empty(),
        }))
    }
}

impl JwtAuth {
    /// Ignore and do not check auth for specified path.
    pub fn exclude<T: Into<String>>(mut self, path: T) -> Self {
        Rc::get_mut(&mut self.0)
            .unwrap()
            .exclude
            .insert(path.into());
        self
    }

    /// Ignore and do not check auth for paths that match regex
    pub fn exclude_regex<T: Into<String>>(mut self, path: T) -> Self {
        let inner = Rc::get_mut(&mut self.0).unwrap();
        let mut patterns = inner.exclude_regex.patterns().to_vec();
        patterns.push(path.into());
        let regex_set = RegexSet::new(patterns).unwrap();
        inner.exclude_regex = regex_set;
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
        let check_auth = !(self.inner.exclude.contains(req.path())
            || self.inner.exclude_regex.is_match(req.path()));

        if check_auth {
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
    let ctx = req
        .app_data::<Context>()
        .map(|data| data.clone())
        .expect("no context");

    let auth_service = ctx.auth_service;

    let access_token_encoded = {
        let header_name = auth_service.access_token_header_name();
        req.headers()
            .get(header_name)
            .ok_or_else(|| {
                actix_web::error::ErrorUnauthorized(format!(
                    r#""{}" header not found"#,
                    header_name
                ))
            })?
            .to_str()
            .map_err(actix_web::error::ErrorBadRequest)?
    };

    let refresh_token_encoded = {
        let cookie_name = auth_service.refresh_token_cookie_name();
        req.cookie(cookie_name).ok_or_else(|| {
            actix_web::error::ErrorUnauthorized(format!(r#""{}" cookie not found"#, cookie_name))
        })?
    };

    let jwt: Jwt = auth_service
        .authorize(access_token_encoded, refresh_token_encoded.value())
        .await
        .map_err(actix_web::error::ErrorUnauthorized)?
        .into();

    req.extensions_mut().insert(jwt);

    Ok(())
}

#[derive(Clone)]
pub struct Jwt {}

impl From<domain_models::auth::Jwt> for Jwt {
    fn from(jwt: domain_models::auth::Jwt) -> Self {
        todo!()
    }
}

impl FromRequest for Jwt {
    type Error = ActixError;
    type Future = future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(jwt) = req.extensions().get::<Jwt>() {
            future::ok(jwt.clone())
        } else {
            future::err(actix_web::error::ErrorBadRequest("JWT not found"))
        }
    }
}
