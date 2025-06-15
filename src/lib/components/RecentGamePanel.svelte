<script lang="ts">
    // Recent games data
    const recentGames = [
        {name: "TechGiants Inc.", day: 42, quarter: "Q2", year: 2025, timeAgo: "2 hours ago"},
        {name: "Startup Hustle", day: 156, quarter: "Q4", year: 2024, timeAgo: "Yesterday"},
        {name: "CodeCraft Solutions", day: 78, quarter: "Q3", year: 2025, timeAgo: "3 days ago"}
    ];

</script>
<div class="mt-8">
    <div class="text-xs uppercase text-slate-500 font-bold mb-2">Recent Games</div>
    <div class="space-y-2">
        {#each recentGames as game}
            <div
                    class="px-3 py-2 bg-slate-800 rounded border border-slate-700 hover:bg-slate-700 cursor-pointer"
                    on:click={() => loadGame(game)}
            >
                <div class="font-medium">{game.name}</div>
                <div class="flex justify-between text-xs text-slate-400">
                    <span>Day {game.day} • {game.quarter} {game.year}</span>
                    <span>{game.timeAgo}</span>
                </div>
            </div>
        {/each}
    </div>
</div>