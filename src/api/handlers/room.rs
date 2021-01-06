use crate::api::prelude::*;

pub async fn create_room(ctx: Context) -> ResponseJson {
    let (id, password) = ctx
        .room_service
        .create_session()
        .await
        .map_err(ErrorResponse::err_with_internal_error)?;

    return Ok(warp::reply::json(&responses::CreateRoomResponse {
        session_id: id,
        session_password: password,
    }));
}

pub mod requests {}

pub mod responses {
    use crate::services::prelude::*;
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct CreateRoomResponse {
        pub session_id: RoomId,
        pub session_password: RoomPassword,
    }
}
