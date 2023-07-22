use axum::{
    http::header::{self, HeaderMap, HeaderValue},
    response::IntoResponse,
};
use axum_macros::debug_handler;
use mime::{
    // TEXT_CSS, 
    TEXT_JAVASCRIPT,
};

// const FAVICON: &str = include_str!("../../static/image/favicon.ico");
const HTMX: &str = include_str!("../../static/js/htmx.min.js");
const HTMX_EXT_JSON: &str = include_str!("../../static/js/json-enc.js");
const HYPERSCRIPT: &str = include_str!("../../static/js/hyperscript.min.js");

async fn asset(source: &'static [u8], ty: &'static str) -> impl IntoResponse {
    let mut headermap = HeaderMap::new();
    headermap.insert(header::CONTENT_TYPE, HeaderValue::from_static(ty));
    (headermap, source)
}

// async fn css(source: &'static str) -> impl IntoResponse {
//     asset(source.as_bytes(), TEXT_CSS.as_ref()).await
// }

async fn js(source: &'static str) -> impl IntoResponse {
    asset(source.as_bytes(), TEXT_JAVASCRIPT.as_ref()).await
}


#[debug_handler]
pub async fn htmx_js() -> impl IntoResponse {
    js(HTMX).await
}

pub async fn htmx_ext_json_js() -> impl IntoResponse {
    js(HTMX_EXT_JSON).await
}

pub async fn hyperscript_js() -> impl IntoResponse {
    js(HYPERSCRIPT).await
}

// pub async fn favicon() -> impl IntoResponse {
//     asset(IMG_FAVICON, "image/x-icon").await
// }