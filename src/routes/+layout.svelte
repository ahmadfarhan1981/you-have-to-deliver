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
    import {type StressSnapshot, stressSnapshotEventName, stressSnapshots} from "$lib/stores/stress";
    import {
        stressHistoryEventName,
        type StressHistorySnapshot,
        stressHistorySnapshots
    } from "$lib/stores/stressHistory";


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

        const stress_unlisten = listen<StressSnapshot[]>(stressSnapshotEventName, (event)=>{
            // console.log(JSON.stringify(event.payload))
            stressSnapshots.set(event.payload)
            // console.log(JSON.stringify($stressSnapshots))
        } );

        const stress_history_unlisten =listen<StressHistorySnapshot[]>(stressHistoryEventName, (event) => {
            // console.log('Received stress history snapshot:', event.payload);
            stressHistorySnapshots.set(event.payload);
        });

        const unlistenDebugDisplays = listen<AllPersonDebugDisplays>(personDebugSnapshotEventName, (event) => {
            personDebugDisplays.set(event.payload);
        });

        initializeTauriNotificationListeners();

        return () => {
            // Make sure to unsubscribe when the component is destroyed
            unlisten.then(fn => fn());
            person_unlisten.then(fn=>fn());
            company_unlisten.then(fn=>fn());
            teams_unlisten.then(fn=>fn());
            stress_unlisten.then(fn=>fn());
            stress_history_unlisten.then(fn=>fn());
            unlistenDebugDisplays.then(fn => fn());

            cleanupTauriNotificationListeners();

        };
    });


</script>
<Toaster />
{@render children()}