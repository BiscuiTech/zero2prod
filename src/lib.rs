pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
