<script lang="ts">
  // import {tick} from '$lib/stores/tick';

  import {onMount} from 'svelte';
  import {ChevronDown, ChevronRight, Plus, Settings, X} from 'lucide-svelte';

  import {activeView} from '$lib/stores/ui_states'
  import {activeTab, tabState} from '$lib/stores/TabStore'
  // Employee data
  import Sidebar from "$lib/components/Sidebar.svelte"
  import StatusBar from "$lib/components/StatusBar.svelte";
  import ReportDashboard from "$lib/components/ReportDashboard.svelte";
  import EmployeeRow from "$lib/components/EmployeeRow.svelte";
  import EmployeeDetails from "$lib/components/EmployeeDetails.svelte";
  import TopBar from "$lib/components/TopBar.svelte";
  import {teamManager, teams, teamSizes, teamToPersons, unassignedPersons} from '$lib/stores/teams'
  import Tabs from "$lib/components/Tabs.svelte";
  import {gameSpeed} from "$lib/stores/gameSpeed";
  import EmployeeTableSectionHeader from "$lib/components/EmployeeTableSectionHeader.svelte";


  // Navigation
  function navigateTo(view) {
    $activeView = view;
    // In a real app, this would change the content displayed
  }

  // Clock
  let currentTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

  onMount(() => {
    const timer = setInterval(() => {
      currentTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }, 60000);

    return () => {
      clearInterval(timer);
    };
  });

  let showUnassign = $state(true)

</script>

<div class="flex h-screen w-full bg-slate-100 font-mono text-sm">
  <!-- Sidebar -->
  <Sidebar activeView={activeView} navigateTo={navigateTo}/>

  <!-- Main Content -->
  <div class="flex-1 flex flex-col overflow-hidden">
    <!-- Top Bar -->
    <TopBar />

    <!-- Tabs -->
    <Tabs />

    <!-- Main Grid Area -->
    <div class="flex-1 overflow-auto bg-slate-400">
      {#if $activeTab?.id === "overview"}
        <!-- Team Sections -->
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
      {:else if $activeTab?.id === "reports"}
        <ReportDashboard />
      {:else}
        <!-- Employee Detail Tab -->
        {#if $activeTab?.type === "person"}
          {@const currentTab = $activeTab}
          {@const person = $activeTab.context.person}
          
          <EmployeeDetails person= {person} />
        {:else}
          <!-- Empty tab or custom tab content -->
          <div class="p-6">
            <h2 class="text-xl font-bold mb-4">New Tab</h2>
            <p class="text-slate-600">This is a new tab. You can customize its content.</p>
          </div>
        {/if}
      {/if}
    </div>

    <StatusBar tick={$gameSpeed.tick.tick}/>
  </div>
</div>