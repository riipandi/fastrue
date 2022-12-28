// use axum::{
//     body::Bytes,
//     http::{HeaderMap, Request},
//     response::Response,
// };
// use std::time::Duration;
// use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
// use tracing::Span;

// pub fn logger() {
//     TraceLayer::new_for_http()
//         .on_request(|_request: &Request<_>, _span: &Span| {
//             // println!("message log on_request");
//         })
//         .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
//             // println!("message log on_response");
//         })
//         .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
//             // println!("message log on_body_chunk");
//         })
//         .on_eos(
//             |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
//                 // println!("message log on_eos");
//             },
//         )
//         .on_failure(
//             |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
//                 // println!("message log on_failure");
//             },
//         )
// }
