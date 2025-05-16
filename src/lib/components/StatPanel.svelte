<script lang="ts">
    import type { StatsSnapshot } from '$lib/stores/employees';
    export let stats: StatsSnapshot;
</script>

<div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">
    <h3 class="text-lg font-bold mb-4">Skills & Attributes</h3>

    {#each [
        {
            group: 'Cognition',
            items: [
                { label: 'Judgement', key: 'judgement', class: 'bg-blue-500' },
                { label: 'Creativity', key: 'creativity', class: 'bg-pink-500' }
            ]
        },
        {
            group: 'Perception',
            items: [
                { label: 'Systems Thinking', key: 'systems', class: 'bg-green-500' },
                { label: 'Precision', key: 'precision', class: 'bg-purple-500' }
            ]
        },
        {
            group: 'Drive',
            items: [
                { label: 'Focus', key: 'focus', class: 'bg-blue-500' },
                { label: 'Discipline', key: 'discipline', class: 'bg-amber-500' }
            ]
        },
        {
            group: 'Social',
            items: [
                { label: 'Empathy', key: 'empathy', class: 'bg-rose-500' },
                { label: 'Communication', key: 'communication', class: 'bg-cyan-500' }
            ]
        },
        {
            group: 'Defense',
            items: [
                { label: 'Resilience', key: 'resilience', class: 'bg-emerald-500' },
                { label: 'Adaptability', key: 'adaptability', class: 'bg-indigo-500' }
            ]
        }
    ] as group}
        <div class="mb-4">
            <h4 class="text-sm font-semibold text-slate-600 mb-2">{group.group}</h4>
            <div class="space-y-4">
                {#each group.items as item}
                    <div class="mb-1">
                        <div class="flex justify-between text-xs mb-1">
                            <span>{item.label}</span>
                            <span>{stats[item.key]}/100</span>
                        </div>
                        <div class="w-full bg-slate-200 rounded-full h-2">
                            <div
                                    class={`h-2 rounded-full ${item.class}`}
                                    style={`width: ${stats[item.key]}%`}
                            ></div>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    {/each}
</div>
