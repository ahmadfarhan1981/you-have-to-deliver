import {writable, derived, get, type Readable} from "svelte/store";
import {personArray, type PersonSnapshotWithTotal} from "./persons";
import type { PersonSnapshot } from "./persons";

export type Team = {
    id: string;
    name: string;
    expanded: boolean;
};

// === Internal writable state ===

const _teams = writable<Team[]>([
    { id: "alpha", name: "Team Alpha", expanded: true },
    { id: "bravo", name: "Team Bravo", expanded: true }
]);

const _personToTeam = writable<Map<number, string>>(new Map());

// === Derived: team_id → list of people ===

const _teamToPersons = derived(
    [personArray, _personToTeam],
    ([$personArray, $personToTeam]) => {
        const map = new Map<string, PersonSnapshot[]>();
        for (const person of $personArray) {
            const teamId = $personToTeam.get(person.person_id);
            if (!teamId) continue;
            if (!map.has(teamId)) map.set(teamId, []);
            map.get(teamId)!.push(person);
        }
        return map;
    }
);

// === Derived: people not assigned to any team ===


export const unassignedSort = writable({
    key: 'total_points',   // or 'name', etc.
    direction: 'asc'       // or 'desc'
});

export const _unassignedPersons = derived(
    [personArray, _personToTeam, unassignedSort],
    ([$personArray, $personToTeam, $unassignedSort]) => {
        let result = $personArray.filter(p => !$personToTeam.has(p.person_id));

        const { key, direction } = $unassignedSort;

        if (key) {
            result.sort((a, b) => {
                const aVal = a[key];
                const bVal = b[key];

                // Handle string or number
                const comparison =
                    typeof aVal === 'string'
                        ? aVal.localeCompare(bVal)
                        : aVal - bVal;

                return direction === 'asc' ? comparison : -comparison;
            });
        }

        return result;
    }
);

// === Internal helpers (non-exported) ===

function _assignById(person_id: number, team_id: string) {
    _personToTeam.update(map => {
        const newMap = new Map(map);
        newMap.set(person_id, team_id);
        return newMap;
    });
}

function _unassignById(person_id: number) {
    _personToTeam.update(map => {
        const newMap = new Map(map);
        newMap.delete(person_id);
        return newMap;
    });
}

// === Public helper methods ===

const _teamManager = {
    assignPersonToTeam(person: PersonSnapshot, team_id: string) {
        _assignById(person.person_id, team_id);
    },

    unassignPerson(person: PersonSnapshot) {
        _unassignById(person.person_id);
    },

    getTeamOfPerson(person: PersonSnapshot): Team | undefined {
        const map = get(_personToTeam);
        const teamId = map.get(person.person_id);
        if (!teamId) return undefined;

        const allTeams = get(_teams);
        return allTeams.find(t => t.id === teamId);
    },

    toggleTeamExpanded(team_id: string) {
        _teams.update(teamList =>
            teamList.map(team =>
                team.id === team_id
                    ? { ...team, expanded: !team.expanded }
                    : team
            )
        );
    }
};

const _teamSizes = derived(_teamToPersons, ($teamToPersons) => {
    const result = new Map<string, number>();
    for (const [teamId, people] of $teamToPersons.entries()) {
        result.set(teamId, people.length);
    }
    return result;
});


// === Exports ===

/**
 * Exported stores:
 * - teams
 * - personToTeam
 * - teamToPersons
 * - unassignedPersons
 * - teamManager
 *
 * Use `$teams` in Svelte templates, or `teamManager.assignPersonToTeam()` in logic.
 */

export const teams = _teams;
export const personToTeam = _personToTeam;
export const teamToPersons = _teamToPersons;
export const unassignedPersons = _unassignedPersons;
export const teamManager = _teamManager;
export const teamSizes = _teamSizes;



export const teamSnapshotEventName ="teams_snapshot";
export type TeamSnapshot= {
    id: number,
    name: string,
    description: string,
    members: Number[],

}
export const teamSnapshots = writable<TeamSnapshot[]>([
]);

export type TeamWithPeople = {
    id: number;
    name: string;
    description: string;
    members: PersonSnapshotWithTotal[];

};
export const teamSnapshotsWithPeople: Readable<TeamWithPeople[]> = derived(
    [teamSnapshots, personArray],
    ([$teams, $people]) => {
        const teamsWithPeople = $teams.map(team => ({
            ...team,
            members: team.members
                .map(id => $people.find(p => p.person_id === id)!)
                .filter(Boolean)
        }));

        const unassignedPeople = $people.filter(p => p.team === null);

        const unassignedTeam = {
            id: -1, // use a negative or sentinel value to avoid collision
            name: "Unassigned",
            description: "People not currently assigned to any team",
            members: unassignedPeople
        };

        return [...teamsWithPeople, unassignedTeam];
    }
);


