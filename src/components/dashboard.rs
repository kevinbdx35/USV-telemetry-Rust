use dioxus::prelude::*;
use crate::models::TelemetryData;
use crate::components::{MapComponent, CameraComponent, TelemetryPanel, StatusPanel};

#[component]
pub fn Dashboard() -> Element {
    let mut telemetry_data = use_signal(|| TelemetryData::default());
    
    use_effect(move || {
        spawn(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(1000).await;
                telemetry_data.with_mut(|data| {
                    *data = generate_mock_data();
                });
            }
        });
    });

    rsx! {
        div { class: "dashboard",
            div { class: "main-view",
                div { class: "card",
                    div { class: "card-header", "USV Tracking Map" }
                    div { class: "card-content",
                        MapComponent { telemetry_data: telemetry_data.read().clone() }
                    }
                }
                div { class: "card",
                    div { class: "card-header", "Camera Feed" }
                    div { class: "card-content",
                        CameraComponent {}
                    }
                }
            }

            div { class: "sidebar",
                div { class: "card",
                    div { class: "card-header", "Telemetry Data" }
                    div { class: "card-content",
                        TelemetryPanel { data: telemetry_data.read().clone() }
                    }
                }
                div { class: "card",
                    div { class: "card-header", "System Status" }
                    div { class: "card-content",
                        StatusPanel { data: telemetry_data.read().clone() }
                    }
                }
            }
        }
    }
}

fn generate_mock_data() -> TelemetryData {
    use chrono::Utc;
    use uuid::Uuid;
    use crate::models::*;
    
    let mut data = TelemetryData::new();
    data.id = Uuid::new_v4();
    data.timestamp = Utc::now();
    
    data.position.latitude += (js_sys::Math::random() - 0.5) * 0.001;
    data.position.longitude += (js_sys::Math::random() - 0.5) * 0.001;
    data.velocity.speed = js_sys::Math::random() * 20.0;
    data.velocity.heading = js_sys::Math::random() * 360.0;
    
    data.battery.level = (js_sys::Math::random() * 100.0) as u8;
    data.engine.rpm = (js_sys::Math::random() * 3000.0) as u16;
    data.sensors.water_temperature = (15.0 + js_sys::Math::random() * 15.0) as f32;
    
    data
}