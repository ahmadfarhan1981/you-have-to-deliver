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


    export let personStore: Readable<PersonSnapshotWithTotal | null>;
    $: person = $personStore;
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

    <div class="border-t border-gray-300 my-6"></div>

    <div class="grid grid-cols-3 gap-6">


        <div>
            <PersonOverviewPanel person={person}/>
            <PersonalityMatrixPanel personality={person.personality}/>
<!--            <CalendarPanel  currentMonthView="1" currentYear="1"/>-->

        </div>


        <div class="col-span-1">
            <StatPanel personStore={personStore}/>

        </div>
        <div>
            <DayCalendarPanel />
        </div>

        <div class="col-span-1">
            <SkillsPanel person={person}/>
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