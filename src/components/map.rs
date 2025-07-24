use dioxus::prelude::*;
use crate::models::TelemetryData;

#[component]
pub fn MapComponent(telemetry_data: TelemetryData) -> Element {
    rsx! {
        div { class: "map-container",
            div { class: "map-grid" }
            
            div { 
                class: "usv-marker",
                style: "left: 50%; top: 50%; transform: translate(-50%, -50%) rotate({telemetry_data.velocity.heading}deg);"
            }
            
            div { class: "usv-trail" }
            
            div { class: "coordinates",
                div { "Lat: {telemetry_data.position.latitude:.6}" }
                div { "Lon: {telemetry_data.position.longitude:.6}" }
                div { "Alt: {telemetry_data.position.altitude:.1}m" }
            }
            
            div { class: "compass",
                "{telemetry_data.velocity.heading:.0}°"
            }
            
            div { class: "speed-indicator",
                "Speed: {telemetry_data.velocity.speed:.1} kn"
            }
        }
    }
}