use crate::api::rest::prelude::*;
use crate::models::auth::WebSocketTicketDecoded;

pub async fn handle_connection(
    ws: warp::ws::Ws,
    ctx: Context,
    ws_ticket: WebSocketTicketDecoded,
) -> CustomResponse<impl warp::Reply> {
    Ok(ws.on_upgrade(|ws| async move {}))
}
