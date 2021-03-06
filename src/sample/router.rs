use rocket;

use crate::connection;
use crate::sample;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/people",
               routes![
                    sample::handler::post,
                    sample::handler::all

                    ],
        ).launch();
}