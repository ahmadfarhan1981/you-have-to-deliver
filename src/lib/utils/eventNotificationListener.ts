// src/stores/notifications.ts (or your main event store)
import {writable} from 'svelte/store';
import {listen, type UnlistenFn} from '@tauri-apps/api/event';
import {toast} from 'svelte-hot-french-toast';

// Define the single, generic payload interface for the frontend
interface UINotificationPayload {
    context_id?: string; // Optional context ID
    notification_type: string; // e.g., "info", "success", "warning", "error"
    title: string;
    message: string;
}

// You might not even need specific stores for each event type anymore,
// but rather a single "stream" of notifications or directly trigger toasts.
export const latestAppNotification = writable<UINotificationPayload | null>(null);

let unlistenFunctions: UnlistenFn[] = [];

export async function initializeTauriNotificationListeners() {
    // This is a generic listener for ALL events that carry UINotificationPayload
    // We can use a single handler if all events map to the same UI behavior.
    const eventNames = [
        'talent_pool_refreshed',
        'market_fluctuation',
        'weekend_arrived',
        'new_project_started',
        'employee_hired',
        'bug_found',
        'general_notification',
        'init_done'
        // Add all other event names here
    ];

    const toastStyle = {
        style: "background-color: #1e293b; /* bg-slate-800 */\n"+
            "         border-width: 1px;         /* border */\n"+
            "         border-style: solid;       /* border */\n"+
            "         border-color: #334155;     /* border-slate-700 */\n"+
            "         border-radius: 0.25rem;    /* rounded */\n"+
            "         font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace; /* font-mono */\n"+
            "         font-size: 0.875rem;       /* text-sm (14px) */\n"+
            "         line-height: 1.25rem;      /* text-sm (20px) */\n"+
            "         color: white;              /* To ensure text is visible on dark background */\n"+
            "         padding: 0.75rem 1rem;"
    };
    for (const eventName of eventNames) {
        const unlisten = await listen<UINotificationPayload>(eventName, (event) => {
            console.log(`Frontend: Received '${eventName}' event:`, event.payload);
            latestAppNotification.set(event.payload); // Update a general store

            const message =
`${event.payload.title}

${event.payload.message}`
            // Directly trigger a svelte-hot-toast notification
            switch (event.payload.notification_type) {
                case 'success':
                    toast.success(message, toastStyle);
                    break;
                case 'error':
                    toast.error(message, toastStyle);
                    break;
                case 'warning':
                    toast(message, {icon: '⚠️', ...toastStyle}); // Customize icon
                    break;
                case 'info':
                default:
                    toast(message, {icon:'🪧', ...toastStyle});
                    break;
            }
            // You can also access event.payload.context_id here if needed for deeper UI interaction
        });
        unlistenFunctions.push(unlisten);
    }
}

export function cleanupTauriNotificationListeners() {
    unlistenFunctions.forEach(unlisten => unlisten());
    unlistenFunctions = [];
}