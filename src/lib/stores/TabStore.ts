import {derived, get, writable} from 'svelte/store';
import type {PersonSnapshot} from '$lib/stores/persons';

// ─── Types ────────────────────────────────────────────────────────────────

export type SystemTabId = 'overview' | 'reports' | 'projects';
export type TabId = SystemTabId | string;

export type Tab =
    | {
    id: TabId;
    title: string;
    type: 'system';
    isActive: boolean;
}
    | {
    id: TabId;
    title: string;
    type: 'person';
    isActive: boolean;
    context: {
        person: PersonSnapshot;
    };
};

type TabStateData = {
    tabs: Tab[];
};

// ─── Constants ────────────────────────────────────────────────────────────

const predefinedSystemTabs: Record<SystemTabId, Omit<Tab, 'id' | 'isActive'>> = {
    overview: { title: 'Overview', type: 'system' },
    reports: { title: 'Reports', type: 'system' },
    projects: {title: 'Projects', type: 'system'}
};

const ADD_IF_MISSING = true;

// ─── Store Base ───────────────────────────────────────────────────────────

const { subscribe, update } = writable<TabStateData>({
    tabs: [
        { id: 'overview', title: 'Overview', type: 'system', isActive: true },
        { id: 'reports', title: 'Reports', type: 'system', isActive: false },
    ],
});

// ─── Internal Helpers ─────────────────────────────────────────────────────

function personTabId(personId: number): string {
    return `person-${personId}`;
}

function _setActiveTabById(id: string) {
    update(state => ({
        tabs: state.tabs.map(tab => ({ ...tab, isActive: tab.id === id }))
    }));
}

function _removeTabById(tabId: string) {
    update(state => {
        const newTabs = state.tabs.filter(tab => tab.id !== tabId);
        const removedActive = state.tabs.find(t => t.id === tabId)?.isActive;
        if (!removedActive) return { tabs: newTabs };

        const fallback = newTabs.length > 0 ? newTabs[0] : null;
        return {
            tabs: fallback
                ? newTabs.map(t => ({ ...t, isActive: t.id === fallback.id }))
                : [],
        };
    });
}

// ─── Public Tab Methods ───────────────────────────────────────────────────

function addSystemTab(tabId: SystemTabId) {
    const def = predefinedSystemTabs[tabId];
    update(state => {
        const exists = state.tabs.find(tab => tab.id === tabId);
        if (exists) {
            return {
                tabs: state.tabs.map(tab => ({
                    ...tab,
                    isActive: tab.id === tabId,
                })),
            };
        }

        const newTab: Tab = {
            id: tabId,
            title: def.title,
            type: def.type,
            isActive: true,
        };

        return {
            tabs: state.tabs.map(t => ({ ...t, isActive: false })).concat(newTab),
        };
    });
}

function openEmployeeTab(person: PersonSnapshot) {
    const tabId = personTabId(person.person_id);
    update(state => {
        const exists = state.tabs.find(t => t.id === tabId);
        if (exists) {
            console.log("exists");
            return {
                tabs: state.tabs.map(tab => ({
                    ...tab,
                    isActive: tab.id === tabId,
                })),
            };
        }

        const newTab: Tab = {
            id: tabId,
            title: person.name,
            type: 'person',
            context: { person },
            isActive: true,
        };

        return {
            tabs: state.tabs.map(t => ({ ...t, isActive: false })).concat(newTab),
        };
    });
}

function getSystemTab(tabId: SystemTabId): Tab | undefined {
    return get({ subscribe }).tabs.find(t => t.id === tabId);
}

function getPersonTab(person: PersonSnapshot): Tab {
    const tabId = personTabId(person.person_id);
    const found = get({ subscribe }).tabs.find(t => t.id === tabId);

    if (!found && ADD_IF_MISSING) {
        openEmployeeTab(person);
        return {
            id: tabId,
            title: person.name,
            type: 'person',
            context: { person },
            isActive: true,
        };
    }

    return found!;
}

function removePersonTab(person: PersonSnapshot) {
    _removeTabById(personTabId(person.person_id));
}

function removeSystemTab(tabId: SystemTabId) {
    _removeTabById(tabId);
}
function removeTab(tab: Tab) {
    _removeTabById(tab.id);
}

function setActivePersonTab(person: PersonSnapshot) {
    const tabId = personTabId(person.person_id);
    const exists = get({ subscribe }).tabs.find(t => t.id === tabId);
    if (!exists && ADD_IF_MISSING) {
        openEmployeeTab(person);
    } else {
        _setActiveTabById(tabId);
    }
}
function setActiveTab(tab:Tab){
    _setActiveTabById(tab.id);
}
function setActiveSystemTab(tabId: SystemTabId) {
    _setActiveTabById(tabId);
}


function getActiveTab(): Tab | null {
    return get({ subscribe }).tabs.find(t => t.isActive) ?? null;
}


// ─── Exported Interface ───────────────────────────────────────────────────

export const tabState = {
    subscribe,
    addSystemTab,
    openEmployeeTab,
    getSystemTab,
    getPersonTab,
    setActivePersonTab,
    setActiveSystemTab,
    removePersonTab,
    removeSystemTab,
    setActiveTab,
    removeTab,
    // getActiveTab,

};
export const activeTab = derived(tabState, $state =>
    $state.tabs.find(t => t.isActive) ?? null
);