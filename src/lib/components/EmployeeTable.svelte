<script>
    import {ChevronDown, ChevronRight, Plus, Settings} from "lucide-svelte";
    import EmployeeRow from "$lib/components/EmployeeRow.svelte";
    export let employees
    export let team;
    export let tabs;
    export let activeTabId;
    // Team expansion toggle
    function toggleTeam(team) {
        expandedTeams[team] = !expandedTeams[team];
    }

    // Helper function to get the current employee for an employee tab
    function getEmployeeById(id) {
        for (const team in employees) {
            const employee = employees[team].find(e => e.id === id);
            if (employee) return employee;
        }
        return null;
    }


    let expandedTeams = {
        "Development Team": true,
        "QA Team": false,
        "Business Analyst Team": false,
        "Management Team": false
    };

    // Employee selection - opens in a new tab
    function openEmployeeTab(employee) {
        // Check if this employee already has an open tab
        const existingTab = tabs.find(tab => tab.type === "employee" && tab.employeeId === employee.id);

        if (existingTab) {
            // If tab already exists, just switch to it
            activeTabId = existingTab.id;
        } else {
            // Create a new tab for this employee
            const newTabId = `employee-${employee.id}`;
            tabs = [...tabs, {
                id: newTabId,
                title: employee.name,
                type: "employee",
                employeeId: employee.id,
                employee: employee
            }];
            activeTabId = newTabId;
        }
    }


</script>

<div class="border-b border-slate-200">
    <div
            class="flex items-center p-2 bg-slate-600 cursor-pointer hover:bg-slate-900"
            on:click={() => toggleTeam(team)}
    >
        {#if expandedTeams[team]}
            <ChevronDown size={16} class="mr-2 text-slate-500" />
        {:else}
            <ChevronRight size={16} class="mr-2 text-slate-500" />
        {/if}
        <h2 class="font-bold">{team}</h2>
        <span class="ml-2 text-xs text-slate-500">({employees[team].length} {employees[team].length === 1 ? 'member' : 'members'})</span>
        <div class="ml-auto flex items-center space-x-2">
            <button class="p-1 hover:bg-slate-500 rounded">
                <Plus size={14} />
            </button>
            <button class="p-1 hover:bg-slate-500 rounded">
                <Settings size={14} />
            </button>
        </div>
    </div>

    {#if expandedTeams[team]}
        <!-- Table Header -->
        <div class="grid grid-cols-12 gap-1 px-2 py-1 bg-slate-200 text-xs font-bold text-slate-600 border-y border-slate-300">
            <div class="col-span-2 flex items-center">Employee</div>
            <div class="col-span-2">Current Task</div>
            <div class="col-span-1">Mood</div>
            <div class="col-span-1">Energy</div>
            <div class="col-span-1">Stress</div>
            <div class="col-span-1">Coding</div>
            <div class="col-span-1">Design</div>
            <div class="col-span-1">Testing</div>
            <div class="col-span-1">Teamwork</div>
            <div class="col-span-1">Actions</div>
        </div>

        <!-- Employee Rows -->
        <div class="divide-y divide-slate-100">
            {#each employees[team] as employee}
                <EmployeeRow employee={employee} openEmployeeTab={openEmployeeTab}/>
            {/each}
        </div>
    {/if}
</div>