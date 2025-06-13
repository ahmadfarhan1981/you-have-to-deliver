<script lang="ts">


    import {personDebugDisplays} from "$lib/stores/debugDisplay";

    export let personId: number;

    // Reactive statement to get entries for the specific personId
    $: entries = $personDebugDisplays[personId] || [];
</script>

<div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm">
    <h3 class="text-lg font-bold mb-4">Debug Information (Person ID: {personId})</h3>

    {#if entries && entries.length > 0}
        <div class="space-y-1 text-sm">
            {#each entries as entry (entry.label)}
                <div class="p-2 bg-slate-50 rounded border border-slate-200 flex">
                    <div class="font-medium text-slate-700 w-2/5 pr-2 truncate">{entry.label}:</div>
                    <div class="text-slate-600 w-3/5 truncate">{entry.value}</div>
                </div>
            {/each}
        </div>
    {:else}
        <p class="text-slate-500 text-sm">No debug entries to display for Person ID: {personId}.</p>
    {/if}
</div>