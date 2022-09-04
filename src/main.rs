mod webserver;

/// # Home sensors
/// A project to take data from sensors or pi picos and display them as a website.
///
/// Humidor sensor - dht-22 sensor
/// House plant sensor - capacitive soil moisture via a pi pico.
///
/// ## Update frequency
/// Not sure yet what I want the balence between fidelity, data usage, and power to be.
///
/// ## Website
/// Local host website to begin with. Port forward in time.
///
/// Workflow. Pico/sensor -> csv -> plotters graph -> website.

use std::fs;
use plotters;
use csv;


fn generate_graph(file: String) {
    let data = csv::Reader::from_path(file);

}

fn main() {

}


