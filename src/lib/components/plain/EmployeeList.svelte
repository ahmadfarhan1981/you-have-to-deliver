<script lang="ts">
    import { persons } from '$lib/stores/persons';
    import { selectedEmployee } from '$lib/stores/ui_states';
</script>

<ul class="divide-y divide-slate-200">
    {#each $persons as emp (emp.person_id)}
        <li class="p-4 hover:bg-slate-100 cursor-pointer" on:click={() => selectedEmployee.set(emp)}>
            <div class="font-medium">{emp.name}</div>
            <div class="text-sm text-slate-500">{emp.gender}</div>
        </li>
    {/each}
</ul>
