use dioxus::prelude::*;
use crate::models::TelemetryData;

#[component]
pub fn StatusPanel(data: TelemetryData) -> Element {
    rsx! {
        div { class: "status-panel",
            div { class: "system-health",
                div { class: "health-score", "98%" }
                div { "System Health" }
            }

            div { 
                class: if data.engine.running { "status-indicator" } else { "status-indicator warning" },
                div { 
                    class: if data.engine.running { "status-dot" } else { "status-dot warning" }
                }
                div { class: "status-text",
                    div { class: "status-title", "Engine Status" }
                    div { class: "status-detail", 
                        if data.engine.running { "Running normally" } else { "Engine stopped" }
                    }
                }
            }

            div { 
                class: if data.battery.level > 20 { "status-indicator" } else { "status-indicator critical" },
                div { 
                    class: if data.battery.level > 20 { "status-dot" } else { "status-dot critical" }
                }
                div { class: "status-text",
                    div { class: "status-title", "Power System" }
                    div { class: "status-detail", 
                        if data.battery.charging { "Charging ({data.battery.level}%)" } 
                        else { "On battery ({data.battery.level}%)" }
                    }
                }
            }

            div { 
                class: if data.communication.connected { "status-indicator" } else { "status-indicator critical" },
                div { 
                    class: if data.communication.connected { "status-dot" } else { "status-dot critical" }
                }
                div { class: "status-text",
                    div { class: "status-title", "Communication" }
                    div { class: "status-detail", 
                        if data.communication.connected { 
                            "{data.communication.network_type} Connected" 
                        } else { 
                            "Connection lost" 
                        }
                    }
                }
            }

            div { class: "communication-status",
                div { class: "signal-bars",
                    div { class: "signal-bar", style: "height: 4px;" }
                    div { class: "signal-bar", style: "height: 6px;" }
                    div { class: "signal-bar", style: "height: 8px;" }
                    div { class: "signal-bar", style: "height: 10px;" }
                }
                span { "Signal: {data.communication.signal_strength} dBm" }
            }

            div { 
                class: "status-indicator",
                div { class: "status-dot" }
                div { class: "status-text",
                    div { class: "status-title", "Navigation" }
                    div { class: "status-detail", "GPS Lock - {data.status}" }
                }
            }

            div { 
                class: if data.sensors.water_temperature > 30.0 { "status-indicator warning" } 
                       else { "status-indicator" },
                div { 
                    class: if data.sensors.water_temperature > 30.0 { "status-dot warning" } 
                           else { "status-dot" }
                }
                div { class: "status-text",
                    div { class: "status-title", "Environmental" }
                    div { class: "status-detail", "All sensors nominal" }
                }
            }
        }
    }
}