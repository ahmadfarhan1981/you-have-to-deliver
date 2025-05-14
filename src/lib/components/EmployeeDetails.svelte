<script>
import {AlertCircle} from "lucide-svelte";
export let employee;
</script>

<div class="p-6">
    <div class="flex items-start justify-between mb-6">
        <div class="flex items-center">
            <div class="w-12 h-12 rounded-full bg-blue-500 flex items-center justify-center text-white mr-4 text-lg">
                {employee.initials}
            </div>
            <div>
                <h2 class="text-2xl font-bold">{employee.name}</h2>
                <p class="text-slate-600">{employee.role}</p>
            </div>
        </div>
        {#if employee.warning}
            <div class="bg-amber-100 text-amber-800 px-3 py-1 rounded-full text-xs font-medium flex items-center">
                <AlertCircle size={12} class="mr-1" />
                Requires Attention
            </div>
        {/if}
    </div>

    <div class="grid grid-cols-3 gap-6">
        <div class="col-span-2">
            <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">
                <h3 class="text-lg font-bold mb-4">Overview</h3>

                <div class="grid grid-cols-2 gap-6">
                    <div>
                        <h4 class="font-bold mb-2 text-slate-700">Contact Information</h4>
                        <div class="space-y-2 text-sm">
                            <p><span class="text-slate-500">Email:</span> {employee.details?.email || 'N/A'}</p>
                            <p><span class="text-slate-500">Phone:</span> {employee.details?.phone || 'N/A'}</p>
                            <p><span class="text-slate-500">Hire Date:</span> {employee.details?.hireDate || 'N/A'}</p>
                            <p><span class="text-slate-500">Salary:</span> {employee.details?.salary || 'N/A'}</p>
                        </div>
                    </div>

                    <div>
                        <h4 class="font-bold mb-2 text-slate-700">Current Assignment</h4>
                        <div class="space-y-2 text-sm">
                            <p><span class="text-slate-500">Task:</span> {employee.task}</p>
                            <p><span class="text-slate-500">Project:</span> {employee.project}</p>
                            <p><span class="text-slate-500">Status:</span> <span class="text-{employee.taskStatus}-600 font-medium">{employee.taskStatus === 'green' ? 'On Track' : employee.taskStatus === 'amber' ? 'At Risk' : employee.taskStatus === 'blue' ? 'In Progress' : 'Delayed'}</span></p>
                        </div>
                    </div>
                </div>

                {#if employee.details?.notes}
                    <div class="mt-6 p-3 bg-{employee.warning ? 'amber' : 'blue'}-50 border border-{employee.warning ? 'amber' : 'blue'}-200 rounded">
                        <h4 class="font-bold mb-1 text-{employee.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>
                        <p class="text-sm text-{employee.warning ? 'amber' : 'blue'}-800">{employee.details.notes}</p>
                    </div>
                {/if}
            </div>

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
            <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">
                <h3 class="text-lg font-bold mb-4">Skills & Attributes</h3>

                <div class="space-y-4">
                    <div>
                        <div class="flex justify-between text-xs mb-1">
                            <span>Coding</span>
                            <span>{employee.skills.coding}/100</span>
                        </div>
                        <div class="w-full bg-slate-200 rounded-full h-2">
                            <div class="bg-blue-500 h-2 rounded-full" style="width: {employee.skills.coding}%"></div>
                        </div>
                    </div>
                    <div>
                        <div class="flex justify-between text-xs mb-1">
                            <span>Design</span>
                            <span>{employee.skills.design}/100</span>
                        </div>
                        <div class="w-full bg-slate-200 rounded-full h-2">
                            <div class="bg-pink-500 h-2 rounded-full" style="width: {employee.skills.design}%"></div>
                        </div>
                    </div>
                    <div>
                        <div class="flex justify-between text-xs mb-1">
                            <span>Testing</span>
                            <span>{employee.skills.testing}/100</span>
                        </div>
                        <div class="w-full bg-slate-200 rounded-full h-2">
                            <div class="bg-green-500 h-2 rounded-full" style="width: {employee.skills.testing}%"></div>
                        </div>
                    </div>
                    <div>
                        <div class="flex justify-between text-xs mb-1">
                            <span>Teamwork</span>
                            <span>{employee.skills.teamwork}/100</span>
                        </div>
                        <div class="w-full bg-slate-200 rounded-full h-2">
                            <div class="bg-purple-500 h-2 rounded-full" style="width: {employee.skills.teamwork}%"></div>
                        </div>
                    </div>
                </div>

                <h4 class="font-bold mb-2 mt-6 text-slate-700">Wellness</h4>
                <div class="space-y-4">
                    <div>
                        <div class="flex justify-between text-xs mb-1">
                            <span>Mood</span>
                            <span>{employee.mood}%</span>
                        </div>
                        <div class="w-full bg-slate-200 rounded-full h-2">
                            <div class="bg-{employee.mood > 70 ? 'green' : employee.mood > 40 ? 'amber' : 'red'}-500 h-2 rounded-full" style="width: {employee.mood}%"></div>
                        </div>
                    </div>
                    <div>
                        <div class="flex justify-between text-xs mb-1">
                            <span>Energy</span>
                            <span>{employee.energy}%</span>
                        </div>
                        <div class="w-full bg-slate-200 rounded-full h-2">
                            <div class="bg-{employee.energy > 70 ? 'green' : employee.energy > 40 ? 'amber' : 'red'}-500 h-2 rounded-full" style="width: {employee.energy}%"></div>
                        </div>
                    </div>
                    <div>
                        <div class="flex justify-between text-xs mb-1">
                            <span>Stress</span>
                            <span>{employee.stress}%</span>
                        </div>
                        <div class="w-full bg-slate-200 rounded-full h-2">
                            <div class="bg-{employee.stress < 30 ? 'green' : employee.stress < 70 ? 'purple' : 'red'}-500 h-2 rounded-full" style="width: {employee.stress}%"></div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm">
                <h3 class="text-lg font-bold mb-4">Projects</h3>

                {#if employee.details?.projects}
                    <div class="space-y-2">
                        {#each employee.details.projects as project}
                            <div class="p-3 bg-slate-50 rounded border border-slate-200">
                                <div class="font-medium">{project}</div>
                                <div class="text-xs text-slate-500 mt-1">Active</div>
                            </div>
                        {/each}
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