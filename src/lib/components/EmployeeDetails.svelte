<script lang="ts">
    import {AlertCircle} from "lucide-svelte";
    import EmployeePotrait from "$lib/components/EmployeePotrait.svelte";
    import StatPanel from "$lib/components/StatPanel.svelte";
    import StatPanel2 from "$lib/components/StatPanel2.svelte";
    import type {PersonSnapshot} from "$lib/stores/persons";
    import PersonalityMatrixPanel from "$lib/components/PersonalityMatrixPanel.svelte";
    import PersonalityMatrix2 from "$lib/components/PersonalityMatrix2.svelte";
    export let person:PersonSnapshot;

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
                        <EmployeePotrait person={person} />
                    </div>

                    <div class=" grid grid-cols-1 gap-6 ">
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
                        <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">
                            <h3 class="text-lg font-bold mb-4 w-full">{person.name}</h3>

                            <div class="grid grid-cols-2 gap-6">
                                <div>
                                    <h4 class="font-bold mb-2 text-slate-700">Contact Information</h4>
                                    <div class="space-y-2 text-sm">
                                        <p><span class="text-slate-500">Email:</span>{person.name} {"person.details?.email" || 'N/A'}</p>
                                        <p><span class="text-slate-500">Phone:</span> {"person.details?.phone" || 'N/A'}</p>
                                        <p><span class="text-slate-500">Hire Date:</span> {"person.details?.hireDate" || 'N/A'}</p>
                                        <p><span class="text-slate-500">Salary:</span> {"person.details?.salary" || 'N/A'}</p>
                                    </div>
                                </div>

                                <div>
                                    <h4 class="font-bold mb-2 text-slate-700">Current Assignment</h4>
                                    <div class="space-y-2 text-sm">
                                        <p><span class="text-slate-500">Task:</span> {"person.task"}</p>
                                        <p><span class="text-slate-500">Project:</span> {"person.project"}</p>
                                        <p><span class="text-slate-500">Status:</span> <span class="text-{"person.taskStatus"}-600 font-medium">{"person.taskStatus" === 'green' ? 'On Track' : "person.taskStatus" === 'amber' ? 'At Risk' : "person.taskStatus" === 'blue' ? 'In Progress' : 'Delayed'}</span></p>
                                    </div>
                                </div>
                            </div>

                            <!--{#if person.details?.notes}-->
                            <!--    <div class="mt-6 p-3 bg-{person.warning ? 'amber' : 'blue'}-50 border border-{person.warning ? 'amber' : 'blue'}-200 rounded">-->
                            <!--        <h4 class="font-bold mb-1 text-{person.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>-->
                            <!--        <p class="text-sm text-{person.warning ? 'amber' : 'blue'}-800">{person.details.notes}</p>-->
                            <!--    </div>-->
                            <!--{/if}-->

<!--                            <div class="mt-6 p-3 bg-{person.warning ? 'amber' : 'blue'}-50 border border-{person.warning ? 'amber' : 'blue'}-200 rounded">-->
<!--                                <h4 class="font-bold mb-1 text-{person.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>-->
<!--                                <p class="text-sm text-{person.warning ? 'amber' : 'blue'}-800">{person.details.notes}</p>-->
<!--                            </div> -->
<!--                            <div class="mt-6 p-3 bg-{person.warning ? 'amber' : 'blue'}-50 border border-{person.warning ? 'amber' : 'blue'}-200 rounded">-->
<!--                            <h4 class="font-bold mb-1 text-{person.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>-->
<!--                            <p class="text-sm text-{person.warning ? 'amber' : 'blue'}-800">{person.details.notes}</p>-->
<!--                        </div>-->

                        </div>

                    </div>
<!--                    <div class="ml-2">-->
<!--                        <StatPanel stats={person.stats}/>-->
<!--                    </div>-->

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
            <div class="bg-panel-bg rounded-lg p-6 shadow-lg max-w-md">
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-lg font-semibold text-gray-800">Technical Skills</h3>
                    <button class="text-blue-600 hover:text-blue-800 text-sm font-medium">View All</button>
                </div>

                <!-- Search Bar -->
                <div class="mb-4">
                    <div class="relative">
                        <input
                                type="text"
                                placeholder="Search skills..."
                                class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                id="skillSearch"
                                oninput={()=>filterSkills()}
                        >
                        <svg class="absolute right-3 top-2.5 h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                        </svg>
                    </div>
                </div>

                <!-- Skill Categories Tabs -->
                <div class="flex space-x-1 mb-4 bg-gray-100 rounded-lg p-1">
                    <button class="skill-tab active px-3 py-1 text-xs font-medium rounded-md bg-white text-gray-700 shadow-sm" onclick={()=>showCategory('programming')}>Programming</button>
                    <button class="skill-tab px-3 py-1 text-xs font-medium rounded-md text-gray-600 hover:text-gray-700" onclick={()=>showCategory('tools')}>Tools</button>
                    <button class="skill-tab px-3 py-1 text-xs font-medium rounded-md text-gray-600 hover:text-gray-700" onclick={()=>showCategory('soft')}>Soft</button>
                </div>

                <!-- Skills Overview Stats -->
                <div class="grid grid-cols-3 gap-2 mb-4 text-center">
                    <div class="bg-green-50 rounded-lg p-2">
                        <div class="text-lg font-bold text-green-600">23</div>
                        <div class="text-xs text-green-600">Expert</div>
                    </div>
                    <div class="bg-yellow-50 rounded-lg p-2">
                        <div class="text-lg font-bold text-yellow-600">18</div>
                        <div class="text-xs text-yellow-600">Proficient</div>
                    </div>
                    <div class="bg-red-50 rounded-lg p-2">
                        <div class="text-lg font-bold text-red-600">12</div>
                        <div class="text-xs text-red-600">Learning</div>
                    </div>
                </div>

                <!-- Skills List Container -->
                <div class="skills-container max-h-64 overflow-y-auto">
                    <!-- Programming Skills -->
                    <div class="skill-category" data-category="programming">
                        <div class="space-y-2">
                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">JavaScript</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-green-500 h-1.5 rounded-full" style="width: 92%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">92</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">Python</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-green-500 h-1.5 rounded-full" style="width: 88%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">88</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">React</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-green-500 h-1.5 rounded-full" style="width: 85%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">85</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">Node.js</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-yellow-500 h-1.5 rounded-full" style="width: 72%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">72</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">TypeScript</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-yellow-500 h-1.5 rounded-full" style="width: 68%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">68</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-red-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">Rust</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-red-500 h-1.5 rounded-full" style="width: 35%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">35</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">SQL</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-green-500 h-1.5 rounded-full" style="width: 90%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">90</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Tools Skills -->
                    <div class="skill-category hidden" data-category="tools">
                        <div class="space-y-2">
                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">Git</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-green-500 h-1.5 rounded-full" style="width: 95%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">95</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">Docker</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-green-500 h-1.5 rounded-full" style="width: 82%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">82</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">Kubernetes</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-yellow-500 h-1.5 rounded-full" style="width: 65%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">65</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">VS Code</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-green-500 h-1.5 rounded-full" style="width: 98%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">98</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Soft Skills -->
                    <div class="skill-category hidden" data-category="soft">
                        <div class="space-y-2">
                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">Problem Solving</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-green-500 h-1.5 rounded-full" style="width: 89%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">89</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">Team Leadership</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-yellow-500 h-1.5 rounded-full" style="width: 73%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">73</span>
                                </div>
                            </div>

                            <div class="skill-item flex items-center justify-between py-1">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                    <span class="text-sm font-medium text-gray-700">Communication</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                        <div class="bg-green-500 h-1.5 rounded-full" style="width: 81%"></div>
                                    </div>
                                    <span class="text-xs text-gray-600 w-8">81</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Show More Button -->
                <div class="mt-4 text-center">
                    <button class="text-blue-600 hover:text-blue-800 text-sm font-medium">
                        Show 15 more skills...
                    </button>
                </div>
            </div>

<!--            <PersonalityMatrixPanel />-->
            <div class="p-2">
            <PersonalityMatrix2 />
            </div>
<!--            <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">-->
<!--                <h3 class="text-lg font-bold mb-4">Overview</h3>-->

<!--                <div class="grid grid-cols-2 gap-6">-->
<!--                    <div>-->
<!--                        <h4 class="font-bold mb-2 text-slate-700">Contact Information</h4>-->
<!--                        <div class="space-y-2 text-sm">-->
<!--                            <p><span class="text-slate-500">Email:</span> {person.details?.email || 'N/A'}</p>-->
<!--                            <p><span class="text-slate-500">Phone:</span> {person.details?.phone || 'N/A'}</p>-->
<!--                            <p><span class="text-slate-500">Hire Date:</span> {person.details?.hireDate || 'N/A'}</p>-->
<!--                            <p><span class="text-slate-500">Salary:</span> {person.details?.salary || 'N/A'}</p>-->
<!--                        </div>-->
<!--                    </div>-->

<!--                    <div>-->
<!--                        <h4 class="font-bold mb-2 text-slate-700">Current Assignment</h4>-->
<!--                        <div class="space-y-2 text-sm">-->
<!--                            <p><span class="text-slate-500">Task:</span> {person.task}</p>-->
<!--                            <p><span class="text-slate-500">Project:</span> {person.project}</p>-->
<!--                            <p><span class="text-slate-500">Status:</span> <span class="text-{person.taskStatus}-600 font-medium">{person.taskStatus === 'green' ? 'On Track' : person.taskStatus === 'amber' ? 'At Risk' : person.taskStatus === 'blue' ? 'In Progress' : 'Delayed'}</span></p>-->
<!--                        </div>-->
<!--                    </div>-->
<!--                </div>-->

<!--                {#if person.details?.notes}-->
<!--                    <div class="mt-6 p-3 bg-{person.warning ? 'amber' : 'blue'}-50 border border-{person.warning ? 'amber' : 'blue'}-200 rounded">-->
<!--                        <h4 class="font-bold mb-1 text-{person.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>-->
<!--                        <p class="text-sm text-{person.warning ? 'amber' : 'blue'}-800">{person.details.notes}</p>-->
<!--                    </div>-->
<!--                {/if}-->
<!--            </div>-->

            <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm">
                <h3 class="text-lg font-bold mb-4">Performance History</h3>

                <div class="h-48 bg-slate-100 rounded flex items-center justify-center mb-4">
                    <span class="text-slate-500">Performance Chart Placeholder</span>
                </div>

                <div class="grid grid-cols-3 gap-4 text-center">
                    <div class="p-3 bg-slate-50 rounded border border-slate-200">
                        <div class="text-xs text-slate-500">Tasks Completed</div>
                        <div class="text-xl font-bold mt-1">42</div>
                    </div>
                    <div class="p-3 bg-slate-50 rounded border border-slate-200">
                        <div class="text-xs text-slate-500">Avg. Completion Time</div>
                        <div class="text-xl font-bold mt-1">2.3 days</div>
                    </div>
                    <div class="p-3 bg-slate-50 rounded border border-slate-200">
                        <div class="text-xs text-slate-500">Quality Score</div>
                        <div class="text-xl font-bold mt-1">87%</div>
                    </div>
                </div>
            </div>
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
                    <button class="px-3 py-1.5 bg-slate-200 hover:bg-slate-300 rounded text-sm">Schedule Meeting</button>
                    <button class="px-3 py-1.5 bg-blue-500 hover:bg-blue-600 text-white rounded text-sm">Assign Task</button>
                </div>
            </div>
        </div>
    </div>
</div>