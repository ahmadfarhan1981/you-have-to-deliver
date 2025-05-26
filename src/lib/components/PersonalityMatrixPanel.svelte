<script>
    // Function to calculate position percentage from -100 to +100 scale
    function calculatePosition(value) {
        return ((value + 100) / 200) * 100;
    }

    // Example values for demonstration
    const personalityData = {
        assertiveness: 35,
        structure: -22,
        openness: 58,
        sociability: -45,
        influence: 12
    };

    // You can use this function to dynamically update positions
    function updatePersonalityMatrix(data) {
        // Implementation would update the positions based on actual data
        console.log('Updating personality matrix with:', data);
    }

</script>
<div class="bg-game-bg p-4">
<div class="bg-panel-bg rounded-lg p-6 shadow-lg max-w-md">
    <h3 class="text-xl font-semibold text-gray-900 mb-6">Personality Matrix</h3>

    <div class="space-y-6">
        <!-- Assertiveness -->
        <div>
            <div class="flex justify-between items-center mb-2">
                <span class="text-sm font-medium text-gray-800">Assertiveness</span>
                <span class="text-sm font-bold text-gray-900">+35</span>
            </div>
            <div class="relative">
                <!-- Background track -->
                <div class="w-full h-3 bg-gray-200 rounded-full relative">
                    <!-- Center line -->
                    <div class="absolute left-1/2 top-0 w-0.5 h-3 bg-gray-400 transform -translate-x-0.5"></div>
                    <!-- Negative side (left) -->
                    <div class="absolute left-0 top-0 w-1/2 h-3 bg-blue-100 rounded-l-full"></div>
                    <!-- Positive side (right) -->
                    <div class="absolute right-0 top-0 w-1/2 h-3 bg-orange-100 rounded-r-full"></div>
                    <!-- Position indicator -->
                    <div class="absolute top-0 h-3 w-3 bg-orange-500 rounded-full transform -translate-x-1/2" style="left: 67.5%"></div>
                </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600 mt-1">
                <span>Accommodating</span>
                <span>Dominant</span>
            </div>
        </div>

        <!-- Structure Preference -->
        <div>
            <div class="flex justify-between items-center mb-2">
                <span class="text-sm font-medium text-gray-800">Structure Preference</span>
                <span class="text-sm font-bold text-gray-900">-22</span>
            </div>
            <div class="relative">
                <div class="w-full h-3 bg-gray-200 rounded-full relative">
                    <div class="absolute left-1/2 top-0 w-0.5 h-3 bg-gray-400 transform -translate-x-0.5"></div>
                    <div class="absolute left-0 top-0 w-1/2 h-3 bg-blue-100 rounded-l-full"></div>
                    <div class="absolute right-0 top-0 w-1/2 h-3 bg-green-100 rounded-r-full"></div>
                    <div class="absolute top-0 h-3 w-3 bg-blue-500 rounded-full transform -translate-x-1/2" style="left: 39%"></div>
                </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600 mt-1">
                <span>Spontaneous</span>
                <span>Methodical</span>
            </div>
        </div>

        <!-- Openness -->
        <div>
            <div class="flex justify-between items-center mb-2">
                <span class="text-sm font-medium text-gray-800">Openness</span>
                <span class="text-sm font-bold text-gray-900">+58</span>
            </div>
            <div class="relative">
                <div class="w-full h-3 bg-gray-200 rounded-full relative">
                    <div class="absolute left-1/2 top-0 w-0.5 h-3 bg-gray-400 transform -translate-x-0.5"></div>
                    <div class="absolute left-0 top-0 w-1/2 h-3 bg-blue-100 rounded-l-full"></div>
                    <div class="absolute right-0 top-0 w-1/2 h-3 bg-purple-100 rounded-r-full"></div>
                    <div class="absolute top-0 h-3 w-3 bg-purple-500 rounded-full transform -translate-x-1/2" style="left: 79%"></div>
                </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600 mt-1">
                <span>Practical</span>
                <span>Idealistic</span>
            </div>
        </div>

        <!-- Sociability -->
        <div>
            <div class="flex justify-between items-center mb-2">
                <span class="text-sm font-medium text-gray-800">Sociability</span>
                <span class="text-sm font-bold text-gray-900">-45</span>
            </div>
            <div class="relative">
                <div class="w-full h-3 bg-gray-200 rounded-full relative">
                    <div class="absolute left-1/2 top-0 w-0.5 h-3 bg-gray-400 transform -translate-x-0.5"></div>
                    <div class="absolute left-0 top-0 w-1/2 h-3 bg-blue-100 rounded-l-full"></div>
                    <div class="absolute right-0 top-0 w-1/2 h-3 bg-pink-100 rounded-r-full"></div>
                    <div class="absolute top-0 h-3 w-3 bg-blue-500 rounded-full transform -translate-x-1/2" style="left: 27.5%"></div>
                </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600 mt-1">
                <span>Reserved</span>
                <span>Outgoing</span>
            </div>
        </div>

        <!-- Influence -->
        <div>
            <div class="flex justify-between items-center mb-2">
                <span class="text-sm font-medium text-gray-800">Influence</span>
                <span class="text-sm font-bold text-gray-900">+12</span>
            </div>
            <div class="relative">
                <div class="w-full h-3 bg-gray-200 rounded-full relative">
                    <div class="absolute left-1/2 top-0 w-0.5 h-3 bg-gray-400 transform -translate-x-0.5"></div>
                    <div class="absolute left-0 top-0 w-1/2 h-3 bg-blue-100 rounded-l-full"></div>
                    <div class="absolute right-0 top-0 w-1/2 h-3 bg-cyan-100 rounded-r-full"></div>
                    <div class="absolute top-0 h-3 w-3 bg-cyan-500 rounded-full transform -translate-x-1/2" style="left: 56%"></div>
                </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600 mt-1">
                <span>Suggestive</span>
                <span>Persuasive</span>
            </div>
        </div>
    </div>

    <!-- Summary Section -->
    <div class="mt-6 pt-4 border-t border-gray-200">
        <h4 class="text-sm font-medium text-gray-800 mb-3">Personality Profile</h4>
        <div class="text-xs text-gray-600 space-y-1">
            <div>• <span class="font-medium">Moderately assertive</span> with leadership tendencies</div>
            <div>• <span class="font-medium">Prefers flexibility</span> over rigid structure</div>
            <div>• <span class="font-medium">Highly idealistic</span> and open to new ideas</div>
            <div>• <span class="font-medium">Quite reserved</span> in social situations</div>
            <div>• <span class="font-medium">Balanced influence style</span> between suggestion and persuasion</div>
        </div>
    </div>
</div>
</div>
