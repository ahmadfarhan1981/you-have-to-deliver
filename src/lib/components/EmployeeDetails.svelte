<script lang="ts">
    import EmployeePotrait from "$lib/components/EmployeePotrait.svelte";
    import StatPanel from "$lib/components/StatPanel.svelte";
    import type {PersonSnapshot, PersonSnapshotWithTotal} from "$lib/stores/persons";
    import PersonalityMatrixPanel from "$lib/components/PersonalityMatrixPanel.svelte";
    import SkillsPanel from "$lib/components/SkillsPanel.svelte";
    import PerformanceHistory from "$lib/components/PerformanceHistory.svelte";
    import PersonOverviewPanel from "$lib/components/PersonOverviewPanel.svelte";
    import FinancialHealthPanel from "$lib/components/FinancialHealthPanel.svelte";
    import EmployeeInfoPanel from "$lib/components/EmployeeInfoPanel.svelte";
    import ProjectPanel from "$lib/components/ProjectPanel.svelte";
    import type {Readable, Writable} from "svelte/store";
    import {activeTab} from "$lib/stores/TabStore";
    import DebugDisplay from "$lib/components/plain/DebugDisplay.svelte";
    import ValueBar from "$lib/components/ValueBar.svelte";
    import CalendarPanel from "$lib/components/CalendarPanel.svelte";
    import WeekCalendarPanel from "$lib/components/WeekCalendarPanel.svelte";
    import DayCalendarPanel from "$lib/components/DayCalendarPanel.svelte";
    import WeekCalendarPanel2 from "$lib/components/WeekCalendarPanel2.svelte";
    import TabComponent from "$lib/components/TabComponent.svelte";
    import StressPanel from "$lib/components/StressPanel.svelte";
    import StressHistoryPanel from "$lib/components/StressHistoryPanel.svelte";
    import PersonThoughts from "$lib/components/PersonThoughts.svelte";


    export let personStore: Readable<PersonSnapshotWithTotal>;
    $: person = $personStore;
    const tabItems = [
        { id: 'overview', label: 'Overview' },
        { id: 'stats', label: 'Stats' },
        { id: 'skills', label: 'Skills' },
        { id: 'personality', label: 'Personality' },
        { id: 'schedule', label: 'Schedule' },
        {id:'stress', label: 'Stress'},
        { id: 'performance', label: 'Performance' },
        { id: 'thoughts', label: 'Thoughts' },
    ];
    let currentActiveTabId = tabItems[0].id;
</script>
{#if person != null}
    <div class="p-6 bg-game-bg game-bg">
        <div class="flex items-start justify-between mb-6">
            <div class="flex-row w-full">
                <div class="">
                    <div class="flex items-start ">
                        <div class="pr-6 ">
                            <EmployeePotrait person={person}/>
                        </div>
                        <div class=" grid grid-cols-1 gap-6 ">
<!--                            <FinancialHealthPanel/>-->
                            <EmployeeInfoPanel personStore={personStore}/>
                            <DebugDisplay personId={$personStore.person_id}/>
<!--                            <div>-->
<!--                                Needs-->
<!--                                <ValueBar value={$personStore.energy.level}/>-->
<!--                                <ValueBar value={$personStore.hunger.level}/>-->
<!--                            </div>-->
                        </div>
                    </div>
                </div>
            </div>
        </div>


    </div>



    <div class="border-t border-gray-300"></div>
    <TabComponent items={tabItems} bind:activeTabId={currentActiveTabId} />
    <div class="grid grid-cols-3 gap-6 mt-3">
        {#if currentActiveTabId === "overview" }
            <div class="ml-2">
                <PersonOverviewPanel person={person}/>

            </div>
        {/if}
        {#if currentActiveTabId === "stats" }
            <div class="col-span-1 ml-1">
                <StatPanel personStore={personStore}/>

            </div>
        {/if}
        {#if currentActiveTabId === "skills" }
            <div class="col-span-1">
                <SkillsPanel personStore={personStore}/>
            </div>
        {/if}
        {#if currentActiveTabId === "personality" }
            <div>
                <PersonalityMatrixPanel personality={person.personality}/>
            </div>
        {/if}
        {#if currentActiveTabId === "stress" }
            <div>
                <StressPanel personStore={personStore} />
                <StressHistoryPanel personStore={personStore} />
            </div>
        {/if}
        {#if currentActiveTabId === "schedule" }
            <div class="m-1">
                <DayCalendarPanel />
                <!--            <CalendarPanel  currentMonthView="1" currentYear="1"/>-->
            </div>
        {/if}
        {#if currentActiveTabId === "performance" }
            <div>
                <PerformanceHistory/>
            </div>
            <div>
                <ProjectPanel/>
            </div>
        {/if}
        {#if currentActiveTabId === "thoughts" }
            <div class="col-span-3">
                <PersonThoughts personStore={personStore}/>
            </div>
        {/if}










    </div>
{:else }
    <h2>NOTHING</h2>
{/if}