use {
    std::{thread, fs},
    std::io::Cursor,
    serde_json::json,
    super::api::{
        Error as ApiError,
    },
    warp::{Filter, Reply, Rejection, http::StatusCode},
    crate::Shared,
    std::path::PathBuf,
    rocket::http::{ContentType, Status},
    rocket::response::Response,
};

macro_rules! json_error_response {
    ($code:expr, $message:expr) => {{
        let json = json!({ "error": $message });
        let reply = warp::reply::json(&json);
        Ok(warp::reply::with_status(reply, $code))
    }};
}

pub struct Server {
    shared: Shared,
}

impl Server {
    pub fn new(shared: Shared) -> Self {
        Self { shared }
    }

    pub fn start(&mut self) {
        let shared = self.shared.clone();
        thread::spawn(|| server(shared));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hls/<name>/<file_name>")]
fn hls<'r>(name: String, file_name: String) -> Response<'r> {
    let path = format!("./tmp/stream/{}/{}", name, file_name);
    let filepath = PathBuf::from(path);

    match fs::read(&filepath) {
        Ok(file) => {
            let response = Response::build()
                .status(Status::Ok)
                .header(ContentType::new("application", "x-mpegURL"))
                .raw_header("Accept-Ranges", "bytes")
                .raw_header("Connection", "keep-alive")
                .sized_body(Cursor::new(file))
                .finalize();
            response
        },
        Err(e) => {
            println!("{}", e);
            let response = Response::build()
                .status(Status::Ok)
                .header(ContentType::Plain)
                .sized_body(Cursor::new("Error"))
                .finalize();
            response
        }
    }
}

// Contains some attempt codes
fn server(shared: Shared) {
    println!("http://localhost:8000/");
    rocket::ignite()
        .mount("/", routes![index, hls])
        .launch();
}

fn error_handler(err: Rejection) -> Result<impl Reply, Rejection> {
    match err.find_cause() {
        | Some(e @ ApiError::NoSuchResource)
        | Some(e @ ApiError::StreamNotFound) => {
            json_error_response!(StatusCode::NOT_FOUND, e.to_string())
        },
        None => Err(err)
    }
}
