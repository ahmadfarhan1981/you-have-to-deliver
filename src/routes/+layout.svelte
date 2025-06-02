<script lang="ts">
    import "../app.css";
  let { children } = $props();
    import { onMount } from 'svelte';
    import { listen } from '@tauri-apps/api/event';
    import {gameSpeed, type GameSpeedSnapshot, gameSpeedUpdateEventName} from '$lib/stores/gameSpeed';
    import {
        basePersonArray,
        personArray,
        type PersonSnapshot,
        type PersonSnapshotWithTotal,
        personsSnapshotEventName
    } from "$lib/stores/persons";
    import {company, type CompanySnapshot, companySnapshotEventName} from "$lib/stores/company";
    import {type TeamSnapshot, teamSnapshotEventName, teamSnapshots} from "$lib/stores/teams";
    import {invoke} from "@tauri-apps/api/core";

    onMount(() => {
        const unlisten = listen<GameSpeedSnapshot>(gameSpeedUpdateEventName, (event) => {
            // console.log("game_speed_snapshot")
            gameSpeed.set(event.payload);
        });

        const person_unlisten = listen<PersonSnapshotWithTotal[]>(personsSnapshotEventName, (event) => {
            // console.log("persons snapshot")
            // console.log(JSON.stringify(event.payload))
            basePersonArray.set(event.payload)
        });
        const company_unlisten = listen<CompanySnapshot>(companySnapshotEventName, (event)=>{

            company.set(event.payload)
        } );
        const teams_unlisten = listen<TeamSnapshot[]>(teamSnapshotEventName, (event)=>{
            console.log(JSON.stringify(event.payload))
            teamSnapshots.set(event.payload)
        } );

        return () => {
            // Make sure to unsubscribe when the component is destroyed
            unlisten.then(fn => fn());
            person_unlisten.then(fn=>fn());
            company_unlisten.then(fn=>fn())
            teams_unlisten.then(fn=>fn())

        };
    });


</script>
{@render children()}