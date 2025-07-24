use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Velocity {
    pub speed: f64,
    pub heading: f64,
    pub vertical_speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatteryStatus {
    pub level: u8,
    pub voltage: f32,
    pub current: f32,
    pub temperature: f32,
    pub charging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EngineStatus {
    pub throttle: u8,
    pub rpm: u16,
    pub temperature: f32,
    pub fuel_level: f32,
    pub running: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SensorData {
    pub compass_heading: f64,
    pub pitch: f64,
    pub roll: f64,
    pub yaw: f64,
    pub water_temperature: f32,
    pub air_temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommunicationStatus {
    pub signal_strength: i8,
    pub network_type: String,
    pub connected: bool,
    pub data_usage: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelemetryData {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub position: Position,
    pub velocity: Velocity,
    pub battery: BatteryStatus,
    pub engine: EngineStatus,
    pub sensors: SensorData,
    pub communication: CommunicationStatus,
    pub status: String,
}

impl TelemetryData {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            position: Position {
                latitude: 0.0,
                longitude: 0.0,
                altitude: 0.0,
            },
            velocity: Velocity {
                speed: 0.0,
                heading: 0.0,
                vertical_speed: 0.0,
            },
            battery: BatteryStatus {
                level: 100,
                voltage: 12.0,
                current: 0.0,
                temperature: 25.0,
                charging: false,
            },
            engine: EngineStatus {
                throttle: 0,
                rpm: 0,
                temperature: 25.0,
                fuel_level: 100.0,
                running: false,
            },
            sensors: SensorData {
                compass_heading: 0.0,
                pitch: 0.0,
                roll: 0.0,
                yaw: 0.0,
                water_temperature: 20.0,
                air_temperature: 25.0,
                humidity: 60.0,
                pressure: 1013.25,
            },
            communication: CommunicationStatus {
                signal_strength: -60,
                network_type: "4G".to_string(),
                connected: true,
                data_usage: 0,
            },
            status: "Operational".to_string(),
        }
    }
}

impl Default for TelemetryData {
    fn default() -> Self {
        Self::new()
    }
}