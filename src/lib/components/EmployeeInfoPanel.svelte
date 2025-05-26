<script>
    export let person;
</script>
<div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">
    <h3 class="text-lg font-bold mb-4 w-full">{person.name}</h3>
    <div class="grid grid-cols-2 gap-6">
        <div>
            <h4 class="font-bold mb-2 text-slate-700">Contact Information</h4>
            <div class="space-y-2 text-sm">
                <p><span
                        class="text-slate-500">Email:</span>{person.name} {"person.details?.email" || 'N/A'}
                </p>
                <p><span class="text-slate-500">Phone:</span> {"person.details?.phone" || 'N/A'}
                </p>
                <p><span
                        class="text-slate-500">Hire Date:</span> {"person.details?.hireDate" || 'N/A'}
                </p>
                <p><span
                        class="text-slate-500">Salary:</span> {"person.details?.salary" || 'N/A'}
                </p>
            </div>
        </div>

        <div>
            <h4 class="font-bold mb-2 text-slate-700">Current Assignment</h4>
            <div class="space-y-2 text-sm">
                <p><span class="text-slate-500">Task:</span> {"person.task"}</p>
                <p><span class="text-slate-500">Project:</span> {"person.project"}</p>
                <p><span class="text-slate-500">Status:</span> <span
                        class="text-{"person.taskStatus"}-600 font-medium">{"person.taskStatus" === 'green' ? 'On Track' : "person.taskStatus" === 'amber' ? 'At Risk' : "person.taskStatus" === 'blue' ? 'In Progress' : 'Delayed'}</span>
                </p>
            </div>
        </div>
    </div>
</div>