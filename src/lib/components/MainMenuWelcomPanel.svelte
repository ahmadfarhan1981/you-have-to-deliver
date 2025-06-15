<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {goto} from "$app/navigation";
    import {toast} from "svelte-hot-french-toast";
    import {gameSpeed} from "$lib/stores/gameSpeed.js";
    import {onMount} from "svelte";

    // Terminal cursor blinking effect
    let showCursor = true;

    onMount(() => {
        const cursorInterval = setInterval(() => {
            showCursor = !showCursor;
        }, 500);

        return () => {
            clearInterval(cursorInterval);
        };

    });
</script>

<div class="bg-slate-800 border border-slate-700 rounded p-4 mb-8 font-mono text-sm">
    <div class="text-green-400 mb-1">$ ./welcome.sh</div>
    <div class="text-slate-300">Welcome to DevCorp Simulation v1.0.42</div>
    <div class="text-slate-300">Initializing corporate simulation environment...</div>
    <div class="text-green-400 mt-1">$ _
        {#if showCursor}<span class="terminal-cursor"></span>{/if}
    </div>
    <div class="text-green-400 mt-1">{$gameSpeed.game_speed} {$gameSpeed.tick.tick}
        Tick, {$gameSpeed.tick.quarter_tick} day tick , {$gameSpeed.tick.day}
        Day, {$gameSpeed.tick.week} Week
    </div>
<!--    <button-->
<!--            class="px-3 py-2 border-2" on:click={async () => console.log(await invoke('list_save_slots'))}-->
<!--    >List slots-->
<!--    </button>-->
<!--    <button-->
<!--            class="px-3 py-2 border-2" on:click={async () => goto("/game")}-->
<!--    >Goto game page-->
<!--    </button>-->
<!--    <button-->
<!--            class="px-3 py-2 border-2" on:click={async () => {await invoke('test_save_slots');} }-->
<!--    >Test new save-->
<!--    </button>-->
<!--    <button-->
<!--            class="px-3 py-2 border-2" on:click={async () => await invoke('increase_speed')}-->
<!--    >+-->
<!--    </button>-->
<!--    <button-->
<!--            class="px-3 py-2 border-2" on:click={async ()=> await invoke('decrease_speed')}-->
<!--    >- -->
<!--    </button>-->
<!--    <button-->
<!--            class="px-3 py-2 border-2" on:click={async () => await invoke('stop_sim')}-->
<!--    >Stop-->
<!--    </button>-->
<!--    <button-->
<!--            class="px-3 py-2 border-2"-->
<!--            on:click={async () => await invoke('new_team',{teamName:"new team name", description:"This is a team"})}-->
<!--    >New team-->
<!--    </button>-->
<!--    <button-->
<!--            class="px-3 py-2 border-2"-->
<!--            on:click={() => toast.info("Woohooo",{-->
<!--                                    style: "background-color: #1e293b; /* bg-slate-800 */\n"+-->
<!--                                            "         border-width: 1px;         /* border */\n"+-->
<!--                                            "         border-style: solid;       /* border */\n"+-->
<!--                                            "         border-color: #334155;     /* border-slate-700 */\n"+-->
<!--                                            "         border-radius: 0.25rem;    /* rounded */\n"+-->
<!--                                            "         font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace; /* font-mono */\n"+-->
<!--                                            "         font-size: 0.875rem;       /* text-sm (14px) */\n"+-->
<!--                                            "         line-height: 1.25rem;      /* text-sm (20px) */\n"+-->
<!--                                            "         color: white;              /* To ensure text is visible on dark background */\n"+-->
<!--                                            "         padding: 0.75rem 1rem;"-->
<!--                                })}-->
<!--    >Toast-->
<!--    </button>-->
<!--    <button on:click={() => toast('My first toast')}>-->
<!--        Give me a toast-->
<!--    </button>-->
<!--    <button-->
<!--            class="px-3 py-2 border-2" on:click={async () => await invoke('refresh_data')}-->
<!--    >refresh-->
<!--    </button>-->

</div>

<style>
    :global(body) {
        font-family: 'IBM Plex Mono', monospace;
        background-color: #1e293b; /* slate-800 */
    }


    .terminal-cursor {
        display: inline-block;
        width: 10px;
        height: 18px;
        background-color: #e2e8f0;
        animation: blink 1s step-end infinite;
        vertical-align: middle;
        margin-left: 4px;
    }

    @keyframes blink {
        0%, 100% {
            opacity: 1;
        }
        50% {
            opacity: 0;
        }
    }


</style>