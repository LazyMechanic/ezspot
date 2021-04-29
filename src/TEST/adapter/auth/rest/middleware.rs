use crate::TEST::adapter::auth::rest::models::*;
use crate::TEST::adapter::auth::rest::ACCESS_TOKEN_HEADER_NAME;
use crate::TEST::adapter::auth::rest::ACCESS_TOKEN_PREFIX;
use crate::TEST::adapter::auth::rest::REFRESH_TOKEN_COOKIE_NAME;
use crate::TEST::adapter::rest_prelude::*;
use crate::TEST::port::auth::service as auth_service;
use crate::TEST::port::auth::service::Decode;

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
        .app_data::<State>()
        .map(|data| data.clone())
        .expect("no state");

    let auth_service = ctx.auth_service;

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
        auth_service::AccessTokenDecoded::decode(auth_service.secret(), access_token_encoded)
            .map_err(actix_web::error::ErrorBadRequest)?;

    // Decode refresh token
    let refresh_token_decoded =
        auth_service::RefreshTokenDecoded::decode(auth_service.secret(), access_token_encoded)
            .map_err(actix_web::error::ErrorBadRequest)?;

    // Authorize
    let auth_req = auth_service::AuthorizeRequest {
        jwt: auth_service::Jwt {
            access_token: access_token_decoded,
            refresh_token: refresh_token_decoded,
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
