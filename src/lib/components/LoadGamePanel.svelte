<script lang="ts">
    import type {SimDate} from "$lib/stores/simDate";
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";

    // Keep SaveSlotMetadata as it defines the structure of the nested metadata
    export interface SaveSlotMetadata {
        name: string;
        // game_time_ticks: number; // This field is missing in your Rust SaveSlotMetadata
        employee_count: number;
        sim_date: SimDate;
        save_version: string;
        last_saved_timestamp: number;
    }

    // Define the SaveSlot type which is what list_save_slots returns
    export interface SaveSlot {
        slot_id: string;
        path: string;
        metadata: SaveSlotMetadata | null; // Metadata can be null
        is_empty: boolean;
    }

    let recentGames: SaveSlot[] = []; // Expect an array of SaveSlot

    async function loadGame (slotId : String){
        await invoke("load_game",{slotId:slotId});
        await invoke("resume_sim");
    }

    onMount(async () => {
        try {
            recentGames = await invoke('list_save_slots');
            // Optional: Filter out empty slots if you only want to show those with metadata
            // recentGames = (await invoke('list_save_slots') as SaveSlot[]).filter(slot => slot.metadata && !slot.is_empty);
        } catch (error) {
            console.error("Failed to load save slots:", error);
            recentGames = [];
        }
    });
</script>

<div class="text-lg font-bold mb-3">Load Game</div>
<div class="text-sm text-slate-300 mb-4">
    Continue where you left off with one of your existing companies.
</div>

{#if recentGames.length > 0}
    <div class="space-y-3 mb-6 overflow-y-scroll h-[400px]">
        {#each recentGames as game (game.slot_id)}
            {#if game.metadata && !game.is_empty}
                <div class="p-4 bg-slate-900 border border-slate-700 rounded hover:bg-slate-700 cursor-pointer">
                    <div class="font-medium text-lg">{game.metadata.name}</div>
                    <div class="text-sm text-slate-400 mt-1">
                        Day {game.metadata.sim_date.day} • Week {game.metadata.sim_date.week} • Year {game.metadata.sim_date.year}
                    </div>
                    <div class="mt-2 flex justify-between items-center">
                        <div class="text-xs text-slate-500">
                            Last played: {new Date(game.metadata.last_saved_timestamp * 1000).toLocaleString()} <br />
                            {game.metadata.employee_count} employee{game.metadata.employee_count > 1?'s': ''} <br />
                            Save version {game.metadata.save_version} <br />
                            2 Trillion Cash
                        </div>
                        <button class="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded text-xs text-white" on:click={()=>loadGame(game.slot_id)}>
                            Load
                        </button>
                    </div>
                </div>
            {:else}
                <div class="p-4 bg-slate-800 border border-slate-700 rounded text-slate-500">
                    Slot "{game.slot_id}" is empty or data is unreadable.
                </div>

            {/if}
        {/each}
    </div>
{:else}
    <div class="text-center text-slate-500 py-8">
        No saved games found.
    </div>
{/if}

<div class="mt-auto">
    <button class="w-full py-3 bg-slate-700 hover:bg-slate-600 rounded font-medium text-slate-300 flex items-center justify-center">
        <span>Browse All Saved Games</span>
    </button>
</div>
    