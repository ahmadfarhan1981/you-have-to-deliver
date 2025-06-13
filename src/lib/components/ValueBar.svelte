<script lang="ts">
    export let value: number = 50; // Value from 0 to 100
    export let label: string = "Progress";
    export let lowSpectrumLabel: string = "Low";
    export let highSpectrumLabel: string = "High";

    let showTooltip = false;

    // Ensure value is within 0-100 range for display
    $: displayValue = Math.max(0, Math.min(100, value));

    function getValueColorClass(val: number): string {
        return "bg-gray-200"
        if (val <= 20) return 'bg-red-600';       // Critical
        if (val <= 40) return 'bg-orange-500';   // Low
        if (val <= 60) return 'bg-yellow-400';   // Moderate
        if (val <= 80) return 'bg-teal-500';     // Good
        return 'bg-emerald-600';                 // Excellent
    }

    function getValueDescription(val: number): string {
        if (val <= 20) return "Critical";
        if (val <= 40) return "Low";
        if (val <= 60) return "Moderate";
        if (val <= 80) return "Good";
        return "Excellent";
    }
</script>

<div>
    <div class="relative mb-1">
        <div
                class="w-full space-y-0.5 relative cursor-pointer"
                role="presentation"
                onmouseenter={() => showTooltip = true}
                onmouseleave={() => showTooltip = false}
        >
            <!-- Bar Track with gradient from "bad" to "good" -->
            <div class="w-full h-2 bg-gradient-to-r from-red-500 via-yellow-400 to-emerald-500 rounded-full relative">
                <!-- Marker Dot -->
                <div
                        class="absolute top-0 h-2 w-2 {getValueColorClass(displayValue)} rounded-full transform -translate-x-1/2"
                        style="left: {displayValue}%"
                ></div>
            </div>

            {#if showTooltip}
                <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-3 py-2 bg-gray-900 text-white text-xs rounded whitespace-nowrap z-10 shadow-lg">
                    <div class="space-y-1">
                        <div class="flex items-center gap-2">
                            <div class="w-2 h-2 rounded-full {getValueColorClass(displayValue)}"></div>
                            <span>{label}: {displayValue}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span>{getValueDescription(displayValue)}</span>
                        </div>
                    </div>
                    <!-- Tooltip Arrow -->
                    <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-900"></div>
                </div>
            {/if}
        </div>
    </div>
    <div class="flex justify-between text-xs text-gray-600">
        <span>{lowSpectrumLabel}</span>
        <span>{highSpectrumLabel}</span>
    </div>
</div>