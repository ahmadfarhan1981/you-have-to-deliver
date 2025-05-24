<script lang="ts">
    import {Pause, Play, SkipForward, SkipBack} from "lucide-svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {gameSpeed, getGameSpeedText} from "$lib/stores/gameSpeed"
    import {formatQuarterTickToTimeString, getTimeOfDayEmoji} from "$lib/utils/emojiClock"

    const stop_speed = 1
    const normal_speed =3

    $:  isOfficeHours = getTimeOfDayEmoji($gameSpeed.tick.quarter_tick).isOfficeHours
    $: emoji = getTimeOfDayEmoji($gameSpeed.tick.quarter_tick).emoji
    $: time_string = formatQuarterTickToTimeString($gameSpeed.tick.quarter_tick)
    $: tick = $gameSpeed.tick
    $: is_paused = $gameSpeed.game_speed === stop_speed
</script>

<div class="p-3 pt-1 border-t border-slate-700 bg-slate-900">
    <div class="flex items-center justify-between mb-2 ">
        <div class="text-xl font-bold">&nbsp;</div>
        <div class="flex flex-col items-end text-xs space-y-0.5">
            <div class={isOfficeHours ? 'text-green-400' : 'text-orange-500'}>
               <span class="text-xl "> {emoji}</span> {time_string}
            </div>

            <div class="text-slate-400 ">
                Day {tick.day} • W{tick.week} Year: {tick.year}
            </div>
        </div>
    </div>
    <div class="flex items-center justify-between">
        <div class="flex space-x-1">
            <button class="p-1.5 bg-slate-800 hover:bg-slate-700 rounded  active:bg-slate-600 active:scale-95 transition-all duration-150"
                    on:click={async ()=> await invoke('decrease_speed')}>
                <SkipBack size={12}/>
            </button>
            <button
                    class="p-1.5 { is_paused ? 'bg-green-700 hover:bg-green-600' : 'bg-slate-800 hover:bg-slate-700'} rounded active:bg-slate-600 active:scale-95"
                    on:click={async ()=> await invoke('set_game_speed', {speedNumber:stop_speed } )}
            >
                <Pause size={12}/>
            </button>
            <button
                    class="p-1.5 {!is_paused ? 'bg-green-700 hover:bg-green-600' : 'bg-slate-800 hover:bg-slate-700'} rounded active:bg-slate-600 active:scale-95"
                    on:click={async ()=> await invoke('set_game_speed', {speedNumber:normal_speed } )}
            >
                <Play size={12}/>
            </button>
            <button class="p-1.5 bg-slate-800 hover:bg-slate-700 active:bg-slate-600 active:scale-95 rounded transition-all duration-150"
                    on:click={async ()=> await invoke('increase_speed')}>
                <SkipForward size={12}/>
            </button>
        </div>
        <div class="flex items-center space-x-2">
            <div class="text-xs">Speed:</div>
            <select                    class="bg-slate-800 border border-slate-700 rounded text-xs p-1 "
                    on:change={async (e) => await invoke('set_game_speed', {speedNumber: Number.parseInt(e.target.value)} )}>
                <option value="1" selected={$gameSpeed.game_speed===1}>Stopped</option>
                <option value="2" selected={$gameSpeed.game_speed===2}>Slow</option>
                <option value="3" selected={$gameSpeed.game_speed===3}>Normal</option>
                <option value="4" selected={$gameSpeed.game_speed===4}>Fast</option>
                <option value="5" selected={$gameSpeed.game_speed===5}>Max</option>
            </select>
        </div>
    </div>
</div>