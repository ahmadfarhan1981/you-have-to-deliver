<script lang="ts">
    import "../app.css";
    import {onMount} from 'svelte';
    import {listen} from '@tauri-apps/api/event';
    import {gameSpeed, type GameSpeedSnapshot, gameSpeedUpdateEventName} from '$lib/stores/gameSpeed';
    import {basePersonArray, type PersonSnapshotWithTotal, personsSnapshotEventName} from "$lib/stores/persons";
    import {company, type CompanySnapshot, companySnapshotEventName} from "$lib/stores/company";
    import {type TeamSnapshot, teamSnapshotEventName, teamSnapshots} from "$lib/stores/teams";
        import {Toaster} from "svelte-hot-french-toast"
    import {
        cleanupTauriNotificationListeners,
        initializeTauriNotificationListeners
    } from "$lib/utils/eventNotificationListener";
    import {
        type AllPersonDebugDisplays,
        personDebugDisplays,
        personDebugSnapshotEventName
    } from "$lib/stores/debugDisplay";


    let { children } = $props();

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
            // console.log(JSON.stringify(event.payload))
            teamSnapshots.set(event.payload)
        } );

        const unlistenDebugDisplays = listen<AllPersonDebugDisplays>(personDebugSnapshotEventName, (event) => {
            console.log("debug snapshot", event.payload)
            personDebugDisplays.set(event.payload);
        });

        initializeTauriNotificationListeners();

        return () => {
            // Make sure to unsubscribe when the component is destroyed
            unlisten.then(fn => fn());
            person_unlisten.then(fn=>fn());
            company_unlisten.then(fn=>fn())
            teams_unlisten.then(fn=>fn())
            unlistenDebugDisplays.then(fn => fn());

            cleanupTauriNotificationListeners();

        };
    });


</script>
<Toaster />
{@render children()}