#[macro_use]
extern crate rocket;

use std::process::Command;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, version, execute])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/version")]
fn version() -> String {
    String::from_utf8(
        Command::new("oxido")
            .arg("--version")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .split(' ')
    .collect::<Vec<&str>>()[1]
        .to_string()
}

#[get("/execute/<code>")]
fn execute(code: &str) -> String {
    let code = String::from_utf8(URL_SAFE.decode(code).unwrap()).unwrap();

    String::from_utf8(
        Command::new("oxido")
            .arg("--code")
            .arg(code)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
}
