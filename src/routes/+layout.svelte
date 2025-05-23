<script lang="ts">
    import "../app.css";
  let { children } = $props();
    import { onMount } from 'svelte';
    import { listen } from '@tauri-apps/api/event';
    import {gameSpeed, type GameSpeedSnapshot, gameSpeedUpdateEventName} from '$lib/stores/gameSpeed';
    import {personArray, type PersonSnapshot, personsSnapshotEventName} from "$lib/stores/persons";

    onMount(() => {
        const unlisten = listen<GameSpeedSnapshot>(gameSpeedUpdateEventName, (event) => {
            // console.log("game_speed_snapshot")
            gameSpeed.set(event.payload);
        });

        const person_unlisten = listen<PersonSnapshot[]>(personsSnapshotEventName, (event) => {
            console.log("persons snapshot")
            console.log(JSON.stringify(event.payload))
            personArray.set(event.payload)
        });
        return () => {
            // Make sure to unsubscribe when the component is destroyed
            unlisten.then(fn => fn());
            person_unlisten.then(fn=>fn());
        };
    });
</script>
{@render children()}