<script lang="ts" >
    import {
        getAxisValue,
        type PersonalityAxis, personalityAxisLabels,
        personalityAxisSpectrum, type PersonalityAxisValue,
        type PersonalitySnapshot
    } from "$lib/stores/persons";

    const { personality, axis } = $props<{
        personality: PersonalitySnapshot;
        axis: PersonalityAxis;
    }>();

    const value = $derived(getAxisValue(personality, axis));
    // const value: PersonalityAxisValue = {value:40,desc:"Desc"}
    const percentage_value = $derived( ((value.value + 100)  / 200) * 100);

    let showTooltip = $state(false);

    function getPersonalityColorClass(value: number): string {
        if (value >= 90 || value <= -90) return 'bg-red-600';   // Extreme
        if (value >= 75 || value <= -75) return 'bg-orange-600';   // Very
        if (value >= 35 || value <= -35) return 'bg-emerald-600';   // Fairly

        return 'bg-gray-400'; // Neutral zone
    }

    function getTendencyLabel(value: number){
        if(value == 0 ) return "Neutral";
        if (value > 0 ) return personalityAxisSpectrum[axis].high;
        return personalityAxisSpectrum[axis].low;
    }
</script>
<div>
    <!--            <div class="flex justify-between items-center mb-1">-->
    <!--                <span class="text-sm font-medium text-gray-800">Assertiveness</span>-->
    <!--                <span class="text-sm font-bold text-gray-900">+35</span>-->
    <!--            </div>-->
    <div class="relative mb-1">
        <div
                class="w-full space-y-0.5 relative cursor-pointer"
                role="presentation"
                onmouseenter={() => showTooltip = true}
                onmouseleave={() => showTooltip = false}
        >
        <div class="w-full h-2 bg-gray-200 rounded-full relative">
            <div class="absolute left-1/2 top-0 w-0.5 h-2 bg- bg-gray-400 transform -translate-x-0.5"></div>
            <div class="absolute left-0 top-0 w-1/2 h-2 bg-linear-65 to-green-300 from-orange-300 rounded-l-full"></div>
            <div class="absolute right-0 top-0 w-1/2 h-2 bg-linear-65 from-green-300 to-orange-300 rounded-r-full"></div>
            <div class="absolute top-0 h-2 w-2 {getPersonalityColorClass(value.value)} rounded-full transform -translate-x-0.5" style="left: {percentage_value}%"></div>
        </div>
            {#if showTooltip}
                <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-3 py-2 bg-gray-900 text-white text-xs rounded whitespace-nowrap z-10">
                    <div class="space-y-1">
                        <div class="flex items-center gap-2">
                            <div class="w-2 h-2 rounded-full {getPersonalityColorClass(value.value)}"></div>
                            <span>{personalityAxisLabels[axis].value}: {Math.abs(value.value)}%</span>
                        </div>
                        <div class="flex items-center gap-2">
<!--                            <div class="w-2 h-2 rounded-full {getPersonalityColorClass(value.value)}"></div>-->
                            <span>{getTendencyLabel(value.value)}: {value.desc}</span>
                        </div>
                    </div>
                    <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-2 border-r-2 border-t-2 border-transparent border-t-gray-900"></div>
                </div>
            {/if}
        </div>
    </div>
    <div class="flex justify-between text-xs text-gray-600">
        <span>{personalityAxisSpectrum[axis].low}</span>
        <span>{personalityAxisSpectrum[axis].high}</span>
    </div>

</div>
