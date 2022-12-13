use std::env;
use std::fs::File;
use std::io::BufReader;
use image::ImageFormat;

use plotters::backend::{BitMapBackend, DrawingBackend};
use plotters::element::BackendCoordAndZ;
use plotters::prelude::{BitMapElement, IntoTextStyle};
use plotters::style::{ RED, TextStyle};
use rand::Rng;
use serde_json::{json, json_internal};
use tracing::info;
use RcoploBot::core::network::Ws;
use RcoploBot::service::CONTEXT;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting rbatis...");
    CONTEXT.init_pool().await;
    info!("Starting client...");
    Ws::run().await;
    // sign_image();
}

