<script lang="ts">
    import { selectedEmployee } from '$lib/stores/ui_states';
    import EmployeeStats from '$lib/components/StatPanel.svelte'
</script>

{#if $selectedEmployee}
    <div class="p-6 border rounded-lg shadow bg-white mt-4">
        <h2 class="text-xl font-bold mb-2 ">{$selectedEmployee.name}</h2>
        <p class="text-sm text-slate-600 mb-4 ">{$selectedEmployee.gender}</p>

        <EmployeeStats stats={$selectedEmployee.stats} />
    </div>
{:else}
    <div class="text-slate-500 italic mt-4">Click on an employee to view details.</div>
{/if}
