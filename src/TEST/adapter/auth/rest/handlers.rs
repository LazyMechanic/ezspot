use crate::TEST::adapter::auth::rest::models::*;
use crate::TEST::adapter::auth::rest::REFRESH_TOKEN_COOKIE_NAME;
use crate::TEST::adapter::rest_prelude::*;
use crate::TEST::port::auth::service as auth_service;
use crate::TEST::port::auth::service::Encode;

use actix_web::http::{header, HeaderValue};
use actix_web::{web, Responder};
use chrono::Utc;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout).service(refresh_tokens);
}

#[actix_web::post("/v1/auth/login")]
async fn login(state: web::Data<State>, req: web::Json<LoginRequest>) -> ApiResult {
    let login_req = auth_service::LoginRequest {
        fingerprint: req.0.fingerprint,
        room_id: req.0.room_id,
        room_password: req.0.room_password,
    };

    let login_res = state
        .auth_service
        .login(login_req)
        .await
        .map_err(|err| err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let access_token = login_res
        .jwt
        .access_token
        .encode(state.auth_service.secret())
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;

    let refresh_token = login_res
        .jwt
        .refresh_token
        .encode(state.auth_service.secret())
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;

    let login_res_json = LoginResponse { access_token };

    let res = {
        let mut res = HttpResponse::Ok().json(login_res_json);
        let h = res.headers_mut();

        HeaderValue::from_str(
            &cookie::Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_token)
                .path("/api/v1/auth")
                .http_only(true)
                .max_age(time::Duration::seconds(
                    login_res.jwt.refresh_token.exp().timestamp() - Utc::now().timestamp(),
                ))
                .finish()
                .to_string(),
        )
        .map(|c| {
            h.append(header::SET_COOKIE, c);
        })
        .map_err(err_with_internal_error)?;

        res
    };

    Ok(res)
}

#[actix_web::post("/v1/auth/logout")]
async fn logout(state: web::Data<State>, jwt: Jwt) -> ApiResult {
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::post("/v1/auth/refresh-tokens")]
async fn refresh_tokens(
    state: web::Data<State>,
    jwt: Jwt,
    req: web::Query<RefreshTokensRequest>,
) -> ApiResult {
    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST::adapter::test_utils::*;

    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_login() -> anyhow::Result<()> {
        let mut app = init_app! {
            services: [login]
        };

        let req = test::TestRequest::post()
            .uri("/v1/auth/login")
            .set_json(&LoginRequest {
                fingerprint: "fingerprint".to_string(),
                room_id: 0,
                room_password: "".to_string(),
            })
            .to_request();

        let resp = actix_web::test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK, "status code");

        let resp_body: CreateResponse = {
            let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
            serde_json::from_value(resp_body_json)?
        };

        let exp_resp_body = CreateResponse {
            entry: Entry {
                id: Default::default(),
                title: "123".to_string(),
                payload: Payload {
                    kind: "A".to_string(),
                    value: serde_json::json! {
                        {
                            "field_str": "str",
                            "field_int": 42
                        }
                    },
                },
            },
        };

        assert_eq!(resp_body.entry.title, exp_resp_body.entry.title, "title");
        assert_eq!(
            resp_body.entry.payload, exp_resp_body.entry.payload,
            "payload"
        );

        Ok(())
    }
}
