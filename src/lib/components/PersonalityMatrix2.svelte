<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Compact Personality Matrix Panel</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        tailwind.config = {
            theme: {
                extend: {
                    colors: {
                        'game-bg': '#6b7280',
                        'panel-bg': '#ffffff'
                    }
                }
            }
        }
    </script>
</head>
<body class="bg-game-bg p-4">
<!-- Compact Personality Matrix Panel -->
<div class="bg-panel-bg rounded-lg p-4 shadow-lg max-w-md">
    <h3 class="text-lg font-semibold text-gray-900 mb-3">Personality Matrix</h3>

    <div class="space-y-3">
        <!-- Assertiveness -->
        <div>
<!--            <div class="flex justify-between items-center mb-1">-->
<!--                <span class="text-sm font-medium text-gray-800">Assertiveness</span>-->
<!--                <span class="text-sm font-bold text-gray-900">+35</span>-->
<!--            </div>-->
            <div class="relative mb-1">
                <div class="w-full h-2 bg-gray-200 rounded-full relative">
                    <div class="absolute left-1/2 top-0 w-0.5 h-2 bg-gray-400 transform -translate-x-0.5"></div>
                    <div class="absolute left-0 top-0 w-1/2 h-2 bg-blue-100 rounded-l-full"></div>
                    <div class="absolute right-0 top-0 w-1/2 h-2 bg-orange-100 rounded-r-full"></div>
                    <div class="absolute top-0 h-2 w-2 bg-orange-500 rounded-full transform -translate-x-0.5" style="left: 67.5%"></div>
                </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600">
                <span>Accommodating</span>
                <span>Dominant</span>
            </div>
        </div>

        <!-- Structure Preference -->
        <div>
<!--            <div class="flex justify-between items-center mb-1">-->
<!--                <span class="text-sm font-medium text-gray-800">Structure Preference</span>-->
<!--                <span class="text-sm font-bold text-gray-900">-22</span>-->
<!--            </div>-->
            <div class="relative mb-1">
                <div class="w-full h-2 bg-gray-200 rounded-full relative">
                    <div class="absolute left-1/2 top-0 w-0.5 h-2 bg-gray-400 transform -translate-x-0.5"></div>
                    <div class="absolute left-0 top-0 w-1/2 h-2 bg-blue-100 rounded-l-full"></div>
                    <div class="absolute right-0 top-0 w-1/2 h-2 bg-green-100 rounded-r-full"></div>
                    <div class="absolute top-0 h-2 w-2 bg-blue-500 rounded-full transform -translate-x-0.5" style="left: 39%"></div>
                </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600">
                <span>Spontaneous</span>
                <span>Methodical</span>
            </div>
        </div>

        <!-- Openness -->
        <div>
<!--            <div class="flex justify-between items-center mb-1">-->
<!--                <span class="text-sm font-medium text-gray-800">Openness</span>-->
<!--                <span class="text-sm font-bold text-gray-900">+58</span>-->
<!--            </div>-->
            <div class="relative mb-1">
                <div class="w-full h-2 bg-gray-200 rounded-full relative">
                    <div class="absolute left-1/2 top-0 w-0.5 h-2 bg-gray-400 transform -translate-x-0.5"></div>
                    <div class="absolute left-0 top-0 w-1/2 h-2 bg-blue-100 rounded-l-full"></div>
                    <div class="absolute right-0 top-0 w-1/2 h-2 bg-purple-100 rounded-r-full"></div>
                    <div class="absolute top-0 h-2 w-2 bg-purple-500 rounded-full transform -translate-x-0.5" style="left: 79%"></div>
                </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600">
                <span>Practical</span>
                <span>Idealistic</span>
            </div>
        </div>

        <!-- Sociability -->
        <div>
<!--            <div class="flex justify-between items-center mb-1">-->
<!--                <span class="text-sm font-medium text-gray-800">Sociability</span>-->
<!--                <span class="text-sm font-bold text-gray-900">-45</span>-->
<!--            </div>-->
            <div class="relative mb-1">
                <div class="w-full h-2 bg-gray-200 rounded-full relative">
                    <div class="absolute left-1/2 top-0 w-0.5 h-2 bg-gray-400 transform -translate-x-0.5"></div>
                    <div class="absolute left-0 top-0 w-1/2 h-2 bg-blue-100 rounded-l-full"></div>
                    <div class="absolute right-0 top-0 w-1/2 h-2 bg-pink-100 rounded-r-full"></div>
                    <div class="absolute top-0 h-2 w-2 bg-blue-500 rounded-full transform -translate-x-0.5" style="left: 27.5%"></div>
                </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600">
                <span>Reserved</span>
                <span>Outgoing</span>
            </div>
        </div>

        <!-- Influence -->
        <div>
<!--            <div class="flex justify-between items-center mb-1">-->
<!--                <span class="text-sm font-medium text-gray-800">Influence</span>-->
<!--                <span class="text-sm font-bold text-gray-900">+12</span>-->
<!--            </div>-->
            <div class="relative mb-1">
                <div class="w-full h-2 bg-gray-200 rounded-full relative">
                    <div class="absolute left-1/2 top-0 w-0.5 h-2 bg-gray-400 transform -translate-x-0.5"></div>
                    <div class="absolute left-0 top-0 w-1/2 h-2 bg-blue-100 rounded-l-full"></div>
                    <div class="absolute right-0 top-0 w-1/2 h-2 bg-cyan-100 rounded-r-full"></div>
                    <div class="absolute top-0 h-2 w-2 bg-cyan-500 rounded-full transform -translate-x-0.5" style="left: 56%"></div>
                </div>
            </div>
            <div class="flex justify-between text-xs text-gray-600">
                <span>Suggestive</span>
                <span>Persuasive</span>
            </div>
        </div>
    </div>

    <!-- Compact Summary -->
    <div class="mt-3 pt-3 border-t border-gray-200">
        <div class="text-xs text-gray-600 leading-tight">
            <span class="font-medium">Profile:</span> Moderately assertive, flexible, idealistic, reserved, balanced influence
        </div>
    </div>
</div>
</body>
</html>