<script lang="ts">
  import {tick} from '$lib/stores/tick';

  import {onMount} from 'svelte';
  import {ChevronDown, ChevronRight, Plus, Settings, X} from 'lucide-svelte';

  import {activeView} from '$lib/stores/ui_states'
  import {activeTab, tabState} from '$lib/stores/TabStore'
  // Employee data
  import {employees} from "$lib/mock/mock.js";
  import Sidebar from "$lib/components/Sidebar.svelte"
  import StatusBar from "$lib/components/StatusBar.svelte";
  import ReportDashboard from "$lib/components/ReportDashboard.svelte";
  import EmployeeRow from "$lib/components/EmployeeRow.svelte";
  import EmployeeDetails from "$lib/components/EmployeeDetails.svelte";
  import TopBar from "$lib/components/TopBar.svelte";
  import {teamManager, teams, teamSizes, teamToPersons, unassignedPersons} from '$lib/stores/teams'


  // Navigation
  function navigateTo(view) {
    $activeView = view;
    // In a real app, this would change the content displayed
  }

  // Helper function to get the current employee for an employee tab
  function getEmployeeById(id) {
    for (const team in employees) {
      const employee = employees[team].find(e => e.id === id);
      if (employee) return employee;
    }
    return null;
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


</script>

<div class="flex h-screen w-full bg-slate-100 font-mono text-sm">
  <!-- Sidebar -->
  <Sidebar activeView={activeView} navigateTo={navigateTo}/>

  <!-- Main Content -->
  <div class="flex-1 flex flex-col overflow-hidden">
    <!-- Top Bar -->
    <TopBar />

    <!-- Tabs -->

    <div class="bg-slate-700 border-b border-slate-300 flex overflow-x-auto">
      {#each $tabState.tabs as tab}
        <div 
          class="px-4 py-2 {tab.isActive ? 'bg-slate-900' : ''} border-r border-slate-300 font-medium flex items-center whitespace-nowrap"
          on:click={() =>  tabState.setActiveTab(tab)}
        >
          <span>{tab.title}</span>
          <button 
            class="ml-2 text-slate-400 hover:text-slate-600"
            on:click|stopPropagation={() => tabState.removeTab(tab)}
          >
            <X size={14} />
          </button>
        </div>
      {/each}
      <button class="px-3 py-2 text-slate-600 hover:bg-slate-300" on:click={()=>tabState.addSystemTab("overview")}>
        <Plus size={16} />
      </button>
    </div>
<!--    <h2>Test{JSON.stringify($tabState.tabs)}</h2>-->
<!--    <h2>Test1{JSON.stringify(get(tabState.activeTabId))}</h2>-->
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
                {#each $teamToPersons as person}
                  <EmployeeRow person={person}  openEmployeeTab={openEmployeeTab}/>
                {/each}
              </div>
            {/if}
          </div>
        {/each}
        {#each $unassignedPersons as person}
          <EmployeeRow person={person} />
        {/each}
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

    <StatusBar tick={tick}/>
  </div>
</div>