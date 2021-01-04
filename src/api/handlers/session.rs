use crate::api::handlers::session::responses::CreateSessionResponse;
use crate::api::prelude::*;

pub async fn create_session(ctx: Context) -> ResponseJson {
    let (id, password) = ctx
        .session_service
        .create_session()
        .await
        .map_err(ErrorResponse::with_internal_error)?;

    return Ok(warp::reply::json(&CreateSessionResponse {
        session_id: id,
        session_password: password,
    }));
}

pub mod requests {
    use crate::services::prelude::*;
    use serde::Deserialize;
}

pub mod responses {
    use crate::services::prelude::*;
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct CreateSessionResponse {
        pub session_id: SessionId,
        pub session_password: SessionPassword,
    }
}
