use {
    std::{thread, fs},
    warp::{Filter, Reply, Rejection, http::StatusCode},
    serde_json::json,
    super::api::{
        api,
        Error as ApiError,
    },
    crate::Shared,
    std::path::PathBuf
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
    "Hello, Rocket"
}

#[get("/hls/<name>/playlist.m3u8")]
fn hls(name: String) -> Vec<u8> {
    let path = format!("./tmp/stream/{}/playlist.m3u8", name);
    let filepath = PathBuf::from(path);

    match fs::read(&filepath) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            Vec::new()
        }
    }
}

#[get("/hls/segment/<app_name>/<file_name>")]
fn hls_segment(app_name: String, file_name: String) -> Vec<u8> {
    let path = format!("./tmp/stream/{}/{}", app_name, file_name);
    let filepath = PathBuf::from(path);

    match fs::read(&filepath) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            Vec::new()
        }
    }
}

// Contains some attempt codes
fn server(shared: Shared) {
    let addr = {
        let config = shared.config.read();
        config.web.addr
    };

    let hls_root = {
        let config = shared.config.read();
        config.hls.root_dir.clone()
    };

    println!("{:?}", hls_root);

    println!("http://localhost:8000/");
    rocket::ignite()
        .mount("/", routes![index, hls, hls_segment])
        .launch();

    // let m3u8_file = warp::path("playlist.m3u8");
    // let hls_files = warp::path("hls");
    //     .and(warp::fs::dir(hls_root));

    // let streams_api = warp::path("api")
    //     .and(api(shared));

    // let routes = hls_files
    //     .or(streams_api)
    //     .or(m3u8_file)
    //     .map(|_| {
    //         let filepath = PathBuf::from("./tmp/stream/javv/playlist.m3u8");

    //         match fs::read(&filepath) {
    //             Ok(file) => file,
    //             Err(_) => Vec::new()
    //         }
    //     })
    //     .recover(error_handler);

    // warp::serve(routes).run(addr);
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
