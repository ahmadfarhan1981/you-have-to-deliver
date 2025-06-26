// src/events.rs

use serde::Serialize;
use tauri::{AppHandle, Emitter};

// --- 1. Define the Single, Generic UI Notification Payload Struct ---
// This struct remains the same, as it's the standard format for the frontend.
#[derive(Debug, Serialize, Clone)]
pub struct UINotificationPayload {
    // Optional ID relevant to the context of the event (e.g., project_id, employee_id)
    #[serde(skip_serializing_if = "Option::is_none")] // Don't serialize if None
    pub context_id: Option<String>,
    // Category or severity of the notification (e.g., "info", "success", "warning", "error")
    pub notification_type: String,
    pub title: String,
    pub message: String,
}

// --- 2. Define Your AppEventType Enum (Simplified for Caller) ---
// Variants now represent *what happened*, and optionally carry minimal data
// needed to *construct* the UI message later.
#[derive(Debug)]
pub enum AppEventType {
    //Actually Used
    InitDone,

    // Events that need no extra data from the caller to form their UI message
    TalentPoolRefreshed,
    MarketFluctuation,
    WeekendArrived,
    QuarterlyReportGenerated, // Example of a new event

    // Events that need specific data from the caller to form their UI message
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
        description: String, // Description provided by the system
        severity: String,    // Severity provided by the system (e.g., "Critical", "Major")
    },
    // A fallback for truly ad-hoc/custom notifications, if needed.
    // The caller explicitly provides the full notification details here.
    ShowCustomNotification {
        notification_type: String,
        title: String,
        message: String,
        context_id: Option<String>,
    },
    // Add more event types here
}

// --- 3. The Wrapper Function That Constructs the Payload ---
// This function maps the AppEventType to its string name and constructs the UINotificationPayload.
pub fn emit_app_event(app_handle: &AppHandle, event_type: AppEventType) {
    let (event_name, payload) = match event_type {
        AppEventType::TalentPoolRefreshed => {
            ("talent_pool_refreshed", UINotificationPayload {
                context_id: None,
                notification_type: "info".to_string(),
                title: "Talent Pool Refreshed".to_string(),
                message: "New candidates are available for hiring!".to_string(),
            })
        },
        AppEventType::MarketFluctuation => {
            ("market_fluctuation", UINotificationPayload {
                context_id: None,
                notification_type: "warning".to_string(),
                title: "Market Shift".to_string(),
                message: "Market conditions have changed! Review strategies.".to_string(),
            })
        },
        AppEventType::WeekendArrived => {
            ("weekend_arrived", UINotificationPayload {
                context_id: None,
                notification_type: "info".to_string(),
                title: "Weekend!".to_string(),
                message: "Time for a break, or maybe some crunch...".to_string(),
            })
        },
        AppEventType::QuarterlyReportGenerated => {
            ("quarterly_report_generated", UINotificationPayload {
                context_id: None,
                notification_type: "success".to_string(),
                title: "Quarterly Report".to_string(),
                message: "The latest quarterly report has been generated.".to_string(),
            })
        },
        AppEventType::NewProjectStarted { project_id, project_name } => {
            ("new_project_started", UINotificationPayload {
                context_id: Some(project_id.to_string()),
                notification_type: "success".to_string(),
                title: "Project Started".to_string(),
                message: format!("New project '{}' (ID: {}) has commenced!", project_name, project_id),
            })
        },
        AppEventType::EmployeeHired { employee_id, employee_name, role } => {
            ("employee_hired", UINotificationPayload {
                context_id: Some(employee_id.to_string()),
                notification_type: "success".to_string(),
                title: "Employee Hired".to_string(),
                message: format!("{} has joined as a {}!", employee_name, role),
            })
        },
        AppEventType::BugFound { bug_id, description, severity } => {
            ("bug_found", UINotificationPayload {
                context_id: Some(bug_id.to_string()),
                notification_type: match severity.as_str() { // Map severity to notification_type
                    "Critical" | "Blocker" => "error".to_string(),
                    "Major" => "warning".to_string(),
                    _ => "info".to_string(), // Minor, Trivial, etc.
                },
                title: format!("Bug Found: {}", severity),
                message: description,
            })
        },
        AppEventType::ShowCustomNotification { notification_type, title, message, context_id } => {
            // This variant directly uses the provided details for the payload
            ("custom_notification", UINotificationPayload {
                context_id,
                notification_type,
                title,
                message,
            })
        },
        // Add more match arms for new event types.
        // Each arm defines its specific event name and constructs its UINotificationPayload.
        AppEventType::InitDone => {("init_done", UINotificationPayload {
            context_id: None,
            notification_type: "info".to_string(),
            title: "Game initialization finished.".to_string(),
            message: "New game initialization finished. Ready to start".to_string(),
        })

        }
    };

    // `payload` is now always UINotificationPayload, which implements Serialize + Clone.
    app_handle.emit(event_name, payload)
        .unwrap_or_else(|e| {
            eprintln!("Failed to emit event '{}': {:?}", event_name, e);
        });
}



