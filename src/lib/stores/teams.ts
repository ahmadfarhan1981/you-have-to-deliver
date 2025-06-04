import {derived, type Readable, writable} from "svelte/store";
import { personArray2, type PersonSnapshotWithTotal} from "./persons";


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
    [teamSnapshots, personArray2],
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


