<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DevCorp Sim - Company Schedule</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        tailwind.config = {
            theme: {
                extend: {
                    colors: {
                        'devcorp-dark': '#1a1f2e',
                        'devcorp-darker': '#151a26',
                        'devcorp-blue': '#2d3748',
                        'devcorp-accent': '#4299e1',
                        'devcorp-green': '#48bb78',
                        'devcorp-orange': '#ed8936',
                        'devcorp-red': '#f56565',
                        'devcorp-purple': '#9f7aea'
                    }
                }
            }
        }
    </script>
</head>

<body class="bg-devcorp-darker text-white font-sans">

<div class="flex h-screen">


    <!-- Main Content -->
    <div class="flex-1 flex flex-col">
        <!-- Header -->
        <div class="bg-devcorp-dark border-b border-gray-700 px-6 py-4">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-4">
                    <h2 class="text-2xl font-bold">Company Schedule</h2>
                    <div class="flex items-center space-x-2 text-sm text-gray-400">
                        <span class="bg-devcorp-green text-white px-2 py-1 rounded text-xs">Stable</span>
                        <span>Day 9 • Q1 Year: 0</span>
                    </div>
                </div>
                <div class="flex items-center space-x-4">
                    <button class="bg-devcorp-accent hover:bg-blue-600 px-4 py-2 rounded text-sm font-medium">
                        + New Event
                    </button>
                    <div class="flex bg-devcorp-blue rounded">
                        <button id="monthView" class="px-4 py-2 text-sm font-medium bg-devcorp-accent rounded-l">Month</button>
                        <button id="weekView" class="px-4 py-2 text-sm font-medium hover:bg-devcorp-accent rounded-r">Week</button>
                    </div>
                </div>
            </div>
        </div>

        <!-- Calendar Content -->
        <div class="flex-1 p-6">
            <!-- Calendar Header -->
            <div class="flex items-center justify-between mb-6">
                <div class="flex items-center space-x-4">
                    <button class="p-2 hover:bg-devcorp-blue rounded">
                        <span class="text-xl">←</span>
                    </button>
                    <h3 id="calendarTitle" class="text-xl font-semibold">January 2024</h3>
                    <button class="p-2 hover:bg-devcorp-blue rounded">
                        <span class="text-xl">→</span>
                    </button>
                </div>
                <div class="flex items-center space-x-4">
                    <div class="flex items-center space-x-2 text-sm">
                        <div class="w-3 h-3 bg-devcorp-green rounded"></div>
                        <span>Team Meetings</span>
                    </div>
                    <div class="flex items-center space-x-2 text-sm">
                        <div class="w-3 h-3 bg-devcorp-purple rounded"></div>
                        <span>Ceremonies</span>
                    </div>
                    <div class="flex items-center space-x-2 text-sm">
                        <div class="w-3 h-3 bg-devcorp-orange rounded"></div>
                        <span>Deadlines</span>
                    </div>
                    <div class="flex items-center space-x-2 text-sm">
                        <div class="w-3 h-3 bg-devcorp-red rounded"></div>
                        <span>Critical</span>
                    </div>
                </div>
            </div>

            <!-- Monthly Calendar View -->
            <div id="monthlyView" class="bg-devcorp-dark rounded-lg border border-gray-700">
                <!-- Days of Week Header -->
                <div class="grid grid-cols-7 border-b border-gray-700">
                    <div class="p-4 text-center font-medium text-gray-400">Sun</div>
                    <div class="p-4 text-center font-medium text-gray-400">Mon</div>
                    <div class="p-4 text-center font-medium text-gray-400">Tue</div>
                    <div class="p-4 text-center font-medium text-gray-400">Wed</div>
                    <div class="p-4 text-center font-medium text-gray-400">Thu</div>
                    <div class="p-4 text-center font-medium text-gray-400">Fri</div>
                    <div class="p-4 text-center font-medium text-gray-400">Sat</div>
                </div>

                <!-- Calendar Days -->
                <div class="grid grid-cols-7">
                    <!-- Week 1 -->
                    <div class="h-32 p-2 border-r border-b border-gray-700 text-gray-500">
                        <div class="text-sm">31</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">1</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-green text-xs px-2 py-1 rounded mb-1">Team Alpha Standup</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">2</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">3</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-purple text-xs px-2 py-1 rounded mb-1">Sprint Planning</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">4</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">5</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-orange text-xs px-2 py-1 rounded mb-1">Project Deadline</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-b border-gray-700">
                        <div class="text-sm font-medium">6</div>
                    </div>

                    <!-- Week 2 -->
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">7</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">8</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-green text-xs px-2 py-1 rounded mb-1">Team Bravo Standup</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700 bg-devcorp-blue bg-opacity-30">
                        <div class="text-sm font-medium text-devcorp-accent">9</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-green text-xs px-2 py-1 rounded mb-1">All Hands Meeting</div>
                            <div class="bg-devcorp-purple text-xs px-2 py-1 rounded">Code Review</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">10</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">11</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">12</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-red text-xs px-2 py-1 rounded mb-1">Critical Review</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-b border-gray-700">
                        <div class="text-sm font-medium">13</div>
                    </div>

                    <!-- Week 3 -->
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">14</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">15</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-green text-xs px-2 py-1 rounded mb-1">Team Sync</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">16</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">17</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-purple text-xs px-2 py-1 rounded mb-1">Sprint Retro</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">18</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">19</div>
                    </div>
                    <div class="h-32 p-2 border-b border-gray-700">
                        <div class="text-sm font-medium">20</div>
                    </div>

                    <!-- Week 4 -->
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">21</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">22</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-green text-xs px-2 py-1 rounded mb-1">Weekly Review</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">23</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">24</div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">25</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-orange text-xs px-2 py-1 rounded mb-1">Milestone Due</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-r border-b border-gray-700">
                        <div class="text-sm font-medium">26</div>
                    </div>
                    <div class="h-32 p-2 border-b border-gray-700">
                        <div class="text-sm font-medium">27</div>
                    </div>

                    <!-- Week 5 -->
                    <div class="h-32 p-2 border-r border-gray-700">
                        <div class="text-sm font-medium">28</div>
                    </div>
                    <div class="h-32 p-2 border-r border-gray-700">
                        <div class="text-sm font-medium">29</div>
                    </div>
                    <div class="h-32 p-2 border-r border-gray-700">
                        <div class="text-sm font-medium">30</div>
                        <div class="mt-1">
                            <div class="bg-devcorp-purple text-xs px-2 py-1 rounded mb-1">Monthly Review</div>
                        </div>
                    </div>
                    <div class="h-32 p-2 border-r border-gray-700">
                        <div class="text-sm font-medium">31</div>
                    </div>
                    <div class="h-32 p-2 border-r border-gray-700 text-gray-500">
                        <div class="text-sm">1</div>
                    </div>
                    <div class="h-32 p-2 border-r border-gray-700 text-gray-500">
                        <div class="text-sm">2</div>
                    </div>
                    <div class="h-32 p-2 text-gray-500">
                        <div class="text-sm">3</div>
                    </div>
                </div>
            </div>

            <!-- Weekly Calendar View -->
            <div id="weeklyView" class="bg-devcorp-dark rounded-lg border border-gray-700 ">
                <!-- Week Header -->
                <div class="grid grid-cols-8 border-b border-gray-700">
                    <div class="p-4 text-center font-medium text-gray-400">Time</div>
                    <div class="p-4 text-center font-medium text-gray-400 border-l border-gray-700">
                        <div>Sun</div>
                        <div class="text-xs text-gray-500">Jan 7</div>
                    </div>
                    <div class="p-4 text-center font-medium text-gray-400 border-l border-gray-700">
                        <div>Mon</div>
                        <div class="text-xs text-gray-500">Jan 8</div>
                    </div>
                    <div class="p-4 text-center font-medium text-devcorp-accent border-l border-gray-700 bg-devcorp-blue bg-opacity-30">
                        <div>Tue</div>
                        <div class="text-xs">Jan 9</div>
                    </div>
                    <div class="p-4 text-center font-medium text-gray-400 border-l border-gray-700">
                        <div>Wed</div>
                        <div class="text-xs text-gray-500">Jan 10</div>
                    </div>
                    <div class="p-4 text-center font-medium text-gray-400 border-l border-gray-700">
                        <div>Thu</div>
                        <div class="text-xs text-gray-500">Jan 11</div>
                    </div>
                    <div class="p-4 text-center font-medium text-gray-400 border-l border-gray-700">
                        <div>Fri</div>
                        <div class="text-xs text-gray-500">Jan 12</div>
                    </div>
                    <div class="p-4 text-center font-medium text-gray-400 border-l border-gray-700">
                        <div>Sat</div>
                        <div class="text-xs text-gray-500">Jan 13</div>
                    </div>
                </div>

                <!-- Time Slots -->
                <div class="max-h-96 overflow-y-auto">
                    <!-- 8:00 AM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">8:00 AM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1">
                            <div class="bg-devcorp-green text-xs px-2 py-1 rounded h-full flex items-center">Team Bravo Standup</div>
                        </div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="p-1"></div>
                    </div>

                    <!-- 9:00 AM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">9:00 AM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1">
                            <div class="bg-devcorp-red text-xs px-2 py-1 rounded h-full flex items-center">Critical Review</div>
                        </div>
                        <div class="p-1"></div>
                    </div>

                    <!-- 10:00 AM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">10:00 AM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="p-1"></div>
                    </div>

                    <!-- 11:00 AM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">11:00 AM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="p-1"></div>
                    </div>

                    <!-- 12:00 PM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">12:00 PM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="p-1"></div>
                    </div>

                    <!-- 1:00 PM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">1:00 PM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="p-1"></div>
                    </div>

                    <!-- 2:00 PM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">2:00 PM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20">
                            <div class="bg-devcorp-green text-xs px-2 py-1 rounded h-full flex items-center">All Hands Meeting</div>
                        </div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="p-1"></div>
                    </div>

                    <!-- 3:00 PM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">3:00 PM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="p-1"></div>
                    </div>

                    <!-- 4:00 PM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">4:00 PM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20">
                            <div class="bg-devcorp-purple text-xs px-2 py-1 rounded h-full flex items-center">Code Review</div>
                        </div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="p-1"></div>
                    </div>

                    <!-- 5:00 PM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">5:00 PM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="p-1"></div>
                    </div>

                    <!-- 6:00 PM -->
                    <div class="grid grid-cols-8 border-b border-gray-700 h-16">
                        <div class="p-2 text-sm text-gray-400 border-r border-gray-700 flex items-center">6:00 PM</div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1 bg-devcorp-blue bg-opacity-20"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="border-r border-gray-700 p-1"></div>
                        <div class="p-1"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Upcoming Events Sidebar -->
        <div class="w-80 bg-devcorp-dark border-l border-gray-700 p-6">
            <h3 class="text-lg font-semibold mb-4">Upcoming Events</h3>
            <div class="space-y-3">
                <div class="bg-devcorp-blue p-3 rounded border-l-4 border-devcorp-green">
                    <div class="flex justify-between items-start">
                        <div>
                            <div class="font-medium text-sm">All Hands Meeting</div>
                            <div class="text-xs text-gray-400">Today, 2:00 PM</div>
                            <div class="text-xs text-gray-400">Conference Room A</div>
                        </div>
                        <span class="bg-devcorp-green text-xs px-2 py-1 rounded">Meeting</span>
                    </div>
                </div>

                <div class="bg-devcorp-blue p-3 rounded border-l-4 border-devcorp-purple">
                    <div class="flex justify-between items-start">
                        <div>
                            <div class="font-medium text-sm">Code Review Session</div>
                            <div class="text-xs text-gray-400">Today, 4:00 PM</div>
                            <div class="text-xs text-gray-400">Team Alpha</div>
                        </div>
                        <span class="bg-devcorp-purple text-xs px-2 py-1 rounded">Ceremony</span>
                    </div>
                </div>

                <div class="bg-devcorp-blue p-3 rounded border-l-4 border-devcorp-orange">
                    <div class="flex justify-between items-start">
                        <div>
                            <div class="font-medium text-sm">Project Deadline</div>
                            <div class="text-xs text-gray-400">Jan 5, 11:59 PM</div>
                            <div class="text-xs text-gray-400">Login Feature</div>
                        </div>
                        <span class="bg-devcorp-orange text-xs px-2 py-1 rounded">Deadline</span>
                    </div>
                </div>

                <div class="bg-devcorp-blue p-3 rounded border-l-4 border-devcorp-red">
                    <div class="flex justify-between items-start">
                        <div>
                            <div class="font-medium text-sm">Critical Review</div>
                            <div class="text-xs text-gray-400">Jan 12, 9:00 AM</div>
                            <div class="text-xs text-gray-400">Performance Issues</div>
                        </div>
                        <span class="bg-devcorp-red text-xs px-2 py-1 rounded">Critical</span>
                    </div>
                </div>
            </div>

            <div class="mt-6">
                <h4 class="text-sm font-semibold mb-3">Quick Actions</h4>
                <div class="space-y-2">
                    <button class="w-full text-left p-2 hover:bg-devcorp-blue rounded text-sm">
                        📅 Schedule Team Meeting
                    </button>
                    <button class="w-full text-left p-2 hover:bg-devcorp-blue rounded text-sm">
                        🔄 Plan Sprint Ceremony
                    </button>
                    <button class="w-full text-left p-2 hover:bg-devcorp-blue rounded text-sm">
                        ⏰ Set Project Deadline
                    </button>
                    <button class="w-full text-left p-2 hover:bg-devcorp-blue rounded text-sm">
                        🚨 Schedule Critical Review
                    </button>
                </div>
            </div>
        </div>
    </div>
</div>

<script>
    // View toggle functionality
    document.getElementById('monthView').addEventListener('click', function() {
        // Update button styles
        this.classList.add('bg-devcorp-accent');
        this.classList.remove('hover:bg-devcorp-accent');
        document.getElementById('weekView').classList.remove('bg-devcorp-accent');
        document.getElementById('weekView').classList.add('hover:bg-devcorp-accent');

        // Show/hide views
        document.getElementById('monthlyView').classList.remove('hidden');
        document.getElementById('weeklyView').classList.add('hidden');

        // Update title
        document.getElementById('calendarTitle').textContent = 'January 2024';
    });

    document.getElementById('weekView').addEventListener('click', function() {
        // Update button styles
        this.classList.add('bg-devcorp-accent');
        this.classList.remove('hover:bg-devcorp-accent');
        document.getElementById('monthView').classList.remove('bg-devcorp-accent');
        document.getElementById('monthView').classList.add('hover:bg-devcorp-accent');

        // Show/hide views
        document.getElementById('monthlyView').classList.add('hidden');
        document.getElementById('weeklyView').classList.remove('hidden');

        // Update title
        document.getElementById('calendarTitle').textContent = 'Week of January 7-13, 2024';
    });
</script>
</body>
</html>