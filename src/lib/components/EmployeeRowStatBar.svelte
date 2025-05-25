<!--<script lang="ts">-->
<!--    import {Smile} from "lucide-svelte";-->

<!--    export let value;-->
<!--</script>-->

<!--<div class="col-span-1 flex items-center">-->
<!--    <Smile size={14} class="text-{value > 70 ? 'green' : value > 40 ? 'amber' : 'red'}-500 mr-1"/>-->
<!--    <div class="w-full bg-slate-200 rounded-full h-2">-->
<!--        <div class="h-2 rounded-full bg-{value > 70 ? 'green' : value > 40 ? 'amber' : 'red'}-500" style="width: {value}%">-->
<!--        </div>-->
<!--    </div>-->
<!--</div>-->

<script lang="ts">
    import {Smile} from "lucide-svelte";

    interface Props {
        value1: number;
        value2: number;
        label1?: string;
        label2?: string;
    }

    let { value1, value2, label1 = "Metric 1", label2 = "Metric 2" }: Props = $props();

    let showTooltip = $state(false);

    function getColorClass(value: number) {
        if (value > 70) return 'bg-green-500';
        if (value > 40) return 'bg-amber-500';
        return 'bg-red-500';
    }
</script>

<div class="col-span-1 flex items-center">
    <Smile size={14} class="text-gray-600 mr-1"/>
    <div
            class="w-full space-y-0.5 relative cursor-pointer"
            role="presentation"
            onmouseenter={() => showTooltip = true}
            onmouseleave={() => showTooltip = false}
    >
        <!-- First bar -->
        <div class="w-full bg-slate-200 rounded-full h-1">
            <div class="h-1 rounded-full {getColorClass(value1)}" style="width: {value1}%">
            </div>
        </div>

        <!-- Second bar -->
        <div class="w-full bg-slate-200 rounded-full h-1">
            <div class="h-1 rounded-full {getColorClass(value2)}" style="width: {value2}%">
            </div>
        </div>

        <!-- Unified Tooltip -->
        {#if showTooltip}
            <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-3 py-2 bg-gray-900 text-white text-xs rounded whitespace-nowrap z-10">
                <div class="space-y-1">
                    <div class="flex items-center gap-2">
                        <div class="w-2 h-2 rounded-full {getColorClass(value1)}"></div>
                        <span>{label1}: {value1}%</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <div class="w-2 h-2 rounded-full {getColorClass(value2)}"></div>
                        <span>{label2}: {value2}%</span>
                    </div>
                </div>
                <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-2 border-r-2 border-t-2 border-transparent border-t-gray-900"></div>
            </div>
        {/if}
    </div>
</div>
