import {derived, writable} from 'svelte/store';
import type {PersonSnapshot} from './persons';

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


export const activeView = writable('Dashboard');


//UI state
export const teamExpandedState = writable<Record<number, boolean>>({});
export function toggleTeamExpanded(teamId: number) {
    teamExpandedState.update(state => ({
        ...state,
        [teamId]: !state[teamId]
    }));
}

export const DEFAULT_TEAM_EXPANDED_STATE = true;