<script lang="ts">
    import { AlertCircle, Coffee, Info, MessageSquare, Smile, Activity } from "lucide-svelte";
    import type { PersonSnapshot } from "$lib/stores/employees";
    import {tabState} from "$lib/stores/TabStore";

    export let person: PersonSnapshot;
    export let openEmployeeTab;
    export let employee;
    // Compute group averages
    const avg = (a: number, b: number) => Math.round((a + b) / 2);
    const social = avg(person.stats.empathy, person.stats.communication);
    const drive = avg(person.stats.focus, person.stats.discipline);
    const defense = avg(person.stats.resilience, person.stats.adaptability);

    // For initials
    const initials = person.name.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase();
</script>

<div
        class="bg-slate-50 grid grid-cols-12 gap-1 px-2 py-2 hover:bg-blue-400 cursor-pointer"
        on:click={() =>tabState.openEmployeeTab(person)}
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
            <div class="text-slate-800 text-xs">Assigned</div>
            <div class="text-xs text-slate-500">Placeholder project</div>
        </div>
    </div>

    <!-- Social → Mood -->
    <div class="col-span-1 flex items-center">
        <Smile size={14} class="text-{social > 70 ? 'green' : social > 40 ? 'amber' : 'red'}-500 mr-1" />
        <div class="w-full bg-slate-200 rounded-full h-2">
            <div class="h-2 rounded-full bg-{social > 70 ? 'green' : social > 40 ? 'amber' : 'red'}-500" style="width: {social}%"></div>
        </div>
    </div>

    <!-- Drive → Energy -->
    <div class="col-span-1 flex items-center">
        <Coffee size={14} class="text-{drive > 70 ? 'green' : drive > 40 ? 'amber' : 'red'}-500 mr-1" />
        <div class="w-full bg-slate-200 rounded-full h-2">
            <div class="h-2 rounded-full bg-{drive > 70 ? 'green' : drive > 40 ? 'amber' : 'red'}-500" style="width: {drive}%"></div>
        </div>
    </div>

    <!-- Defense → Stress -->
    <div class="col-span-1 flex items-center">
        <Activity size={14} class="text-{defense < 30 ? 'red' : defense < 70 ? 'amber' : 'green'}-500 mr-1" />
        <div class="w-full bg-slate-200 rounded-full h-2">
            <div class="h-2 rounded-full bg-{defense < 30 ? 'red' : defense < 70 ? 'purple' : 'green'}-500" style="width: {defense}%"></div>
        </div>
    </div>

    <!-- Cognition (Judgement + Creativity) -->
    <div class="text-slate-800 col-span-1 text-center">
        {avg(person.stats.judgement, person.stats.creativity)}
    </div>

    <!-- Perception (Systems + Precision) -->
    <div class="text-slate-800 col-span-1 text-center">
        {avg(person.stats.systems, person.stats.precision)}
    </div>

    <!-- Social (already visualized but here for symmetry) -->
    <div class="text-slate-800 col-span-1 text-center">
        {social}
    </div>

    <!-- Drive (already visualized but here for symmetry) -->
    <div class="text-slate-800 col-span-1 text-center">
        {drive}
    </div>

    <!-- Action buttons -->
    <div class="text-slate-800 col-span-1 flex items-center justify-center space-x-1">
        <button class="p-1 hover:bg-slate-200 rounded">
            <MessageSquare size={14} />
        </button>
        <button class="p-1 hover:bg-slate-200 rounded">
            <Info size={14} />
        </button>
    </div>
</div>
