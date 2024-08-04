use crate::api::{ChatCompletionRequest, ChatCompletionResponse, FinishResaon};
use actix_web::web::Data;
use codegeex4::TextGeneration;
use std::sync::{Arc, Mutex};

pub async fn chat_with_codegeex(
    pipeline: Data<Arc<Mutex<TextGeneration>>>,
    request: ChatCompletionRequest,
) -> ChatCompletionResponse {
    let mut prompt = String::new();
    for content in request.messages {
        let content = content.content;
        prompt.push_str(&content);
    }
    let len = request.max_tokens;
    let output = chat(pipeline, &prompt, len)
        .await
        .expect("Failed to gen output");
    let mut response = ChatCompletionResponse::empty();
    response.choices[0].message.content = output;
    response.choices[0].finish_reason = Some(FinishResaon::STOP);
    return response;
}

pub async fn chat(
    pipeline: Data<Arc<Mutex<TextGeneration>>>,
    prompt: &str,
    len: usize,
) -> Result<String, ()> {
    let mut pipeline = pipeline.lock().unwrap();
    return pipeline.run(prompt, len).await;
}
