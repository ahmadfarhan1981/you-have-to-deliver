<!-- Standalone Panel Example -->
<div class="mt-12">
    <h2 class="text-xl font-bold mb-4 text-devcorp-accent">Standalone Day Panel</h2>
    <div class="max-w-sm">
        <!-- Standalone Day Schedule Panel -->
        <div class="bg-devcorp-dark rounded-lg border border-gray-700">
            <!-- Panel Header -->
            <div class="p-4 border-b border-gray-700">
                <div class="flex items-center justify-between">
                    <div>
                        <h3 class="text-lg font-semibold">Today's Schedule</h3>
                        <p class="text-sm text-gray-400">Tuesday, January 9, 2024</p>
                    </div>
                    <div class="flex items-center space-x-2">
                        <button class="p-1 hover:bg-devcorp-blue rounded">
                            <span class="text-sm">←</span>
                        </button>
                        <button class="p-1 hover:bg-devcorp-blue rounded">
                            <span class="text-sm">→</span>
                        </button>
                    </div>
                </div>
            </div>

            <!-- Compact Time Slots -->
            <div class="max-h-64 overflow-y-auto">
                <!-- Morning Block -->
                <div class="p-3 border-b border-gray-700">
                    <div class="text-xs text-gray-400 mb-2">Morning</div>
                    <div class="space-y-2">
                        <div class="bg-devcorp-green bg-opacity-20 border-l-2 border-devcorp-green px-2 py-1 rounded-r">
                            <div class="flex justify-between items-center">
                                <div>
                                    <div class="text-xs font-medium text-devcorp-green">Daily Standup</div>
                                    <div class="text-xs text-gray-400">9:00 AM</div>
                                </div>
                                <div class="text-xs text-gray-400">15min</div>
                            </div>
                        </div>
                        <div class="bg-devcorp-purple bg-opacity-20 border-l-2 border-devcorp-purple px-2 py-1 rounded-r">
                            <div class="flex justify-between items-center">
                                <div>
                                    <div class="text-xs font-medium text-devcorp-purple">Code Review</div>
                                    <div class="text-xs text-gray-400">10:00 AM</div>
                                </div>
                                <div class="text-xs text-gray-400">1hr</div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Afternoon Block -->
                <div class="p-3 border-b border-gray-700">
                    <div class="text-xs text-gray-400 mb-2">Afternoon</div>
                    <div class="space-y-2">
                        <div class="bg-devcorp-green bg-opacity-30 border-l-2 border-devcorp-green px-2 py-1 rounded-r">
                            <div class="flex justify-between items-center">
                                <div>
                                    <div class="text-xs font-medium text-devcorp-green">All Hands Meeting</div>
                                    <div class="text-xs text-gray-400">2:00 PM • Now</div>
                                </div>
                                <div class="w-2 h-2 bg-devcorp-green rounded-full animate-pulse"></div>
                            </div>
                        </div>
                        <div class="bg-devcorp-orange bg-opacity-20 border-l-2 border-devcorp-orange px-2 py-1 rounded-r">
                            <div class="flex justify-between items-center">
                                <div>
                                    <div class="text-xs font-medium text-devcorp-orange">Client Call</div>
                                    <div class="text-xs text-gray-400">4:00 PM</div>
                                </div>
                                <div class="text-xs text-gray-400">30min</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Compact Footer -->
            <div class="p-3 border-t border-gray-700 bg-devcorp-blue bg-opacity-20">
                <div class="flex items-center justify-between text-xs">
                    <span class="text-gray-400">4 events today</span>
                    <button class="text-devcorp-accent hover:text-blue-300 font-medium">
                        + Add
                    </button>
                </div>
            </div>
        </div>
    </div>
</div>