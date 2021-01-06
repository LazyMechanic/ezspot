pub mod context;
pub mod handlers;
pub mod prelude;
pub mod response;
pub mod routes;

use std::sync::Arc;

use warp::Filter;

use prelude::*;

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

pub async fn start(settings: Arc<Settings>) {
    let ctx = {
        let room_service = Arc::new(RoomService::new(settings.as_ref()));
        let auth_service = Arc::new(AuthService::new(
            settings.as_ref(),
            Arc::clone(&room_service),
        ));
        let ws_service = Arc::new(WebSocketService::new(settings.as_ref()));

        Context {
            room_service,
            auth_service,
            ws_service,
        }
    };

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("Content-Type")
        .allow_header("Authorization")
        .allow_header("Content-Length")
        .allow_header("Content-Disposition")
        .allow_method("GET")
        .allow_method("PUT")
        .allow_method("POST")
        .allow_method("DELETE")
        .allow_method("OPTIONS")
        .build();
    let log = warp::log(APPLICATION_NAME);
    let routes = routes::routes(ctx)
        .recover(ErrorResponse::unpack)
        .with(log)
        .with(cors);

    warp::serve(routes).run(settings.address).await;
}
