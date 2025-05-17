<script lang="ts">
    import {AlertCircle} from "lucide-svelte";
    import EmployeePotrait from "$lib/components/EmployeePotrait.svelte";
    import StatPanel from "$lib/components/StatPanel.svelte";
    import StatPanel2 from "$lib/components/StatPanel2.svelte";
    import type {PersonSnapshot} from "$lib/stores/persons";




    export let person:PersonSnapshot;


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
                        <EmployeePotrait person={person} />
                    </div>

                    <div class=" grid grid-cols-1 gap-6">

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
                    <div class="ml-2">
                        <StatPanel stats={person.stats}/>
                    </div>

                </div>
                <!--{#if person.warning}-->
                {#if true}
                    <div class="mt-6 bg-amber-100 text-amber-800 px-3 py-1 rounded-full text-xs font-medium flex items-center">
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
<!--            <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm">-->
<!--                <h3 class="text-lg font-bold mb-4">Projects</h3>-->

<!--                {#if person.details?.projects}-->
<!--                    <div class="space-y-2">-->
<!--                        {#each person.details.projects as project}-->
<!--                            <div class="p-3 bg-slate-50 rounded border border-slate-200">-->
<!--                                <div class="font-medium">{project}</div>-->
<!--                                <div class="text-xs text-slate-500 mt-1">Active</div>-->
<!--                            </div>-->
<!--                        {/each}-->
<!--                    </div>-->
<!--                {:else}-->
<!--                    <p class="text-slate-500 text-sm">No projects assigned.</p>-->
<!--                {/if}-->

<!--                <div class="mt-6 flex justify-end space-x-2">-->
<!--                    <button class="px-3 py-1.5 bg-slate-200 hover:bg-slate-300 rounded text-sm">Schedule Meeting</button>-->
<!--                    <button class="px-3 py-1.5 bg-blue-500 hover:bg-blue-600 text-white rounded text-sm">Assign Task</button>-->
<!--                </div>-->
<!--            </div>-->
        </div>
    </div>
</div>