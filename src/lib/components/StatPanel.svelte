<script lang="ts">
    import StatsEchart from "$lib/components/StatsEchart.svelte";
    import {stats_definition, type StatsSnapshot} from "$lib/models/stats";

    export let stats: StatsSnapshot;

</script>

<div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">
    <h3 class="text-lg font-bold mb-4">Skills & Attributes</h3>
    <div class="h-[300px] w-[300px]"><StatsEchart statsSnapshot={stats} /></div>
    <!--{#each stats_definition as group}-->
    <!--    <div class="mb-4 ">-->


    <!--        <h4 class="text-sm font-semibold text-slate-600 mb-2">{group.group}</h4>-->
    <!--        <div class="space-y-4">-->
    <!--            {#each group.items as item}-->
    <!--                <div class="mb-1">-->
    <!--                    <div class="flex justify-between text-xs mb-1">-->
    <!--                        <span>{item.label}</span>-->
    <!--                        <span>{stats[item.key]}/100</span>-->
    <!--                    </div>-->
    <!--                    <div class="w-full bg-slate-200 rounded-full h-2">-->
    <!--                        <div-->
    <!--                                class={`h-2 rounded-full ${item.class}`}-->
    <!--                                style={`width: ${stats[item.key]}%`}-->
    <!--                        ></div>-->
    <!--                    </div>-->
    <!--                </div>-->
    <!--            {/each}-->
    <!--        </div>-->
    <!--    </div>-->
    <!--{/each}-->
</div>
