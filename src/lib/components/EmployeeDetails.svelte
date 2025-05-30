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
    import ProjectPanel from "$lib/components/ProjectPanel.svelte";
    import StatsEchart from "$lib/components/StatsEchart.svelte";

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

        <div class="flex-row w-full">
            <div class="">
                <div class="flex items-start ">
                    <div class="pr-6 ">
                        <EmployeePotrait person={person}/>
                    </div>
                    <div class=" grid grid-cols-1 gap-6 ">
                        <FinancialHealthPanel/>
                        <EmployeeInfoPanel person={person}/>
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
    </div>

    <div class="col-span-1">
        <StatPanel stats={person.stats}/>

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
