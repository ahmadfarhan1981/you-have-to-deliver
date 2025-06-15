<!--<head>-->
<!--    <meta charset="UTF-8">-->
<!--    <meta name="viewport" content="width=device-width, initial-scale=1.0">-->
<!--    <title>DevCorp - Day View Panel</title>-->
<!--    <script src="https://cdn.tailwindcss.com"></script>-->
<!--    <script>-->
<!--        tailwind.config = {-->
<!--            theme: {-->
<!--                extend: {-->
<!--                    colors: {-->
<!--                        'devcorp-dark': '#1a1f2e',-->
<!--                        'devcorp-darker': '#151a26',-->
<!--                        'devcorp-blue': '#2d3748',-->
<!--                        'devcorp-accent': '#4299e1',-->
<!--                        'devcorp-green': '#48bb78',-->
<!--                        'devcorp-orange': '#ed8936',-->
<!--                        'devcorp-red': '#f56565',-->
<!--                        'devcorp-purple': '#9f7aea'-->
<!--                    }-->
<!--                }-->
<!--            }-->
<!--        }-->
<!--    </script>-->
<!--</head>-->

<!-- Demo Layout showing 3-column integration -->

<div class="bg-white rounded-lg shadow-sm border border-gray-200 font-sans">
    <!-- Panel Header -->
    <div class="p-4 border-b border-gray-200">
        <div class="flex items-center justify-between">
            <div>
                <h3 class="text-lg font-semibold text-gray-800">Today's Schedule</h3>
                <p class="text-sm text-gray-500">Tuesday, January 9, 2024</p>
            </div>
            <div class="flex items-center space-x-2">
                <button class="p-1 hover:bg-gray-100 rounded text-gray-600">
                    <span class="text-sm">←</span>
                </button>
                <button class="p-1 hover:bg-gray-100 rounded text-gray-600">
                    <span class="text-sm">→</span>
                </button>
            </div>
        </div>
    </div>

    <!-- Time Slots with 15-minute support -->
    <div class="max-h-80 overflow-y-auto">
        <!-- 8:00 AM Hour Block -->
        <div class="border-b border-gray-100">
            <div class="flex">
                <div class="w-16 p-2 text-xs text-gray-400 flex items-start border-r border-gray-100">
                    8:00
                </div>
                <div class="flex-1 p-1">
                    <!-- 15-minute slots in this hour -->
                    <div class="space-y-1">
                        <!-- 8:00-8:15 -->
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <!-- 8:15-8:30 -->
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <!-- 8:30-8:45 -->
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <!-- 8:45-9:00 -->
                        <div class="h-3 bg-gray-50 rounded"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 9:00 AM Hour Block with Events -->
        <div class="border-b border-gray-100">
            <div class="flex">
                <div class="w-16 p-2 text-xs text-gray-400 flex items-start border-r border-gray-100">
                    9:00
                </div>
                <div class="flex-1 p-1">
                    <div class="space-y-1">
                        <!-- 9:00-9:15 - Daily Standup -->
                        <div class="h-3 bg-green-100 border-l-2 border-green-500 rounded-r flex items-center px-1">
                            <span class="text-xs text-green-700 font-medium truncate">Daily Standup</span>
                        </div>
                        <!-- 9:15-9:30 - Empty -->
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <!-- 9:30-9:45 - Quick Sync -->
                        <div class="h-3 bg-blue-100 border-l-2 border-blue-500 rounded-r flex items-center px-1">
                            <span class="text-xs text-blue-700 font-medium truncate">Quick Sync</span>
                        </div>
                        <!-- 9:45-10:00 - Empty -->
                        <div class="h-3 bg-gray-50 rounded"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 10:00 AM Hour Block with Long Event -->
        <div class="border-b border-gray-100">
            <div class="flex">
                <div class="w-16 p-2 text-xs text-gray-400 flex items-start border-r border-gray-100">
                    10:00
                </div>
                <div class="flex-1 p-1">
                    <div class="space-y-1">
                        <!-- 10:00-10:45 - Code Review (3 slots) -->
                        <div class="h-10 bg-purple-100 border-l-2 border-purple-500 rounded-r flex items-center px-2">
                            <div class="flex-1">
                                <div class="text-xs font-medium text-purple-700">Code Review Session</div>
                                <div class="text-xs text-gray-500">Login Feature - 45min</div>
                            </div>
                        </div>
                        <!-- 10:45-11:00 - Empty -->
                        <div class="h-3 bg-gray-50 rounded"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 11:00 AM Hour Block -->
        <div class="border-b border-gray-100">
            <div class="flex">
                <div class="w-16 p-2 text-xs text-gray-400 flex items-start border-r border-gray-100">
                    11:00
                </div>
                <div class="flex-1 p-1">
                    <div class="space-y-1">
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 12:00 PM Hour Block with Lunch -->
        <div class="border-b border-gray-100">
            <div class="flex">
                <div class="w-16 p-2 text-xs text-gray-400 flex items-start border-r border-gray-100">
                    12:00
                </div>
                <div class="flex-1 p-1">
                    <div class="space-y-1">
                        <!-- Full hour lunch -->
                        <div class="h-14 bg-gray-100 border-l-2 border-gray-400 rounded-r flex items-center px-2">
                            <div class="text-xs font-medium text-gray-600">Lunch Break</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 1:00 PM Hour Block -->
        <div class="border-b border-gray-100">
            <div class="flex">
                <div class="w-16 p-2 text-xs text-gray-400 flex items-start border-r border-gray-100">
                    1:00
                </div>
                <div class="flex-1 p-1">
                    <div class="space-y-1">
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 2:00 PM Hour Block - Current Time with Multiple Events -->
        <div class="border-b border-gray-100 bg-blue-50">
            <div class="flex">
                <div class="w-16 p-2 text-xs text-blue-600 flex items-start border-r border-gray-100 font-medium">
                    2:00
                </div>
                <div class="flex-1 p-1">
                    <div class="space-y-1">
                        <!-- 2:00-2:30 - All Hands Meeting (2 slots) -->
                        <div class="h-7 bg-green-100 border-l-2 border-green-500 rounded-r flex items-center px-2">
                            <div class="flex items-center justify-between w-full">
                                <div class="flex-1">
                                    <div class="text-xs font-medium text-green-700">All Hands Meeting</div>
                                    <div class="text-xs text-gray-500">Conference Room A</div>
                                </div>
                                <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                            </div>
                        </div>
                        <!-- 2:30-2:45 - Team Check-in -->
                        <div class="h-3 bg-gray-50 rounded"></div>

                        <!-- 2:45-3:00 - Empty -->
                        <div class="h-3 bg-yellow-100 border-l-2 border-yellow-500 rounded-r flex items-center px-1">
                            <span class="text-xs text-yellow-700 font-medium truncate">Team Check-in</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 3:00 PM Hour Block -->
        <div class="border-b border-gray-100">
            <div class="flex">
                <div class="w-16 p-2 text-xs text-gray-400 flex items-start border-r border-gray-100">
                    3:00
                </div>
                <div class="flex-1 p-1">
                    <div class="space-y-1">
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 4:00 PM Hour Block with Back-to-Back Events -->
        <div class="border-b border-gray-100">
            <div class="flex">
                <div class="w-16 p-2 text-xs text-gray-400 flex items-start border-r border-gray-100">
                    4:00
                </div>
                <div class="flex-1 p-1">
                    <div class="space-y-1">
                        <!-- 4:00-4:15 - Client Call -->
                        <div class="h-3 bg-orange-100 border-l-2 border-orange-500 rounded-r flex items-center px-1">
                            <span class="text-xs text-orange-700 font-medium truncate">Client Call</span>
                        </div>
                        <!-- 4:15-4:30 - Follow-up -->
                        <div class="h-3 bg-red-100 border-l-2 border-red-500 rounded-r flex items-center px-1">
                            <span class="text-xs text-red-700 font-medium truncate">Follow-up Notes</span>
                        </div>
                        <!-- 4:30-4:45 - Email Review -->
                        <div class="h-3 bg-indigo-100 border-l-2 border-indigo-500 rounded-r flex items-center px-1">
                            <span class="text-xs text-indigo-700 font-medium truncate">Email Review</span>
                        </div>
                        <!-- 4:45-5:00 - Planning -->
                        <div class="h-3 bg-pink-100 border-l-2 border-pink-500 rounded-r flex items-center px-1">
                            <span class="text-xs text-pink-700 font-medium truncate">Tomorrow Planning</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 5:00 PM Hour Block -->
        <div class="border-b border-gray-100">
            <div class="flex">
                <div class="w-16 p-2 text-xs text-gray-400 flex items-start border-r border-gray-100">
                    5:00
                </div>
                <div class="flex-1 p-1">
                    <div class="space-y-1">
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                        <div class="h-3 bg-gray-50 rounded"></div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <!-- Panel Footer -->
    <div class="p-3 border-t border-gray-200 bg-gray-50">
        <div class="flex items-center justify-between text-xs">
            <div class="flex items-center space-x-3">
                <div class="flex items-center space-x-1">
                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                    <span class="text-gray-600">6 meetings</span>
                </div>
                <div class="flex items-center space-x-1">
                    <div class="w-2 h-2 bg-orange-500 rounded-full"></div>
                    <span class="text-gray-600">2 calls</span>
                </div>
            </div>
            <button class="text-blue-600 hover:text-blue-700 font-medium">
                + Add Event
            </button>
        </div>
    </div>
</div>


        <!-- Column 2: Day Schedule Panel -->
<!--<div class="bg-white rounded-lg shadow-sm border border-gray-200 font-sans">-->
<!--    &lt;!&ndash; Panel Header &ndash;&gt;-->
<!--    <div class="p-4 border-b border-gray-200">-->
<!--        <div class="flex items-center justify-between">-->
<!--            <div>-->
<!--                <h3 class="text-lg font-semibold text-gray-800">Today's Schedule</h3>-->
<!--                <p class="text-sm text-gray-500">Tuesday, January 9, 2024</p>-->
<!--            </div>-->
<!--            <div class="flex items-center space-x-2">-->
<!--                <button class="p-1 hover:bg-gray-100 rounded text-gray-600">-->
<!--                    <span class="text-sm">←</span>-->
<!--                </button>-->
<!--                <button class="p-1 hover:bg-gray-100 rounded text-gray-600">-->
<!--                    <span class="text-sm">→</span>-->
<!--                </button>-->
<!--            </div>-->
<!--        </div>-->
<!--    </div>-->

<!--    &lt;!&ndash; Time Slots &ndash;&gt;-->
<!--    <div class="max-h-80 overflow-y-auto">-->
<!--        &lt;!&ndash; 8:00 AM &ndash;&gt;-->
<!--        <div class="flex border-b border-gray-100 h-12">-->
<!--            <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-100">-->
<!--                8:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; 9:00 AM &ndash;&gt;-->
<!--        <div class="flex border-b border-gray-100 h-12">-->
<!--            <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-100">-->
<!--                9:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                <div class="bg-green-50 border-l-2 border-green-500 px-2 py-1 h-full flex items-center rounded-r">-->
<!--                    <div>-->
<!--                        <div class="text-xs font-medium text-green-700">Daily Standup</div>-->
<!--                        <div class="text-xs text-gray-500">Team Alpha</div>-->
<!--                    </div>-->
<!--                </div>-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; 10:00 AM &ndash;&gt;-->
<!--        <div class="flex border-b border-gray-100 h-12">-->
<!--            <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-100">-->
<!--                10:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                <div class="bg-purple-50 border-l-2 border-purple-500 px-2 py-1 h-full flex items-center rounded-r">-->
<!--                    <div>-->
<!--                        <div class="text-xs font-medium text-purple-700">Code Review</div>-->
<!--                        <div class="text-xs text-gray-500">Login Feature</div>-->
<!--                    </div>-->
<!--                </div>-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; 11:00 AM &ndash;&gt;-->
<!--        <div class="flex border-b border-gray-100 h-12">-->
<!--            <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-100">-->
<!--                11:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; 12:00 PM &ndash;&gt;-->
<!--        <div class="flex border-b border-gray-100 h-12">-->
<!--            <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-100">-->
<!--                12:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                <div class="bg-gray-50 border-l-2 border-gray-400 px-2 py-1 h-full flex items-center rounded-r">-->
<!--                    <div>-->
<!--                        <div class="text-xs font-medium text-gray-600">Lunch Break</div>-->
<!--                    </div>-->
<!--                </div>-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; 1:00 PM &ndash;&gt;-->
<!--        <div class="flex border-b border-gray-100 h-12">-->
<!--            <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-100">-->
<!--                1:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; 2:00 PM - Current Meeting &ndash;&gt;-->
<!--        <div class="flex border-b border-gray-100 h-12 bg-blue-50">-->
<!--            <div class="w-16 p-2 text-xs text-blue-600 flex items-center border-r border-gray-100 font-medium">-->
<!--                2:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                <div class="bg-green-100 border-l-2 border-green-500 px-2 py-1 h-full flex items-center rounded-r">-->
<!--                    <div class="flex items-center justify-between w-full">-->
<!--                        <div>-->
<!--                            <div class="text-xs font-medium text-green-700">All Hands Meeting</div>-->
<!--                            <div class="text-xs text-gray-500">Conference Room A</div>-->
<!--                        </div>-->
<!--                        <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>-->
<!--                    </div>-->
<!--                </div>-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; 3:00 PM &ndash;&gt;-->
<!--        <div class="flex border-b border-gray-100 h-12">-->
<!--            <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-100">-->
<!--                3:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; 4:00 PM &ndash;&gt;-->
<!--        <div class="flex border-b border-gray-100 h-12">-->
<!--            <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-100">-->
<!--                4:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                <div class="bg-orange-50 border-l-2 border-orange-500 px-2 py-1 h-full flex items-center rounded-r">-->
<!--                    <div>-->
<!--                        <div class="text-xs font-medium text-orange-700">Client Call</div>-->
<!--                        <div class="text-xs text-gray-500">Project Update</div>-->
<!--                    </div>-->
<!--                </div>-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; 5:00 PM &ndash;&gt;-->
<!--        <div class="flex border-b border-gray-100 h-12">-->
<!--            <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-100">-->
<!--                5:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--            </div>-->
<!--        </div>-->

<!--        &lt;!&ndash; 6:00 PM &ndash;&gt;-->
<!--        <div class="flex h-12">-->
<!--            <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-100">-->
<!--                6:00-->
<!--            </div>-->
<!--            <div class="flex-1 p-2">-->
<!--                &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--            </div>-->
<!--        </div>-->
<!--    </div>-->

<!--    &lt;!&ndash; Panel Footer &ndash;&gt;-->
<!--    <div class="p-3 border-t border-gray-200 bg-gray-50">-->
<!--        <div class="flex items-center justify-between text-xs">-->
<!--            <div class="flex items-center space-x-4">-->
<!--                <div class="flex items-center space-x-1">-->
<!--                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>-->
<!--                    <span class="text-gray-600">4 meetings</span>-->
<!--                </div>-->
<!--                <div class="flex items-center space-x-1">-->
<!--                    <div class="w-2 h-2 bg-orange-500 rounded-full"></div>-->
<!--                    <span class="text-gray-600">1 call</span>-->
<!--                </div>-->
<!--            </div>-->
<!--            <button class="text-blue-600 hover:text-blue-700 font-medium">-->
<!--                + Add Event-->
<!--            </button>-->
<!--        </div>-->
<!--    </div>-->
<!--</div>-->

<!--- Old panel --->

<!--        <div class="bg-[#2a2f3e] rounded-lg border border-gray-700  text-white font-sans">-->
<!--            &lt;!&ndash; Panel Header &ndash;&gt;-->
<!--            <div class="p-4 border-b border-gray-700">-->
<!--                <div class="flex items-center justify-between">-->
<!--                    <div>-->
<!--                        <h3 class="text-lg font-semibold">Today's Schedule</h3>-->
<!--                        <p class="text-sm text-gray-400">Tuesday, January 9, 2024</p>-->
<!--                    </div>-->
<!--                    <div class="flex items-center space-x-2">-->
<!--                        <button class="p-1 hover:bg-[#2d3748] rounded">-->
<!--                            <span class="text-sm">←</span>-->
<!--                        </button>-->
<!--                        <button class="p-1 hover:bg-[#2d3748] rounded">-->
<!--                            <span class="text-sm">→</span>-->
<!--                        </button>-->
<!--                    </div>-->
<!--                </div>-->
<!--            </div>-->

<!--            &lt;!&ndash; Time Slots &ndash;&gt;-->
<!--            <div class="max-h-80 overflow-y-auto">-->
<!--                &lt;!&ndash; 8:00 AM &ndash;&gt;-->
<!--                <div class="flex border-b border-gray-700 h-12">-->
<!--                    <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-700">-->
<!--                        8:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--                    </div>-->
<!--                </div>-->

<!--                &lt;!&ndash; 9:00 AM &ndash;&gt;-->
<!--                <div class="flex border-b border-gray-700 h-12">-->
<!--                    <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-700">-->
<!--                        9:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        <div class="bg-[#48bb78]/20 border-l-2 border-[#48bb78] px-2 py-1 h-full flex items-center">-->
<!--                            <div>-->
<!--                                <div class="text-xs font-medium text-[#48bb78]">Daily Standup</div>-->
<!--                                <div class="text-xs text-gray-400">Team Alpha</div>-->
<!--                            </div>-->
<!--                            <div>-->
<!--                                <div class="text-xs font-medium text-[#48bb78]">Daily Standup</div>-->
<!--                                <div class="text-xs text-gray-400">Team Alpha</div>-->
<!--                            </div>-->
<!--                            <div>-->
<!--                                <div class="text-xs font-medium text-[#48bb78]">Daily Standup</div>-->
<!--                                <div class="text-xs text-gray-400">Team Alpha</div>-->
<!--                            </div>-->
<!--                            <div>-->
<!--                                <div class="text-xs font-medium text-[#48bb78]">Daily Standup</div>-->
<!--                                <div class="text-xs text-gray-400">Team Alpha</div>-->
<!--                            </div>-->
<!--                        </div>-->
<!--                    </div>-->
<!--                </div>-->

<!--                &lt;!&ndash; 10:00 AM &ndash;&gt;-->
<!--                <div class="flex border-b border-gray-700 h-12">-->
<!--                    <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-700">-->
<!--                        10:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        <div class="bg-[#9f7aea]/20 border-l-2 border-[#9f7aea] px-2 py-1 h-full flex items-center">-->
<!--                            <div>-->
<!--                                <div class="text-xs font-medium text-[#9f7aea]">Code Review</div>-->
<!--                                <div class="text-xs text-gray-400">Login Feature</div>-->
<!--                            </div>-->
<!--                        </div>-->
<!--                    </div>-->
<!--                </div>-->

<!--                &lt;!&ndash; 11:00 AM &ndash;&gt;-->
<!--                <div class="flex border-b border-gray-700 h-12">-->
<!--                    <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-700">-->
<!--                        11:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--                    </div>-->
<!--                </div>-->

<!--                &lt;!&ndash; 12:00 PM &ndash;&gt;-->
<!--                <div class="flex border-b border-gray-700 h-12">-->
<!--                    <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-700">-->
<!--                        12:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        <div class="bg-gray-600 bg-opacity-20 border-l-2 border-gray-500 px-2 py-1 h-full flex items-center">-->
<!--                            <div>-->
<!--                                <div class="text-xs font-medium text-gray-300">Lunch Break</div>-->
<!--                            </div>-->
<!--                        </div>-->
<!--                    </div>-->
<!--                </div>-->

<!--                &lt;!&ndash; 1:00 PM &ndash;&gt;-->
<!--                <div class="flex border-b border-gray-700 h-12">-->
<!--                    <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-700">-->
<!--                        1:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--                    </div>-->
<!--                </div>-->

<!--                &lt;!&ndash; 2:00 PM &ndash;&gt;-->
<!--                <div class="flex border-b border-gray-700 h-12 bg-[#2d3748] bg-opacity-10">-->
<!--                    <div class="w-16 p-2 text-xs text-[#4299e1] flex items-center border-r border-gray-700">-->
<!--                        2:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        <div class="bg-[#48bb78]/30 border-l-2 border-[#48bb78] px-2 py-1 h-full flex items-center">-->
<!--                            <div class="flex items-center justify-between w-full">-->
<!--                                <div>-->
<!--                                    <div class="text-xs font-medium text-[#48bb78]">All Hands Meeting</div>-->
<!--                                    <div class="text-xs text-gray-400">Conference Room A</div>-->
<!--                                </div>-->
<!--                                <div class="w-2 h-2 bg-[#48bb78] rounded-full animate-pulse"></div>-->
<!--                            </div>-->
<!--                        </div>-->
<!--                    </div>-->
<!--                </div>-->

<!--                &lt;!&ndash; 3:00 PM &ndash;&gt;-->
<!--                <div class="flex border-b border-gray-700 h-12">-->
<!--                    <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-700">-->
<!--                        3:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--                    </div>-->
<!--                </div>-->

<!--                &lt;!&ndash; 4:00 PM &ndash;&gt;-->
<!--                <div class="flex border-b border-gray-700 h-12">-->
<!--                    <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-700">-->
<!--                        4:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        <div class="bg-[#ed8936]/20 bg-opacity-20 border-l-2 border-[#ed8936] px-2 py-1 h-full flex items-center">-->
<!--                            <div>-->
<!--                                <div class="text-xs font-medium text-[#ed8936]">Client Call</div>-->
<!--                                <div class="text-xs text-gray-400">Project Update</div>-->
<!--                            </div>-->
<!--                        </div>-->
<!--                    </div>-->
<!--                </div>-->

<!--                &lt;!&ndash; 5:00 PM &ndash;&gt;-->
<!--                <div class="flex border-b border-gray-700 h-12">-->
<!--                    <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-700">-->
<!--                        5:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--                    </div>-->
<!--                </div>-->

<!--                &lt;!&ndash; 6:00 PM &ndash;&gt;-->
<!--                <div class="flex h-12">-->
<!--                    <div class="w-16 p-2 text-xs text-gray-400 flex items-center border-r border-gray-700">-->
<!--                        6:00-->
<!--                    </div>-->
<!--                    <div class="flex-1 p-2">-->
<!--                        &lt;!&ndash; Empty slot &ndash;&gt;-->
<!--                    </div>-->
<!--                </div>-->
<!--            </div>-->

<!--            &lt;!&ndash; Panel Footer &ndash;&gt;-->
<!--            <div class="p-3 border-t border-gray-700 bg-[#2d3748] bg-opacity-20">-->
<!--                <div class="flex items-center justify-between text-xs">-->
<!--                    <div class="flex items-center space-x-4">-->
<!--                        <div class="flex items-center space-x-1">-->
<!--                            <div class="w-2 h-2 bg-[#48bb78] rounded-full"></div>-->
<!--                            <span class="text-gray-400">4 meetings</span>-->
<!--                        </div>-->
<!--                        <div class="flex items-center space-x-1">-->
<!--                            <div class="w-2 h-2 bg-[#ed8936] rounded-full"></div>-->
<!--                            <span class="text-gray-400">1 call</span>-->
<!--                        </div>-->
<!--                    </div>-->
<!--                    <button class="text-[#4299e1] hover:text-blue-300 font-medium">-->
<!--                        + Add Event-->
<!--                    </button>-->
<!--                </div>-->
<!--            </div>-->
<!--        </div>-->






