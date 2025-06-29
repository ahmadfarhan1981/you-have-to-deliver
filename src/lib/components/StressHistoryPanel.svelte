<script lang="ts">
    import {simDateFromTick, simDateToRelativeString, simDateToString} from "$lib/stores/simDate.js";
    import {gameSpeed} from "$lib/stores/gameSpeed.js";
    import type {Readable} from "svelte/store";
    import type {PersonSnapshotWithTotal} from "$lib/stores/persons.js";
    import {stressSnapshots} from "$lib/stores/stress";
    import {stressHistorySnapshots} from "$lib/stores/stressHistory";


    export let personStore:Readable<PersonSnapshotWithTotal>;
    $: person = $personStore


    $: stress = $stressSnapshots.find((s)=>s.person_id===$personStore.person_id)
    $: stressHistory = $stressHistorySnapshots.find((s)=>s.person_id===$personStore.person_id)!


    // Helper function to get stress level color
    function getStressColor(value) {
        if (value <= 50) return 'text-green-600';
        if (value <= 100) return 'text-yellow-600';
        return 'text-red-600';
    }

    function getStressBarColor(value) {
        if (value <= 50) return 'bg-green-500';
        if (value <= 100) return 'bg-yellow-500';
        return 'bg-red-500';
    }

    function getStressDotColor(value) {
        if (value <= 50) return 'bg-green-400';
        if (value <= 100) return 'bg-yellow-400';
        return 'bg-red-400';
    }

    // Calculate percentage for progress bars
    function getPercentage(value) {
        return Math.min((value / 150) * 100, 100);
    }

    // Get trend icon
    function getTrendIcon(trend) {
        switch(trend) {
            case 'up': return '↗';
            case 'down': return '↘';
            default: return '→';
        }
    }

    let showTooltip = null;
</script>

<div class="bg-white rounded-lg shadow-md border border-gray-200 p-3">
    <!-- Header with status dot -->
    <div class="flex items-center justify-between mb-3">
        <div class="flex items-center space-x-2">
            <span class="text-sm font-medium text-gray-700">Stress</span>
            <div class="w-2 h-2 rounded-full {getStressDotColor(stress.current)} animate-pulse"></div>
        </div>
        <div class="flex items-center space-x-1 text-xs text-gray-500">
            <div class="w-1.5 h-1.5 bg-green-400 rounded-full"></div>
            <div class="w-1.5 h-1.5 bg-yellow-400 rounded-full"></div>
            <div class="w-1.5 h-1.5 bg-red-400 rounded-full"></div>
        </div>
    </div>

    <!-- Main content in horizontal layout -->
    <div class="flex items-center space-x-4">
        <!-- Current Stress - Prominent but compact -->
        <div
                class="relative flex-shrink-0"
                role="button"
                tabindex="0"
                onmouseenter={() => showTooltip = 'current'}
                onmouseleave={() => showTooltip = null}
        >
            <div class="text-center">
                <div class="{getStressColor(stress.current)} text-2xl font-bold leading-none">
                    { stress.current?.toFixed(0).padStart(3, '0') }
                </div>
                <div class="text-xs text-gray-500 mt-0.5">now</div>
            </div>

            <!-- Tooltip for current -->
            {#if showTooltip === 'current'}
                <div class="absolute -top-8 left-1/2 transform -translate-x-1/2 bg-gray-800 text-white text-xs rounded px-2 py-1 whitespace-nowrap z-10">
                    Current stress
                    <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-2 border-r-2 border-t-2 border-transparent border-t-gray-800"></div>
                </div>
            {/if}
        </div>

        <!-- Vertical divider -->
        <div class="w-px h-12 bg-gray-200"></div>

        <!-- Short-term and Long-term in compact grid -->
        <div class="flex-1 grid grid-cols-2 gap-3">
            <!-- Short-term -->
            <div
                    class="relative"
                    role="button"
                    tabindex="0"
                    onmouseenter={() => showTooltip = 'short'}
                    onmouseleave={() => showTooltip = null}
            >
                <div class="flex items-center justify-between mb-1">
                    <span class="text-xs text-gray-600 font-medium">7d</span>
                    <span class="text-xs">{getTrendIcon(stressHistory.average_felt_stress)}</span>
                </div>
                <div class="{getStressColor(stressHistory.average_felt_stress)} text-lg font-semibold leading-none">
                    {stressHistory.average_felt_stress}
                </div>
                <div class="mt-1">
                    <div class="w-full bg-gray-200 rounded-full h-1">
<!--                        <div-->
<!--                                class="{getStressBarColor(stressData.shortTerm)} h-1 rounded-full transition-all duration-300"-->
<!--                                style="width: {getPercentage(stressData.shortTerm)}%"-->
<!--                        ></div>-->
                    </div>
                </div>

                <!-- Tooltip -->
                {#if showTooltip === 'short'}
                    <div class="absolute -top-8 left-1/2 transform -translate-x-1/2 bg-gray-800 text-white text-xs rounded px-2 py-1 whitespace-nowrap z-10">
                        7-day average
                        <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-2 border-r-2 border-t-2 border-transparent border-t-gray-800"></div>
                    </div>
                {/if}
            </div>

            <!-- Long-term -->
            <div
                    class="relative"
                    role="button"
                    tabindex="0"
                    onmouseenter={() => showTooltip = 'long'}
                    onmouseleave={() => showTooltip = null}
            >
                <div class="flex items-center mb-1">
                    <span class="text-xs text-gray-600 font-medium">28d</span>
                </div>
<!--                <div class="{getStressColor(stressData.longTerm)} text-lg font-semibold leading-none">-->
<!--                    {stressData.longTerm}-->
<!--                </div>-->
                <div class="mt-1">
                    <!-- Mini sparkline -->
                    {stressHistory.felt_stress_history && stressHistory.felt_stress_history.length}
                    <div class="flex items-end space-x-0.5 h-3">
                        {#each stressHistory.felt_stress_history as point, i}
                            <div
                                    class="{getStressBarColor(point)} rounded-sm transition-all duration-200"
                                    style="width: 3px; height: {Math.max((point / 150) * 12, 1)}px"
                            ></div>
                        {/each}
                    </div>
                </div>

                <!-- Tooltip -->
                {#if showTooltip === 'long'}
                    <div class="absolute -top-8 left-1/2 transform -translate-x-1/2 bg-gray-800 text-white text-xs rounded px-2 py-1 whitespace-nowrap z-10">
                        28-day trend
                        <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-2 border-r-2 border-t-2 border-transparent border-t-gray-800"></div>
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>
