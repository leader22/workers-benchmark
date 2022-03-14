use worker::*;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    // See https://rustwasm.github.io/book/reference/code-size.html#avoid-string-formatting
    // let res = format!("Hello worker! at {}", req.url()?);
    let res = String::from("Hello worker! at ") + &req.url()?.to_string();
    Response::ok(res)
}
