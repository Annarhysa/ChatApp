#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str{
  "Hello, world!"
}

#[launch]
fn rocket() -> _{
  rocket::build(): Rocket<Build>.mount(base: "/hello", routes :routes![world])

}
  