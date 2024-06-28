use hyper::{Response, Body, header};

pub fn with_cors(response: Response<Body>) -> Response<Body> {
    let (mut parts, body) = response.into_parts();
    parts.headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, header::HeaderValue::from_static("*"));
    parts.headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, header::HeaderValue::from_static("GET, POST, OPTIONS"));
    parts.headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, header::HeaderValue::from_static("Content-Type"));
    Response::from_parts(parts, body)
}
