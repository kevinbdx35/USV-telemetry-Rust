use dioxus::prelude::*;

#[component]
pub fn CameraComponent() -> Element {
    rsx! {
        div { class: "camera-container",
            div { class: "camera-grid" }
            
            div { class: "camera-status", "HD • 30fps" }
            div { class: "record-indicator" }
            
            div { class: "camera-placeholder",
                div { 
                    style: "font-size: 48px; margin-bottom: 10px;",
                    "📹"
                }
                div { "Camera Feed" }
                div { 
                    style: "font-size: 12px; color: #999; margin-top: 10px;",
                    "Waiting for video stream..."
                }
            }
            
            div { class: "camera-controls",
                button { class: "control-btn", title: "Pan Left", "◀" }
                button { class: "control-btn", title: "Zoom In", "+" }
                button { class: "control-btn", title: "Zoom Out", "-" }
                button { class: "control-btn", title: "Pan Right", "▶" }
            }
        }
    }
}