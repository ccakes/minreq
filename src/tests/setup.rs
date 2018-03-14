use std::thread;
use std::sync::{Once, ONCE_INIT};
use tiny_http::{Method, Response, Server};

static INIT: Once = ONCE_INIT;

/// This is a simple server that serves the following endpoints:
/// - `GET /boop`
///   This is just for testing simple GETs, returns "beep".
/// - `PUT /insert`
///   Inserts the request body into a list.
/// - `POST /clear`
///   Clears the list filled with `/insert`s.
/// - `GET /list`
///   Returns the list operated by `POST /clear` and `PUT /insert`
pub fn setup() {
    INIT.call_once(|| {
        let server = Server::http("0.0.0.0:35562").unwrap();
        thread::spawn(move || {
            let mut list = Vec::new();
            for mut request in server.incoming_requests() {
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content).ok();

                let url = String::from(request.url());
                match request.method() {
                    &Method::Get if url == "/list" => {
                        request
                            .respond(Response::from_string(format!("{:?}", list)))
                            .ok();
                    }
                    &Method::Delete if url == "/list" => {
                        list.clear();
                        request.respond(Response::from_string("ok")).ok();
                    }
                    &Method::Put if url == "/insert" => {
                        list.push(content);
                        request
                            .respond(Response::from_string("ok").with_status_code(201))
                            .ok();
                    }
                    _ => {
                        request
                            .respond(Response::from_string("Not Found").with_status_code(404))
                            .ok();
                    }
                }
            }
        });
    });
}

pub fn url(req: &str) -> String {
    format!("http://0.0.0.0:35562{}", req)
}
