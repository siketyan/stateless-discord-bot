mod context;
mod discord;
mod error;
mod http;
mod utils;

use worker::*;

use crate::context::Context;
use crate::http::{HttpRequest, HttpResponse, HttpStatus};

impl HttpRequest {
    async fn from_worker(mut request: Request) -> Result<Self> {
        Ok(HttpRequest {
            headers: request.headers().entries().collect(),
            body: request.text().await?,
        })
    }
}

impl From<HttpResponse> for Result<Response> {
    fn from(response: HttpResponse) -> Self {
        match response.status {
            HttpStatus::Ok => Response::ok(response.body),
            _ => Response::error(response.body, response.status as u16),
        }
    }
}

#[event(fetch)]
pub async fn main(request: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::set_panic_hook();

    Context {
        env: vec![("PUBLIC_KEY".to_owned(), env.var("PUBLIC_KEY")?.to_string())]
            .into_iter()
            .collect(),
        request: HttpRequest::from_worker(request).await?,
    }
    .handle_http_request()
    .into()
}
