
// src/events.rs

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

/// Defines all possible application-specific events.
/// Each variant can hold the data that will be serialized as the event payload.
#[derive(Debug)] // Debug is useful for logging
pub enum AppEventType {
    // Simple events with no specific data beyond their name
    TalentPoolRefreshed,
    MarketFluctuation,
    WeekendArrived,


    // Events with specific, relevant data
    NewProjectStarted {
        project_id: u32,
        project_name: String,
    },
    EmployeeHired {
        employee_id: u32,
        employee_name: String,
        role: String,
    },
    BugFound {
        bug_id: u32,
        description: String,
        severity: String,
    },
    GeneralNotification {
        message: String,
        severity: String, // e.g., "info", "warning", "error"
    },
    // Add more event types as your game evolves
}

/// A generic struct for events that don't need a specific payload beyond their name.
/// Serde will serialize this as an empty JSON object: `{}`
#[derive(Serialize, Clone)]
struct EmptyPayload;

/// A wrapper function to emit application-specific events to the frontend.
/// This handles mapping the `AppEventType` to a string name and a serializable payload.
pub fn emit_app_event(app_handle: &AppHandle, event_type: AppEventType) {
    let (event_name, payload_json) = match event_type {
        AppEventType::TalentPoolRefreshed => {
            ("talent_pool_refreshed", serde_json::to_value(EmptyPayload).unwrap())
        },
        AppEventType::MarketFluctuation => {
            ("market_fluctuation", serde_json::to_value(EmptyPayload).unwrap())
        },
        AppEventType::WeekendArrived => {
            ("weekend_arrived", serde_json::to_value(EmptyPayload).unwrap())
        },
        AppEventType::NewProjectStarted { project_id, project_name } => {
            #[derive(Serialize, Clone)]
            struct Payload { project_id: u32, project_name: String }
            ("new_project_started", serde_json::to_value(Payload { project_id, project_name }).unwrap())
        },
        AppEventType::EmployeeHired { employee_id, employee_name, role } => {
            #[derive(Serialize, Clone)]
            struct Payload { employee_id: u32, employee_name: String, role: String }
            ("employee_hired", serde_json::to_value(Payload { employee_id, employee_name, role }).unwrap())
        },
        AppEventType::BugFound { bug_id, description, severity } => {
            #[derive(Serialize, Clone)]
            struct Payload { bug_id: u32, description: String, severity: String }
            ("bug_found", serde_json::to_value(Payload { bug_id, description, severity }).unwrap())
        },
        AppEventType::GeneralNotification { message, severity } => {
            #[derive(Serialize, Clone)]
            struct Payload { message: String, severity: String }
            ("general_notification", serde_json::to_value(Payload { message, severity }).unwrap())
        },
        // Add more match arms for new event types
    };

    app_handle.emit(event_name, payload_json)
        .unwrap_or_else(|e| {
            eprintln!("Failed to emit event '{}': {:?}", event_name, e);
        });
}