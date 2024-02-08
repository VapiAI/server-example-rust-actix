use crate::api::custom_llm::basic;
use crate::api::custom_llm::openai_advanced;
use crate::api::custom_llm::openai_sse;
use crate::api::functions::basic as basic_functions;
use crate::api::functions::rag;
use crate::api::inbound;
use crate::api::outbound;
use crate::api::webhook;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/inbound").route(web::post().to(inbound::inbound)))
            .service(web::resource("/outbound").route(web::post().to(outbound::outbound)))
            .service(
                web::scope("/functions")
                    .service(web::resource("/basic").route(web::post().to(basic_functions::basic)))
                    .service(web::resource("/rag").route(web::post().to(rag::rag))),
            )
            .service(
                web::scope("/custom-llm")
                    .service(
                        web::resource("/basic/chat/completions")
                            .route(web::post().to(basic::basic)),
                    )
                    .service(
                        web::resource("/openai-sse/chat/completions")
                            .route(web::post().to(openai_sse::openai_sse)),
                    )
                    .service(
                        web::resource("/openai-advanced/chat/completions")
                            .route(web::post().to(openai_advanced::openai_advanced)),
                    ),
            )
            .service(
                web::scope("/webhook")
                    .service(web::resource("/").route(web::post().to(webhook::index::webhook))),
            ),
    );
}
