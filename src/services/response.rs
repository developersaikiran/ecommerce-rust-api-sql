use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ResponseInterface<T, E> {
    pub status: i32,
    pub success: bool,
    pub message: String,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<E>, // Make err an Option<T>
}

impl<T, E> ResponseInterface<T, E> {
    pub fn new(status: i32, success: bool, message: &str, data: T, err: Option<E>) -> Self {
        ResponseInterface {
            status,
            success,
            message: message.to_string(),
            data,
            err,
        }
    }
}

pub fn success_response<T>(data: T, message: &str) -> ResponseInterface<T, ()> {
    ResponseInterface::new(200, true, message, data, None)
}

pub fn server_error_response<T, E>(data: T, message: &str, err: E) -> ResponseInterface<T, E> {
    ResponseInterface::new(500, false, message, data, Some(err))
}

pub fn bad_request_response<T, E>(data: T, message: &str, err: E) -> ResponseInterface<T, E> {
    ResponseInterface::new(400, false, message, data, Some(err))
}