<script lang="ts">
    import { derived } from 'svelte/store';
    import {type PersonSnapshot} from "$lib/stores/persons"
    import EmployeeSkillRow from "$lib/components/EmployeeSkillRow.svelte";


    let  {person}: {person:PersonSnapshot} = $props();

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
        {#each person.assigned_skill.assigned_skills as skill}
            <EmployeeSkillRow skill={skill} />
        {/each}
        <!-- Programming Skills -->
<!--        <div class="skill-category" data-category="programming">-->
<!--            <div class="space-y-2">-->
<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-green-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">JavaScript</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-green-500 h-1.5 rounded-full" style="width: 92%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">92</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-green-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">Python</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-green-500 h-1.5 rounded-full" style="width: 88%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">88</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-green-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">React</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-green-500 h-1.5 rounded-full" style="width: 85%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">85</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">Node.js</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-yellow-500 h-1.5 rounded-full" style="width: 72%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">72</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">TypeScript</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-yellow-500 h-1.5 rounded-full" style="width: 68%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">68</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-red-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">Rust</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-red-500 h-1.5 rounded-full" style="width: 35%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">35</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-green-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">SQL</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-green-500 h-1.5 rounded-full" style="width: 90%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">90</span>-->
<!--                    </div>-->
<!--                </div>-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; Tools Skills &ndash;&gt;-->
<!--        <div class="skill-category hidden" data-category="tools">-->
<!--            <div class="space-y-2">-->
<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-green-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">Git</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-green-500 h-1.5 rounded-full" style="width: 95%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">95</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-green-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">Docker</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-green-500 h-1.5 rounded-full" style="width: 82%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">82</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">Kubernetes</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-yellow-500 h-1.5 rounded-full" style="width: 65%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">65</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-green-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">VS Code</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-green-500 h-1.5 rounded-full" style="width: 98%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">98</span>-->
<!--                    </div>-->
<!--                </div>-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; Soft Skills &ndash;&gt;-->
<!--        <div class="skill-category hidden" data-category="soft">-->
<!--            <div class="space-y-2">-->
<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-green-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">Problem Solving</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-green-500 h-1.5 rounded-full" style="width: 89%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">89</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">Team Leadership</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-yellow-500 h-1.5 rounded-full" style="width: 73%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">73</span>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="skill-item flex items-center justify-between py-1">-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-2 h-2 bg-green-500 rounded-full"></div>-->
<!--                        <span class="text-sm font-medium text-gray-700">Communication</span>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <div class="w-16 bg-gray-200 rounded-full h-1.5">-->
<!--                            <div class="bg-green-500 h-1.5 rounded-full" style="width: 81%"></div>-->
<!--                        </div>-->
<!--                        <span class="text-xs text-gray-600 w-8">81</span>-->
<!--                    </div>-->
<!--                </div>-->
<!--            </div>-->
<!--        </div>-->
    </div>

    <!-- Show More Button -->
    <div class="mt-4 text-center">
        <button class="text-blue-600 hover:text-blue-800 text-sm font-medium">
            Show 15 more skills...
        </button>
    </div>
</div>