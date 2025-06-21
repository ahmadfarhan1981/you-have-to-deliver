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


    export let personStore: Readable<PersonSnapshotWithTotal>;
    $: person = $personStore;
    const tabItems = [
        { id: 'overview', label: 'Overview' },
        { id: 'statsSkills', label: 'Stats & Skills' },
        { id: 'detailsSchedule', label: 'Details & Schedule' },
        { id: 'performanceProjects', label: 'Performance & Projects' },
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


        <div>
            <PersonOverviewPanel person={person}/>
            <PersonalityMatrixPanel personality={person.personality}/>
<!--            <CalendarPanel  currentMonthView="1" currentYear="1"/>-->

        </div>


        <div class="col-span-1">
            <StatPanel personStore={personStore}/>

        </div>
        <div>
            <StressPanel personStore={personStore} />
            <StressHistoryPanel personStore={personStore} />

            <DayCalendarPanel />
        </div>

        <div class="col-span-1">
            <SkillsPanel personStore={personStore}/>
        </div>
        <div>
            <PersonalityMatrixPanel personality={person.personality}/>
        </div>
        <div>
            <PerformanceHistory/>
        </div>
        <div>
            <ProjectPanel/>
        </div>
    </div>
{:else }
    <h2>NOTHING</h2>
{/if}