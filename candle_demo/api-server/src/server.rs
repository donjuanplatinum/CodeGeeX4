use crate::{api::ChatCompletionRequest, args::Args};
use actix_web::{
    post,
    web::{self, Data},
    App, HttpServer, Responder,
};

use codegeex4::TextGeneration;
use owo_colors::OwoColorize;
use std::sync::{Arc, Mutex};

pub struct Server {
    config: Args,
    data: Arc<Mutex<TextGeneration>>,
}

impl Server {
    pub fn new(config: Args, data: Arc<Mutex<TextGeneration>>) -> Self {
        return Server { config, data };
    }
    pub async fn run(&self) -> () {
        let data = Data::new(self.data.clone());

        HttpServer::new(move || {
            App::new()
                .service(codegeex4_completion)
                .app_data(data.clone())
        })
        .bind(&self.config.address)
        .expect(&format!("{}", "Unable To Bind Server !".red()))
        .workers(self.config.workers)
        .run()
        .await
        .expect(&format!("{}", "Unable To Run the Server !".red()));
    }
}

use super::chat::chat_with_codegeex;
#[post("/v1/chat/completions")]
pub async fn codegeex4_completion(
    request: web::Json<ChatCompletionRequest>,
    pipeline: Data<Arc<Mutex<TextGeneration>>>,
) -> impl Responder {
    println!("Get Request {:?}",request);
    let request = request.into_inner();

    if request.stream == false {
        return web::Json(chat_with_codegeex(pipeline, request).await);
    }
    return web::Json(chat_with_codegeex(pipeline, request).await);
}
