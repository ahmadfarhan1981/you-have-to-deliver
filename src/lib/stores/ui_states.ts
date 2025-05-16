
import {derived, type Writable, writable} from 'svelte/store';
import type { PersonSnapshot } from './employees';

export const selectedEmployee = writable<PersonSnapshot | null>(null);
export type TabType = 'system' | 'employee';

export type Tab =
    | {
    id: string;
    title: string;
    type: 'system';
    isActive: boolean;
}
    | {
    id: string;
    title: string;
    type: 'employee';
    isActive: boolean;
    context: {
        employeeId: number;
    };
};

export const tabs = writable<Tab[]>([
    { id: 'overview', title: 'Overview', type: 'system', isActive: true },
    { id: 'reports', title: 'Reports', type: 'system', isActive: false },
]);
export const activeTabId = derived(tabs, $tabs => $tabs.find(t => t.isActive)?.id ?? null);

export const nextTabId = writable(1);

export const expandedTeams = writable({
    "Development Team": true,
    "QA Team": false,
    "Business Analyst Team": false,
    "Management Team": false
});

export const activeView = writable('Dashboard');