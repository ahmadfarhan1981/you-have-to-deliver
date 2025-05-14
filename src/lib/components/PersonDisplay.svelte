<script lang="ts">
    import { onMount } from 'svelte';

    // Dummy data (copied here for isolation)
    const person = {
        person_id: { value: 1 },
        name: "MITCH RONCSKEVITZ",
        gender: "Male",
        stats: {
            judgement: 76,
            creativity: 86,
            systems: 85,
            precision: 83,
            focus: 92,
            discipline: 83,
            empathy: 84,
            communication: 79
        }
    };
</script>

<div class="border border-gray-300 p-4 rounded-lg max-w-md">
    <h2 class="text-xl font-bold">{person.name}</h2>
    <p class="text-sm text-gray-600">Gender: {person.gender}</p>
    <table class="table-auto w-full mt-2">
        <thead>
        <tr class="bg-gray-100">
            <th class="px-2 py-1 text-left">Stat</th>
            <th class="px-2 py-1 text-left">Value</th>
        </tr>
        </thead>
        <tbody>
        {#each Object.entries(person.stats) as [key, value]}
            <tr class="border-t">
                <td class="px-2 py-1 capitalize">{key}</td>
                <td class="px-2 py-1">{value}</td>
            </tr>
        {/each}
        </tbody>
    </table>
</div>
