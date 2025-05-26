<script lang="ts">
    import {AlertCircle} from "lucide-svelte";
    import EmployeePotrait from "$lib/components/EmployeePotrait.svelte";
    import StatPanel from "$lib/components/StatPanel.svelte";
    import StatPanel2 from "$lib/components/StatPanel2.svelte";
    import type {PersonSnapshot} from "$lib/stores/persons";
    import PersonalityMatrixPanel from "$lib/components/PersonalityMatrixPanel.svelte";
    import SkillsPanel from "$lib/components/SkillsPanel.svelte";
    import PerformanceHistory from "$lib/components/PerformanceHistory.svelte";
    import PersonOverviewPanel from "$lib/components/PersonOverviewPanel.svelte";
    import FinancialHealthPanel from "$lib/components/FinancialHealthPanel.svelte";
    import EmployeeInfoPanel from "$lib/components/EmployeeInfoPanel.svelte";

    export let person: PersonSnapshot;

    function showCategory(category) {
        // Hide all categories
        document.querySelectorAll('.skill-category').forEach(cat => {
            cat.classList.add('hidden');
        });

        // Show selected category
        document.querySelector(`[data-category="${category}"]`).classList.remove('hidden');

        // Update tab styles
        document.querySelectorAll('.skill-tab').forEach(tab => {
            tab.classList.remove('active', 'bg-white', 'text-gray-700', 'shadow-sm');
            tab.classList.add('text-gray-600');
        });

        event.target.classList.add('active', 'bg-white', 'text-gray-700', 'shadow-sm');
        event.target.classList.remove('text-gray-600');
    }

    function filterSkills() {
        const searchTerm = document.getElementById('skillSearch').value.toLowerCase();
        const skillItems = document.querySelectorAll('.skill-item');

        skillItems.forEach(item => {
            const skillName = item.querySelector('span').textContent.toLowerCase();
            if (skillName.includes(searchTerm)) {
                item.style.display = 'flex';
            } else {
                item.style.display = 'none';
            }
        });
    }
</script>

<div class="p-6 bg-game-bg game-bg">
    <div class="flex items-start justify-between mb-6">
        <div class="flex-row border-2 border-black w-full">
            <!--            <div class="flex items-center">-->
            <!--                            <div class="border-2 w-20 h-20 rounded-full shadow bg-blue-500 flex items-center justify-center text-white mr-4 text-lg">-->
            <!--                                {employee.initials}-->
            <!--                            </div>-->
            <!--                <div>-->
            <!--                    <h2 class="text-2xl font-bold">{employee.name}</h2>-->
            <!--                    <p class="text-slate-600">{employee.role}</p>-->
            <!--                </div>-->
            <!--            </div>-->

            <div class="">

                <div class="flex items-start ">
                    <div class="pr-6 ">
                        <EmployeePotrait person={person}/>
                    </div>
                    <div class=" grid grid-cols-1 gap-6 ">
                        <FinancialHealthPanel/>
                        <EmployeeInfoPanel person={person}/>
                        <!--{#if person.details?.notes}-->
                        <!--    <div class="mt-6 p-3 bg-{person.warning ? 'amber' : 'blue'}-50 border border-{person.warning ? 'amber' : 'blue'}-200 rounded">-->
                        <!--        <h4 class="font-bold mb-1 text-{person.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>-->
                        <!--        <p class="text-sm text-{person.warning ? 'amber' : 'blue'}-800">{person.details.notes}</p>-->
                        <!--    </div>-->
                        <!--{/if}-->

                        <!--                            <div class="mt-6 p-3 bg-{person.warning ? 'amber' : 'blue'}-50 border border-{person.warning ? 'amber' : 'blue'}-200 rounded">-->
                        <!--                                <h4 class="font-bold mb-1 text-{person.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>-->
                        <!--                                <p class="text-sm text-{person.warning ? 'amber' : 'blue'}-800">{person.details.notes}</p>-->
                        <!--                            </div>-->
                        <!--                            <div class="mt-6 p-3 bg-{person.warning ? 'amber' : 'blue'}-50 border border-{person.warning ? 'amber' : 'blue'}-200 rounded">-->
                        <!--                            <h4 class="font-bold mb-1 text-{person.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>-->
                        <!--                            <p class="text-sm text-{person.warning ? 'amber' : 'blue'}-800">{person.details.notes}</p>-->
                        <!--                        </div>-->

                    </div>
                </div>
            </div>
            <!--{#if person.warning}-->
            <!--{#if true}-->
            <!--    <div class="mt-6 bg-amber-100 text-amber-800 px-3 py-1 rounded-full text-xs font-medium flex items-center">-->
            <!--        <AlertCircle size={12} class="mr-1" />-->
            <!--        Requires Attention-->
            <!--    </div>-->
            <!--{/if}-->
        </div>
    </div>


</div>

<div class="border-t border-gray-300 my-6"></div>

<div class="grid grid-cols-3 gap-6">
    <div class="col-span-2">
        <SkillsPanel/>
        <PersonalityMatrixPanel personality={person.personality}/>
        <PersonOverviewPanel person={person}/>
        <PerformanceHistory/>
    </div>

    <div class="col-span-1">

        <StatPanel stats={person.stats}/>
        <!--            <StatPanel2 employee={person} />-->
        <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm">
            <h3 class="text-lg font-bold mb-4">Projects</h3>

            {#if true}
                <div class="space-y-2">

                    <div class="p-3 bg-slate-50 rounded border border-slate-200">
                        <div class="font-medium">Project phoenix</div>
                        <div class="text-xs text-slate-500 mt-1">Active</div>
                    </div>

                </div>
            {:else}
                <p class="text-slate-500 text-sm">No projects assigned.</p>
            {/if}

            <div class="mt-6 flex justify-end space-x-2">
                <button class="px-3 py-1.5 bg-slate-200 hover:bg-slate-300 rounded text-sm">Schedule Meeting
                </button>
                <button class="px-3 py-1.5 bg-blue-500 hover:bg-blue-600 text-white rounded text-sm">Assign Task
                </button>
            </div>
        </div>
    </div>
</div>
</div>