<script>
    import {Pause, Play, SkipForward} from "lucide-svelte";
    import { tick } from "\$lib/stores/tick";}
</script>

<div class="p-3 border-t border-slate-700 bg-slate-900">
    <div class="flex items-center justify-between mb-2">
        <div class="text-xs font-bold">SIMULATION</div>
        <div class="text-xs text-slate-400">Day 42 • Q2 2025</div>
    </div>
    <div class="flex items-center justify-between">
        <div class="flex space-x-1">
            <button
                    class="p-1.5 { true ? 'bg-green-700 hover:bg-green-600' : 'bg-slate-800 hover:bg-slate-700'} rounded"
                    on:click={toggleSimulation}
            >
                <Pause size={14} />
            </button>
            <button
                    class="p-1.5 {isPlaying ? 'bg-green-700 hover:bg-green-600' : 'bg-slate-800 hover:bg-slate-700'} rounded"
                    on:click={toggleSimulation}
            >
                <Play size={14} />
            </button>
            <button class="p-1.5 bg-slate-800 hover:bg-slate-700 rounded">
                <SkipForward size={14} />
            </button>
        </div>
        <div class="flex items-center space-x-2">
            <div class="text-xs">Speed:</div>

            <select
                    class="bg-slate-800 border border-slate-700 rounded text-xs p-1"
                    bind:value={simulationSpeed}
                    on:change={changeSpeed}
            >
                <option>Stopped</option>
                <option>Slow</option>
                <option>Normal</option>
                <option>Fast</option>
                <option>Max</option>
            </select>
        </div>
    </div>
</div>