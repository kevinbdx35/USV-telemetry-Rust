use dioxus::prelude::*;
use crate::models::TelemetryData;

#[component]
pub fn TelemetryPanel(data: TelemetryData) -> Element {
    rsx! {
        div { 
            class: "telemetry-panel",
            div { 
                class: "metric-group",
                h3 { "Position" }
                div { 
                    class: "metric-row",
                    span { class: "metric-label", "Latitude" }
                    span { class: "metric-value", "{data.position.latitude:.6}°" }
                }
                div { 
                    class: "metric-row",
                    span { class: "metric-label", "Longitude" }
                    span { class: "metric-value", "{data.position.longitude:.6}°" }
                }
                div { 
                    class: "metric-row",
                    span { class: "metric-label", "Altitude" }
                    span { class: "metric-value", "{data.position.altitude:.1}m" }
                }
            }
            div { 
                class: "metric-group",
                h3 { "Velocity" }
                div { 
                    class: "metric-row",
                    span { class: "metric-label", "Speed" }
                    span { class: "metric-value", "{data.velocity.speed:.1} kn" }
                }
                div { 
                    class: "metric-row",
                    span { class: "metric-label", "Heading" }
                    span { class: "metric-value", "{data.velocity.heading:.0}°" }
                }
            }
            div { 
                class: "metric-group",
                h3 { "Battery" }
                div { 
                    class: "metric-row",
                    span { class: "metric-label", "Level" }
                    span { class: "metric-value", "{data.battery.level}%" }
                }
                div { 
                    class: "metric-row",
                    span { class: "metric-label", "Voltage" }
                    span { class: "metric-value", "{data.battery.voltage:.1}V" }
                }
            }
        }
    }
}