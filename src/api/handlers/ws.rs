pub use crate::api::prelude::*;

pub async fn handle_connection(
    ws: warp::ws::Ws,
    ws_ticket: WebSocketTicketEntry,
    ctx: Context,
) -> ResponseCustom<impl warp::Reply> {
    Ok(ws.on_upgrade(|ws| async move {}))
}
