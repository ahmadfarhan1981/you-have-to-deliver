<script>
    export let gender = "f";
    export let category = 2;//1 index , yeah ..
    export let batch = 1;
    export let index = 7;// 0 index

    // Sanitize gender
    const safeGender = gender === "f" ? "f" : "m";

    // Clamp and zero-pad category and batch
    const pad = (num) => String(Math.min(99, Math.max(0, Math.floor(num)))).padStart(2, "0");
    const catStr = pad(category);
    const batchStr = pad(batch);

    // Clamp index
    const safeIndex = index >= 0 && index <= 8 ? index : 0;

    // Build filename
    const fileName = `${safeGender}${catStr}${batchStr}.png`;
    const fullImagePath = `/images/${fileName}`;

    // Compute position in grid
    const getPosition = (i) => {
        const col = i % 3;
        const row = Math.floor(i / 3);
        const x = col * 50;
        const y = row * 50;
        return `${x}% ${y}%`;
    };
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
                  background-image: url({fullImagePath});
                  background-position: {getPosition(safeIndex)};
        "
            ></div>

            <!-- Inner subtle glow overlay -->
            <div class="absolute inset-0 rounded-md shadow-inner ring-2 ring-amber-300/50"></div>
        </div>
    </div>

    <!-- Excellence badge -->
    <div class="absolute -top-2 -right-2 bg-gradient-to-r from-amber-500 to-yellow-400 text-white text-xs font-bold px-2 py-1 rounded-full shadow-lg">
        Excellence
    </div>
</div>
