<script lang="ts">
    // import {tick} from '$lib/stores/tick';

    import {ChevronDown, ChevronRight, Plus, Settings} from 'lucide-svelte';
    // Employee data
    import EmployeeRow from "$lib/components/EmployeeRow.svelte";
    import {teamManager, teams, teamSizes, teamToPersons, unassignedPersons} from '$lib/stores/teams'
    import EmployeeTableSectionHeader from "$lib/components/EmployeeTableSectionHeader.svelte";

    let showUnassign = $state(true);
</script>
{#each $teams as team}
          <div class="border-b border-slate-200">
            <div
              class="flex items-center p-2 bg-slate-600 cursor-pointer hover:bg-slate-900"
              on:click={()=>teamManager.toggleTeamExpanded(team.id)}
            >
              {#if team.expanded }
                <ChevronDown size={16} class="mr-2 text-slate-500" />
              {:else}
                <ChevronRight size={16} class="mr-2 text-slate-500" />
              {/if}
              <h2 class="font-bold">{team.name}</h2>
              <span class="ml-2 text-xs text-slate-500">({ $teamSizes.get(team.id)} {$teamSizes.get(team.id) === 1 ? 'member' : 'members'})</span>
              <div class="ml-auto flex items-center space-x-2">
                <button class="p-1 hover:bg-slate-500 rounded">
                  <Plus size={14} />
                </button>
                <button class="p-1 hover:bg-slate-500 rounded">
                  <Settings size={14} />
                </button>
              </div>
            </div>

            {#if team.expanded}
              <!-- Table Header -->
              <EmployeeTableSectionHeader />

              <!-- Employee Rows -->
              <div class="divide-y divide-slate-100">
                {#each $teamToPersons as person}
                  <EmployeeRow person={person} />
                {/each}
              </div>
            {/if}
          </div>
        {/each}
        <div
                class="flex items-center p-2 bg-slate-600 cursor-pointer hover:bg-slate-900"
                on:click={()=>{showUnassign = !showUnassign}}
        >
          {#if showUnassign}
            <ChevronDown size={16} class="mr-2 text-slate-500" />
          {:else}
            <ChevronRight size={16} class="mr-2 text-slate-500" />
          {/if}
          <h2 class="font-bold">Unassigned employees</h2>
          <span class="ml-2 text-xs text-slate-500">({ $unassignedPersons.length } {$unassignedPersons.length === 1 ? 'member' : 'members'})</span>
          <div class="ml-auto flex items-center space-x-2">
            <button class="p-1 hover:bg-slate-500 rounded">
              <Plus size={14} />
            </button>
            <button class="p-1 hover:bg-slate-500 rounded">
              <Settings size={14} />
            </button>
          </div>
        </div>
        {#if showUnassign}
          <EmployeeTableSectionHeader />
        {#each $unassignedPersons as person}

          <div class="divide-y divide-slate-100">
            <EmployeeRow person={person} />
          </div>
        {/each}
          {/if}