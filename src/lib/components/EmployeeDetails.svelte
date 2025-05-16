<script>
    import {AlertCircle, Image} from "lucide-svelte";
    import EmployeePotrait from "$lib/components/EmployeePotrait.svelte";
    import {employees} from "$lib/stores/employees.ts";
    import {get} from "svelte/store";
    import StatPanel from "$lib/components/StatPanel.svelte";
    import StatPanel2 from "$lib/components/StatPanel2.svelte";
export let employee;
let persons = get(employees);
let person = persons[0];

</script>

<div class="p-6">
    <div class="flex items-start justify-between mb-6">
        <div class="flex-row">
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

                <div class="flex items-start">


                    <div class="pr-6">
                        <EmployeePotrait/>
                    </div>

                    <div class=" grid grid-cols-1 gap-6">

                        <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">
                            <h3 class="text-lg font-bold mb-4">Overview</h3>

                            <div class="grid grid-cols-2 gap-6">
                                <div>
                                    <h4 class="font-bold mb-2 text-slate-700">Contact Information</h4>
                                    <div class="space-y-2 text-sm">
                                        <p><span class="text-slate-500">Email:</span>{person.name} {employee.details?.email || 'N/A'}</p>
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
                            <div class="mt-6 p-3 bg-{employee.warning ? 'amber' : 'blue'}-50 border border-{employee.warning ? 'amber' : 'blue'}-200 rounded">
                                <h4 class="font-bold mb-1 text-{employee.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>
                                <p class="text-sm text-{employee.warning ? 'amber' : 'blue'}-800">{employee.details.notes}</p>
                            </div> <div class="mt-6 p-3 bg-{employee.warning ? 'amber' : 'blue'}-50 border border-{employee.warning ? 'amber' : 'blue'}-200 rounded">
                            <h4 class="font-bold mb-1 text-{employee.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>
                            <p class="text-sm text-{employee.warning ? 'amber' : 'blue'}-800">{employee.details.notes}</p>
                        </div>
                        </div>
                    </div>

                </div>
                {#if employee.warning}
                    <div class="bg-amber-100 text-amber-800 px-3 py-1 rounded-full text-xs font-medium flex items-center">
                        <AlertCircle size={12} class="mr-1" />
                        Requires Attention
                    </div>
                {/if}
            </div>
        </div>


    </div>
    <div class="border-t border-gray-300 my-6"></div>




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

            <StatPanel stats={person.stats}/>
            <StatPanel2 employee={employee} />
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