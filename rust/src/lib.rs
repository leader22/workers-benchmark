use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |req, _ctx| {
            Response::ok(format!("Hello worker! at {}", req.url()?))
        })
        .run(req, env)
        .await
}
