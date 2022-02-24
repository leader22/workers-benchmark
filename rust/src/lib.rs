use worker::*;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    Response::ok(format!("Hello worker! at {}", req.url()?))
}
