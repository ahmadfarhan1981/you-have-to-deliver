<script lang="ts">
    import type { Readable } from 'svelte/store';
    import type { PersonSnapshotWithTotal } from '$lib/stores/persons';
    import { simDateToVerboseString } from '$lib/stores/simDate.js';
    import { thoughtsSnapshots } from '$lib/stores/thoughts';

    export let personStore: Readable<PersonSnapshotWithTotal>;
    $: person = $personStore;
    $: thoughtSnapshot = $thoughtsSnapshots.find(t => t.person_id === $personStore.person_id);
</script>

<div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">
    <h3 class="text-lg font-bold mb-4">Thoughts</h3>
    {#if thoughtSnapshot && thoughtSnapshot.thoughts.length > 0}
        <ul class="space-y-1 text-sm">
            {#each thoughtSnapshot.thoughts as t, i}
                <li class="flex justify-between">
                    <span>{simDateToVerboseString(t.sim_date)}</span>
                    <span class="text-slate-600">{JSON.stringify(t.context)}</span>
                </li>
            {/each}
        </ul>
    {:else}
        <p class="text-sm text-slate-500">No thoughts.</p>
    {/if}
</div>
