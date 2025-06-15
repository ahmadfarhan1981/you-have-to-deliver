<script lang="ts">
    // import {tick} from '$lib/stores/tick';

    import {onMount} from 'svelte';
    import {ChevronDown, ChevronRight, Plus, Settings} from 'lucide-svelte';

    import {activeView, DEFAULT_TEAM_EXPANDED_STATE, teamExpandedState, toggleTeamExpanded} from '$lib/stores/ui_states'
    import {activePersonSnapshot, activeTab, tabState} from '$lib/stores/TabStore'
    // Employee data
    import Sidebar from "$lib/components/Sidebar.svelte"
    import StatusBar from "$lib/components/StatusBar.svelte";
    import ReportDashboard from "$lib/components/ReportDashboard.svelte";
    import EmployeeRow from "$lib/components/EmployeeRow.svelte";
    import EmployeeDetails from "$lib/components/EmployeeDetails.svelte";
    import TopBar from "$lib/components/TopBar.svelte";
    import {teamSnapshotsWithPeople} from '$lib/stores/teams'
    import Tabs from "$lib/components/Tabs.svelte";
    import {gameSpeed} from "$lib/stores/gameSpeed";
    import EmployeeTableSectionHeader from "$lib/components/EmployeeTableSectionHeader.svelte";
    import {afterNavigate} from '$app/navigation';
    import CompanyOverview from "$lib/components/CompanyOverview.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {assignPersonToTeam, basePersonArray, personArray2} from "$lib/stores/persons.js";
    import ProjectPanel from "$lib/components/ProjectPanel.svelte";

    $: showDrawer = false;
    afterNavigate(() => {
        window.scrollTo({top: 0, behavior: 'smooth'});
    });


    // Clock
    let currentTime = new Date().toLocaleTimeString([], {hour: '2-digit', minute: '2-digit'});

    onMount(() => {
        invoke("refresh_data")
        const timer = setInterval(() => {
            currentTime = new Date().toLocaleTimeString([], {hour: '2-digit', minute: '2-digit'});
        }, 60000);

        return () => {
            clearInterval(timer);
        };
    });

    onMount(async () => {
        setTimeout(() => {
            invoke("refresh_data");
        }, 50);
    });


    $: people = $basePersonArray.map(person => {
        const calculatedTotalPoints = Object.values(person.stats).reduce((sum, val) => sum + val, 0);

        return {
            ...person,

            total_points: calculatedTotalPoints
        };
    })
    $: showUnassign = true
    let showTeamModal = false;
    let newTeamName = '';
    let newTeamDescription = '';

</script>


<div class="flex h-screen w-full bg-slate-100 font-mono text-sm">
    <!-- Sidebar -->
    <Sidebar activeView={activeView}/>

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Top Bar -->
        <TopBar/>

        <!-- Tabs -->
        <Tabs/>

        <!-- Main Grid Area -->
        <div id="main_container" class="flex-1 overflow-auto bg-slate-400">
            {#if $activeTab?.id === "overview"}
                <!-- Company Overview Card -->
                <CompanyOverview/>
                {#each $teamSnapshotsWithPeople as team}
                    <div class="border-b border-slate-200">


                        <div
                                class="flex items-center p-2  cursor-pointer hover:bg-slate-900  bg-slate-600"
                                on:click={(e)=>toggleTeamExpanded(team.id)}
                                on:dragover|preventDefault
                                on:drop={(e)=>{
                                    const id = e.dataTransfer.getData('person_id');

                                    assignPersonToTeam( Number.parseInt(id), team.id);
                                }}

                        >
                            {#if $teamExpandedState[team.id] ?? DEFAULT_TEAM_EXPANDED_STATE }
                                <ChevronDown size={16} class="mr-2 text-slate-500"/>
                            {:else}
                                <ChevronRight size={16} class="mr-2 text-slate-500"/>
                            {/if}
                            <h2 class="font-bold">{team.name}</h2>
                            <span class="ml-2 text-xs text-slate-500">({ team.members.length } {team.members.length === 1 ? 'member' : 'members'}
                                )</span>
                            <div class="ml-auto flex items-center space-x-2">
                                <button class="p-1 hover:bg-slate-500 rounded" on:click={() => {
    newTeamName = '';
    newTeamDescription = '';
    showTeamModal = true;
  }}>
                                    <Plus size={14}/>
                                </button>
                                <button class="p-1 hover:bg-slate-500 rounded">
                                    <Settings size={14}/>
                                </button>
                            </div>
                        </div>

                        {#if $teamExpandedState[team.id] ?? DEFAULT_TEAM_EXPANDED_STATE}
                            <!-- Table Header -->
                            <div>
                                <EmployeeTableSectionHeader/>
                            </div>
                            <!-- Employee Rows -->
                            <!--{#each team.members as person}-->
                            {#each team.members as person}

                                <div class="divide-y divide-slate-100" draggable="true"
                                     on:dragstart={(e) => {
                                      e.dataTransfer.setData('person_id', String(person.person_id));
                                      e.dataTransfer.effectAllowed = 'move';

                                    }}

                                >
                                    <EmployeeRow person={person}/>
                                </div>
                            {/each}

                        {/if}
                    </div>

                {/each}


            {:else if $activeTab?.id === "reports"}
                <ReportDashboard/>
            {:else if $activeTab?.id === "projects"}
                <ProjectPanel/>
            {:else}
                <!-- Employee Detail Tab -->

                {#if $activeTab?.type === "person"}
                    {@const currentTab = $activeTab}
                    {@const person = $activeTab.context.person}

                    {#if $activePersonSnapshot}<EmployeeDetails personStore={activePersonSnapshot}/>{/if}

                {:else}
                    <!-- Empty tab or custom tab content -->
                    <div class="p-6">
                        <h2 class="text-xl font-bold mb-4">New Tab</h2>
                        <p class="text-slate-600">This is a new tab. You can customize its content.</p>
                    </div>
                {/if}
            {/if}
        </div>
        <button
                on:click={() => {
              showDrawer = !showDrawer
            }}
                class="fixed bottom-4 right-4 z-50 bg-blue-600 text-white rounded-full w-12 h-12 flex items-center justify-center shadow-lg hover:bg-blue-700"
                title="Quick Jump"
        >
            👤
        </button>
        <!-- Slide-Up Panel -->
        {#if showDrawer}
            <div
                    class="fixed bottom-20 right-6 w-[300px] max-h-[80vh] bg-white shadow-2xl border rounded-lg z-40 flex flex-col overflow-hidden animate-slideup"
            >
                <div class="p-3 font-bold border-b">Quick Jump</div>
                <div class="overflow-y-auto">
                    {#each $personArray2 as person}
                        <div
                                class="p-2 hover:bg-gray-100 cursor-pointer text-sm"
                                on:click={() => {
            tabState.openEmployeeTab(person);
            showDrawer = false;
          }}
                        >
                            {person.name}
                            <span class="text-xs text-gray-500 ml-2">{person.role}</span>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}

        <style>
            @keyframes slideup {
                from {
                    opacity: 0;
                    transform: translateY(20px);
                }
                to {
                    opacity: 1;
                    transform: translateY(0);
                }
            }

            .animate-slideup {
                animation: slideup 0.2s ease-out;
            }
        </style>
        <StatusBar tick={$gameSpeed.tick.tick}/>
    </div>
</div>
{#if showTeamModal}
    <div class="fixed inset-0 bg-black bg-transparent backdrop-blur-xs flex items-center justify-center z-50">
        <div class="bg-slate-800 text-white rounded-lg p-6 shadow-lg w-[320px]">
            <h2 class="text-lg font-bold mb-4">Create New Team</h2>

            <label class="block mb-2">
                <span class="text-sm">Team Name</span>
                <input
                        type="text"
                        class="w-full border border-black focus:bg-[#2D4060] px-2 py-1 rounded mt-1"
                        bind:value={newTeamName}
                />
            </label>

            <label class="block mb-4">
                <span class="text-sm">Description</span>
                <textarea
                        class="w-full border px-2 py-1 rounded mt-1"
                        rows="2"
                        bind:value={newTeamDescription}
                />
            </label>

            <div class="flex justify-end space-x-2">
                <button
                        class="px-3 py-1 text-sm rounded bg-slate-300 hover:bg-slate-400"
                        on:click={() => (showTeamModal = false)}
                >
                    Cancel
                </button>
                <button
                        class="px-3 py-1 text-sm rounded bg-blue-600 text-white hover:bg-blue-700"
                        on:click={() => {
            showTeamModal = false;
            invoke('create_team', {
              name: newTeamName,
              description: newTeamDescription
            });
          }}
                >
                    Create
                </button>
            </div>
        </div>
    </div>
{/if}
