<script lang="ts">
    import type {AssignedSkillSnapshot} from "$lib/stores/persons";
    import {get} from "svelte/store";
    let {skill}: {skill: AssignedSkillSnapshot} = $props();

    function getColorClass(value: number) {
        if (value > 70) return 'bg-green-500';
        if (value > 40) return 'bg-yellow-500';
        return 'bg-red-500';
    }
</script>
<div class="skill-item flex items-center justify-between py-1">
    <div class="flex items-center space-x-2">
        <div class="w-2 h-2 {getColorClass(skill.value)} rounded-full"></div>
        <span class="text-sm font-medium text-gray-700">{skill.skill_name}</span>
    </div>
    <div class="flex items-center space-x-2">
        <div class="w-16 bg-gray-200 rounded-full h-1.5">
            <div class="{getColorClass(skill.value)} h-1.5 rounded-full" style="width: {skill.value}%"></div>
        </div>
        <span class="text-xs text-gray-600 w-8">{skill.value}</span>
    </div>
</div>