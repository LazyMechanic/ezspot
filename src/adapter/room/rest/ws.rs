use crate::adapter::rest_prelude::*;
use crate::adapter::room::rest::models::*;

use actix::prelude::*;
use actix_web_actors::ws;

#[derive(Default)]
pub struct WsConn;

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl Handler<FilePart> for WsConn {
    type Result = Result<(), ApiError>;

    fn handle(&mut self, _msg: FilePart, _ctx: &mut <Self as Actor>::Context) -> Self::Result {
        Ok(())
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut <Self as Actor>::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
