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