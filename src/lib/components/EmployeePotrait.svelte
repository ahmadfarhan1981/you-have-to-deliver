<script lang="ts">
    import {getProfileImageData, type PersonSnapshot} from "$lib/stores/persons";
    export let person:PersonSnapshot
    let fileName = "";
    let position = "";
    $: {
        const result = getProfileImageData(person.profile_picture);
        fileName = `/images/${result.fileName}`;
        position = result.position;
    }
</script>

<style>
    .portrait-tile {
        aspect-ratio: 2 / 3;
        background-size: 300% 300%;
        background-repeat: no-repeat;
        background-position: center;
        border-radius: 4px;
        background-color: #ddd;
        overflow: hidden;
    }
</style>

<div class="relative inline-block">
    <!-- Outer glow container -->
    <div class="absolute inset-0 rounded-lg bg-gradient-to-r from-amber-300 via-yellow-400 to-amber-300 blur-md opacity-70 scale-105 animate-pulse"></div>

    <!-- Border frame with inner glow -->
    <div class="relative rounded-lg p-1 bg-gradient-to-r from-amber-500 via-yellow-300 to-amber-500">
        <!-- Image container with fixed aspect ratio -->
        <div class="relative aspect-[2/3] w-64 overflow-hidden rounded-md bg-gray-100">
            <!-- Portrait tile -->
            <div
                    class="portrait-tile"
                    style="
                  background-image: url({fileName});
                  background-position: {position};
        "
            ></div>

            <!-- Inner subtle glow overlay -->
            <div class="absolute inset-0 rounded-md shadow-inner ring-2 ring-amber-300/50"></div>
        </div>
    </div>

    <!-- Excellence badge -->
    <div class="absolute -top-2 -right-2 bg-gradient-to-r from-amber-500 to-yellow-400 text-white text-xs font-bold px-2 py-1 rounded-full shadow-lg">
        {person.name}
    </div>
</div>
