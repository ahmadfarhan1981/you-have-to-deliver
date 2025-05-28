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
  import { afterNavigate } from '$app/navigation';

  afterNavigate(() => {
    window.scrollTo({ top: 0, behavior: 'smooth' });
  });

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
    <div id="main_container" class="flex-1 overflow-auto bg-slate-400">
      {#if $activeTab?.id === "overview"}
        <!--- --->

        <!-- Company Overview Card -->
        <div class="bg-slate-700 rounded-lg border border-slate-600 mb-6 p-6">
          <div class="flex items-center justify-between mb-4">
            <h2 class="text-xl font-semibold text-white flex items-center">
              <svg class="w-5 h-5 mr-2 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
              </svg>
              Company Overview
            </h2>
            <div class="flex items-center space-x-2">
              <span class="px-2 py-1 bg-green-600 text-green-100 text-xs rounded-full">Stable</span>
              <span class="text-slate-400 text-sm">Day 9 • Q1 Year: 0</span>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
            <!-- Financial Health -->
            <div class="bg-slate-800 rounded-lg p-4 border border-slate-600">
              <div class="flex items-center justify-between mb-2">
                <h3 class="text-sm font-medium text-slate-300">Financial Health</h3>
                <svg class="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
                </svg>
              </div>
              <div class="text-2xl font-bold text-white mb-1">$124,500</div>
              <div class="text-xs text-green-400">+$15,200 this week</div>
              <div class="w-full bg-slate-700 rounded-full h-2 mt-2">
                <div class="bg-green-500 h-2 rounded-full" style="width: 78%"></div>
              </div>
            </div>

            <!-- Project Status -->
            <div class="bg-slate-800 rounded-lg p-4 border border-slate-600">
              <div class="flex items-center justify-between mb-2">
                <h3 class="text-sm font-medium text-slate-300">Active Projects</h3>
                <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                </svg>
              </div>
              <div class="text-2xl font-bold text-white mb-1">3</div>
              <div class="text-xs text-yellow-400">2 deadlines approaching</div>
              <div class="flex space-x-1 mt-2">
                <div class="w-1/3 bg-green-500 h-2 rounded-full"></div>
                <div class="w-1/3 bg-yellow-500 h-2 rounded-full"></div>
                <div class="w-1/3 bg-red-500 h-2 rounded-full"></div>
              </div>
            </div>

            <!-- Team Performance -->
            <div class="bg-slate-800 rounded-lg p-4 border border-slate-600">
              <div class="flex items-center justify-between mb-2">
                <h3 class="text-sm font-medium text-slate-300">Team Performance</h3>
                <svg class="w-4 h-4 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
                </svg>
              </div>
              <div class="text-2xl font-bold text-white mb-1">85%</div>
              <div class="text-xs text-green-400">Avg. efficiency</div>
              <div class="w-full bg-slate-700 rounded-full h-2 mt-2">
                <div class="bg-purple-500 h-2 rounded-full" style="width: 85%"></div>
              </div>
            </div>

            <!-- Company Reputation -->
            <div class="bg-slate-800 rounded-lg p-4 border border-slate-600">
              <div class="flex items-center justify-between mb-2">
                <h3 class="text-sm font-medium text-slate-300">Reputation</h3>
                <svg class="w-4 h-4 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"></path>
                </svg>
              </div>
              <div class="text-2xl font-bold text-white mb-1">4.2</div>
              <div class="text-xs text-yellow-400">Industry rating</div>
              <div class="flex mt-2">
                <svg class="w-3 h-3 text-yellow-400 fill-current" viewBox="0 0 20 20">
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                </svg>
                <svg class="w-3 h-3 text-yellow-400 fill-current" viewBox="0 0 20 20">
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                </svg>
                <svg class="w-3 h-3 text-yellow-400 fill-current" viewBox="0 0 20 20">
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                </svg>
                <svg class="w-3 h-3 text-yellow-400 fill-current" viewBox="0 0 20 20">
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                </svg>
                <svg class="w-3 h-3 text-slate-600" viewBox="0 0 20 20">
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                </svg>
              </div>
            </div>
          </div>

          <!-- Recent Activity & Alerts -->
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
            <!-- Recent Achievements -->
            <div class="bg-slate-800 rounded-lg p-4 border border-slate-600">
              <h3 class="text-sm font-medium text-slate-300 mb-3 flex items-center">
                <svg class="w-4 h-4 mr-2 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                Recent Achievements
              </h3>
              <div class="space-y-2">
                <div class="flex items-center text-sm">
                  <div class="w-2 h-2 bg-green-400 rounded-full mr-3"></div>
                  <span class="text-slate-300">Login page feature completed</span>
                  <span class="text-slate-500 text-xs ml-auto">3m ago</span>
                </div>
                <div class="flex items-center text-sm">
                  <div class="w-2 h-2 bg-blue-400 rounded-full mr-3"></div>
                  <span class="text-slate-300">New project proposal received</span>
                  <span class="text-slate-500 text-xs ml-auto">15m ago</span>
                </div>
                <div class="flex items-center text-sm">
                  <div class="w-2 h-2 bg-purple-400 rounded-full mr-3"></div>
                  <span class="text-slate-300">Team productivity increased 12%</span>
                  <span class="text-slate-500 text-xs ml-auto">2h ago</span>
                </div>
              </div>
            </div>

            <!-- Critical Issues & Alerts -->
            <div class="bg-slate-800 rounded-lg p-4 border border-slate-600">
              <h3 class="text-sm font-medium text-slate-300 mb-3 flex items-center">
                <svg class="w-4 h-4 mr-2 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
                </svg>
                Issues & Alerts
              </h3>
              <div class="space-y-2">
                <div class="flex items-center text-sm">
                  <div class="w-2 h-2 bg-red-400 rounded-full mr-3"></div>
                  <span class="text-slate-300">1 Critical Issue</span>
                  <span class="px-2 py-1 bg-red-600 text-red-100 text-xs rounded-full ml-auto">High</span>
                </div>
                <div class="flex items-center text-sm">
                  <div class="w-2 h-2 bg-yellow-400 rounded-full mr-3"></div>
                  <span class="text-slate-300">John experiencing high stress</span>
                  <span class="px-2 py-1 bg-yellow-600 text-yellow-100 text-xs rounded-full ml-auto">Medium</span>
                </div>
                <div class="flex items-center text-sm">
                  <div class="w-2 h-2 bg-orange-400 rounded-full mr-3"></div>
                  <span class="text-slate-300">2 Deadlines approaching</span>
                  <span class="px-2 py-1 bg-orange-600 text-orange-100 text-xs rounded-full ml-auto">Medium</span>
                </div>
              </div>
            </div>
          </div>
        </div>



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