#[macro_use]
extern crate rocket;
use rocket::{form::Form, fs::NamedFile};

#[derive(FromForm, Debug)]
struct form {
    submitter_name: String,
    submitter_email: String,
    submitter_phone: String,
    lo_name: String,
    lo_email: String,
    lo_phone: String,
    lo_address: String,
    message: String,
}

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open("website/index.html").await.unwrap()
}

#[get("/index.js")]
async fn index_js() -> NamedFile {
    NamedFile::open("website/index.js").await.unwrap()
}

#[post("/", data = "<form>")]
async fn index_post(form: Form<form>) -> NamedFile {
    println!("{:?}", form);
    let pool = connect_to_mariadb();
    let pool = futures::executor::block_on(pool).unwrap();
    let query = format!("INSERT INTO helene (submitter_name, submitter_email, submitter_phone, lo_name, lo_email, lo_phone, lo_address, message) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')", form.submitter_name, form.submitter_email, form.submitter_phone, form.lo_name, form.lo_email, form.lo_phone, form.lo_address, form.message);
    let query = sqlx::query(&query);
    let query = query.execute(&pool);
    let query = futures::executor::block_on(query).unwrap();
    NamedFile::open("website/success.html").await.unwrap()
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index, index_post, index_js])
        .launch()
        .await
        .unwrap();
}

async fn connect_to_mariadb() -> sqlx::Result<sqlx::Pool<sqlx::MySql>> {
    sqlx::MySqlPool::connect("mysql://root:AAUUSStinhh1124*@10.0.0.11:3306/helene").await
}
