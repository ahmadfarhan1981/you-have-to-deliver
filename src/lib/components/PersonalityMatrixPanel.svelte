<script lang="ts">
    import PersonalityBar from "$lib/components/PersonalityBar.svelte";
    import {allPersonalityAxes, type PersonalityAxis} from "$lib/models/personality";

    export let personality
</script>
<div class="bg-game-bg p-4">
<!-- Compact Personality Matrix Panel -->
<div class="bg-panel-bg rounded-lg p-4 shadow-lg max-w-md">
    <h3 class="text-lg font-semibold text-gray-900 mb-3">Personality Matrix</h3>

    <div class="space-y-3">
        {#each allPersonalityAxes as axis }
        <PersonalityBar personality={personality} axis={axis} />
        {/each}

    </div>

    <!-- Compact Summary -->
    <div class="mt-3 pt-3 border-t border-gray-200">
        <div class="text-xs text-gray-600 leading-tight">
            <span class="font-medium">Profile:</span> Moderately assertive, flexible, idealistic, reserved, balanced influence
        </div>
    </div>
</div>
</div>
