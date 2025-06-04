<script lang="ts">
    import {AlertCircle, Coffee, Info, MessageSquare, Smile, Activity} from "lucide-svelte";
    import type {PersonSnapshot, PersonSnapshotWithTotal} from "$lib/stores/persons";
    import {tabState} from "$lib/stores/TabStore";
    import EmployeeRowStatBar from "$lib/components/EmployeeRowStatBar.svelte";

    export let person: PersonSnapshotWithTotal | undefined;


    // Compute group averages
    const avg = (a: number, b: number) => Math.round((a + b) / 2);
    $: cognition = avg(person.stats.judgement, person.stats.creativity);
    $: perception = avg(person.stats.systems, person.stats.precision);
    $: drive = avg(person.stats.focus, person.stats.discipline);
    $: social = avg(person.stats.empathy, person.stats.communication);
    $: defense = avg(person.stats.resilience, person.stats.adaptability);

    // For initials
    $: initials = person.name.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase();

    $: total = person.stats.judgement
        + person.stats.creativity
        + person.stats.systems
        + person.stats.precision
        + person.stats.focus
        + person.stats.discipline
        + person.stats.empathy
        + person.stats.communication
        + person.stats.resilience
        + person.stats.adaptability
</script>

<div
        class=" grid grid-cols-12 gap-1 px-2 py-2 hover:bg-blue-400 cursor-pointer border-black border-b-2"
        on:click={() =>{
            tabState.openEmployeeTab(person);
            document.querySelector('#main_container')?.scrollTo({ top: 0, behavior: 'smooth' })
        }}
>
    <!-- Identity -->
    <div class="col-span-2 flex items-center">
        <div class="w-6 h-6 rounded-full bg-blue-500 flex items-center justify-center text-white mr-2">
            {initials}
        </div>
        <div class="flex items-center">
            <div>
                <div class="text-slate-800 font-medium">{person.name}</div>
                <div class="text-xs text-slate-500 capitalize">{person.gender}</div>
            </div>
        </div>
    </div>

    <!-- Task/Mood stand-in -->
    <div class="col-span-2 flex items-center">
        <div class="w-2 h-2 rounded-full bg-blue-500 mr-2"></div>
        <div>
            <div class="text-slate-800 text-xs">Assigned personid </div>
            <div class="text-xs text-slate-500">Placeholder project</div>
        </div>
    </div>

    <!-- Cognition  -->
    <EmployeeRowStatBar value1={person.stats.judgement} value2={person.stats.creativity} label1="Judgement" label2="Creativity"/>
    <!-- perception  -->
    <EmployeeRowStatBar value1={person.stats.systems} value2={person.stats.precision} label1="Systems" label2="Precision"/>
    <!-- Social  -->
    <EmployeeRowStatBar value1={person.stats.focus} value2={person.stats.discipline} label1="Focus" label2="Discipline"/>
    <!-- Drive -->
    <EmployeeRowStatBar value1={person.stats.empathy} value2={person.stats.communication} label1="Empathy" label2="Communication"/>
    <!-- Defense -->
    <EmployeeRowStatBar value1={person.stats.resilience} value2={person.stats.adaptability} label1="Resilience" label2="Adaptability"/>

    <EmployeeRowStatBar value1={person.energy.level} value2={person.hunger.level} label1="Energy" label2="Hunger"/>
    <!-- Rando -->
<!--    <div class="text-slate-800 col-span-1 text-center">-->
<!--        Hooray-->
<!--    </div>-->

    <!-- Social (already visualized but here for symmetry) -->
    <div class="text-slate-800 col-span-1 text-center">
        {person.total_points}
    </div>


    <!-- Action buttons -->
    <div class="text-slate-800 col-span-1 flex items-center justify-center space-x-1">
        <button class="p-1 hover:bg-slate-200 rounded">
            <MessageSquare size={14}/>
        </button>
        <button class="p-1 hover:bg-slate-200 rounded">
            <Info size={14}/>
        </button>
    </div>
</div>
