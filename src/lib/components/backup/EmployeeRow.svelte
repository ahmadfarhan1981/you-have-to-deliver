<script lang="ts">
    import {Activity, AlertCircle, Coffee, Info, MessageSquare, Smile} from "lucide-svelte";
    import {get} from "svelte/store";
    import {persons, type PersonSnapshot} from "$lib/stores/persons.js";
    export let employee;
    export let openEmployeeTab;

    export let person:PersonSnapshot;


</script>

<div class=" bg-slate-50 grid grid-cols-12 gap-1 px-2 py-2 hover:bg-blue-400 cursor-pointer {employee.warning ? 'bg-amber-300' : ''}"
        on:click={() => openEmployeeTab(employee)}>
    <div class="col-span-2 flex items-center">
        <div class="w-6 h-6 rounded-full bg-blue-500 flex items-center justify-center text-slate-800 mr-2">
            {employee.initials}
        </div>
        <div class="flex items-center">
            <div>
                <div class="text-slate-800 font-medium">{person.name}{employee.name}</div>
                <div class="text-slate-600 text-xs text-slate-500">{employee.role}</div>
            </div>
            {#if employee.warning}
                <AlertCircle size={14} class="ml-2 text-amber-500" />
            {/if}
        </div>
    </div>
    <div class="col-span-2 flex items-center">
        <div class="w-2 h-2 rounded-full bg-{employee.taskStatus}-500 mr-2"></div>
        <div>
            <div class="text-slate-800  text-xs">{employee.task}</div>
            <div class="text-xs text-slate-500">{employee.project}</div>
        </div>
    </div>
    <div class="col-span-1 flex items-center">
        <Smile size={14} class="text-{employee.mood > 70 ? 'green' : employee.mood > 40 ? 'amber' : 'red'}-500 mr-1" />
        <div class="w-full bg-slate-200 rounded-full h-2">
            <div class="bg-{employee.mood > 70 ? 'green' : employee.mood > 40 ? 'amber' : 'red'}-500 h-2 rounded-full" style="width: {employee.mood}%"></div>
        </div>
    </div>
    <div class="col-span-1 flex items-center">
        <Coffee size={14} class="text-{employee.energy > 70 ? 'green' : employee.energy > 40 ? 'amber' : 'red'}-500 mr-1" />
        <div class="w-full bg-slate-200 rounded-full h-2">
            <div class="bg-{employee.energy > 70 ? 'green' : employee.energy > 40 ? 'amber' : 'red'}-500 h-2 rounded-full" style="width: {employee.energy}%"></div>
        </div>
    </div>
    <div class="col-span-1 flex items-center">
        <Activity size={14} class="text-{employee.stress < 30 ? 'green' : employee.stress < 70 ? 'amber' : 'red'}-500 mr-1" />
        <div class="w-full bg-slate-200 rounded-full h-2">
            <div class="bg-{employee.stress < 30 ? 'green' : employee.stress < 70 ? 'purple' : 'red'}-500 h-2 rounded-full" style="width: {employee.stress}%"></div>
        </div>
    </div>
    <div class="text-slate-800 col-span-1 text-center">{employee.skills.coding}</div>
    <div class="text-slate-800 col-span-1 text-center">{employee.skills.design}</div>
    <div class="text-slate-800 col-span-1 text-center">{employee.skills.testing}</div>
    <div class="text-slate-800 col-span-1 text-center">{employee.skills.teamwork}</div>
    <div class="text-slate-800 col-span-1 flex items-center justify-center space-x-1">
        <button class="p-1 hover:bg-slate-200 rounded">
            <MessageSquare size={14} />
        </button>
        <button class="p-1 hover:bg-{employee.warning ? 'amber' : 'slate'}-200 rounded {employee.warning ? 'text-amber-700' : ''}">
            <Info size={14} />
        </button>
    </div>
</div>