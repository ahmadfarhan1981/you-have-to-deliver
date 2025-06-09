<script lang="ts">
    import {onMount} from 'svelte';

    import {invoke} from "@tauri-apps/api/core";
    import {goto} from "$app/navigation";
    import {gameSpeed} from "$lib/stores/gameSpeed.js";
    import NewGamePanel from "$lib/components/NewGamePanel.svelte";

    // State management
    let selectedMenu = "New Game";

    // Recent games data
    const recentGames = [
        {name: "TechGiants Inc.", day: 42, quarter: "Q2", year: 2025, timeAgo: "2 hours ago"},
        {name: "Startup Hustle", day: 156, quarter: "Q4", year: 2024, timeAgo: "Yesterday"},
        {name: "CodeCraft Solutions", day: 78, quarter: "Q3", year: 2025, timeAgo: "3 days ago"}
    ];

    // Menu options
    const menuOptions = [
        {id: "New Game", icon: "▶", color: "green"},
        {id: "Load Game", icon: "↻", color: "blue"},
        {id: "Settings", icon: "⚙", color: "purple"},
        {id: "Tutorial", icon: "?", color: "amber"},
        {id: "Exit", icon: "✕", color: "red"}
    ];

    // Handle menu selection
    function selectMenu(menuId) {
        selectedMenu = menuId;
    }


    // Handle loading a saved game
    function loadGame(game) {
        console.log(`Loading game: ${game.name}`);
        // Load the saved game state and navigate to game screen
    }

    // Terminal cursor blinking effect
    let showCursor = true;

    onMount(() => {
        const cursorInterval = setInterval(() => {
            showCursor = !showCursor;
        }, 500);

        return () => {
            clearInterval(cursorInterval);
        };
    });


</script>

<div class="grid-bg min-h-screen flex flex-col">
    <div class="flex-1 flex items-center justify-center">
        <div class="max-w-4xl w-full mx-auto p-6">
            <div class="bg-slate-900 border border-slate-700 rounded-lg shadow-2xl overflow-hidden">
                <!-- Header Bar -->
                <div class="bg-slate-800 px-4 py-2 flex items-center justify-between border-b border-slate-700">
                    <div class="flex items-center">
                        <div class="w-3 h-3 rounded-full bg-red-500 mr-2"></div>
                        <div class="w-3 h-3 rounded-full bg-amber-500 mr-2"></div>
                        <div class="w-3 h-3 rounded-full bg-green-500"></div>
                    </div>
                    <div class="text-slate-400 text-xs">terminal@devcorp:~</div>
                    <div></div>
                </div>

                <!-- Main Content -->
                <div class="p-8 text-slate-200">
                    <!-- Logo/Title -->
                    <div class="mb-10 text-center">
                        <div class="text-4xl font-bold tracking-tight mb-1">DevCorp<span
                                class="text-green-500">Sim</span></div>
                        <div class="text-sm text-slate-400">Software Company Simulation</div>
                    </div>

                    <!-- Terminal-style welcome message -->
                    <div class="bg-slate-800 border border-slate-700 rounded p-4 mb-8 font-mono text-sm">
                        <div class="text-green-400 mb-1">$ ./welcome.sh</div>
                        <div class="text-slate-300">Welcome to DevCorp Simulation v1.0.42</div>
                        <div class="text-slate-300">Initializing corporate simulation environment...</div>
                        <div class="text-green-400 mt-1">$ _
                            {#if showCursor}<span class="terminal-cursor"></span>{/if}
                        </div>
                        <div class="text-green-400 mt-1">{$gameSpeed.game_speed} {$gameSpeed.tick.tick}
                            Tick, {$gameSpeed.tick.quarter_tick} day tick , {$gameSpeed.tick.day}
                            Day, {$gameSpeed.tick.week} Week
                        </div>
                        <button
                                class="px-3 py-2 border-2" on:click={async () => await invoke('new_sim',{company:companyPresets[selectedCompanyIndex], employee:employeeConfigs[selectedEmployeeConfigIndex] })}
                        >Reset sim
                        </button>
                        <button
                                class="px-3 py-2 border-2" on:click={async () => goto("/game")}
                        >Goto game page
                        </button>
                        <button
                                class="px-3 py-2 border-2" on:click={async () => {await invoke('resume_sim');} }
                        >Start
                        </button>
                        <button
                                class="px-3 py-2 border-2" on:click={async () => await invoke('increase_speed')}
                        >+
                        </button>
                        <button
                                class="px-3 py-2 border-2" on:click={async ()=> await invoke('decrease_speed')}
                        >-
                        </button>
                        <button
                                class="px-3 py-2 border-2" on:click={async () => await invoke('stop_sim')}
                        >Stop
                        </button>
                        <button
                                class="px-3 py-2 border-2"
                                on:click={async () => await invoke('new_team',{teamName:"new team name", description:"This is a team"})}
                        >New team
                        </button>
                        <button
                                class="px-3 py-2 border-2"
                                on:click={async () => await invoke('assign_person_to_team',{teamId:1, personId:1})}
                        >add
                        </button>
                        <button
                                class="px-3 py-2 border-2" on:click={async () => await invoke('refresh_data')}
                        >refresh
                        </button>

                    </div>

                    <!-- Menu Options -->
                    <div class="grid grid-cols-5 gap-6">
                        <!-- Left Column - Menu -->
                        <div class="col-span-2">
                            <div class="space-y-4">
                                <div class="text-xs uppercase text-slate-500 font-bold mb-2">Main Menu</div>

                                {#each menuOptions as option}
                                    <button
                                            class="menu-item w-full text-left px-4 py-3 bg-slate-800 hover:bg-slate-700 border-l-4 border-{option.color}-500 rounded flex items-center text-{option.color}-400 font-medium {selectedMenu === option.id ? 'bg-slate-700' : ''}"
                                            on:click={() => selectMenu(option.id)}
                                    >
                                        <span class="mr-3">{option.icon}</span>
                                        <span>{option.id}</span>
                                    </button>
                                {/each}
                            </div>

                            <!-- Recent Games -->
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
                        </div>

                        <!-- Right Column - Game Info -->
                        <div class="col-span-3 bg-slate-800 rounded-lg border border-slate-700 p-5 flex flex-col">
                            {#if selectedMenu === "New Game"}
                                <NewGamePanel />
                            {:else if selectedMenu === "Load Game"}
                                <div class="text-lg font-bold mb-3">Load Game</div>
                                <div class="text-sm text-slate-300 mb-4">
                                    Continue where you left off with one of your existing companies.
                                </div>

                                <div class="space-y-3 mb-6">
                                    {#each recentGames as game}
                                        <div class="p-4 bg-slate-900 border border-slate-700 rounded hover:bg-slate-700 cursor-pointer">
                                            <div class="font-medium text-lg">{game.name}</div>
                                            <div class="text-sm text-slate-400 mt-1">Day {game.day}
                                                • {game.quarter} {game.year}</div>
                                            <div class="mt-2 flex justify-between items-center">
                                                <div class="text-xs text-slate-500">Last played: {game.timeAgo}</div>
                                                <button class="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded text-xs text-white">
                                                    Load
                                                </button>
                                            </div>
                                        </div>
                                    {/each}
                                </div>

                                <div class="mt-auto">
                                    <button class="w-full py-3 bg-slate-700 hover:bg-slate-600 rounded font-medium text-slate-300 flex items-center justify-center">
                                        <span>Browse All Saved Games</span>
                                    </button>
                                </div>

                            {:else if selectedMenu === "Settings"}
                                <div class="text-lg font-bold mb-3">Settings</div>
                                <div class="text-sm text-slate-300 mb-4">
                                    Customize your game experience and adjust system settings.
                                </div>

                                <div class="space-y-4 mb-6">
                                    <div>
                                        <label class="block text-xs text-slate-400 mb-1">Game Speed</label>
                                        <div class="w-full bg-slate-900 rounded h-2 mt-2">
                                            <div class="bg-purple-500 h-2 rounded" style="width: 60%"></div>
                                        </div>
                                        <div class="flex justify-between text-xs text-slate-500 mt-1">
                                            <span>Slow</span>
                                            <span>Normal</span>
                                            <span>Fast</span>
                                        </div>
                                    </div>

                                    <div>
                                        <label class="block text-xs text-slate-400 mb-1">Audio</label>
                                        <div class="flex items-center space-x-2 mt-2">
                                            <div class="text-xs text-slate-300 w-24">Music</div>
                                            <div class="flex-1 bg-slate-900 rounded h-2">
                                                <div class="bg-purple-500 h-2 rounded" style="width: 70%"></div>
                                            </div>
                                        </div>
                                        <div class="flex items-center space-x-2 mt-2">
                                            <div class="text-xs text-slate-300 w-24">Sound Effects</div>
                                            <div class="flex-1 bg-slate-900 rounded h-2">
                                                <div class="bg-purple-500 h-2 rounded" style="width: 80%"></div>
                                            </div>
                                        </div>
                                    </div>

                                    <div>
                                        <label class="block text-xs text-slate-400 mb-1">Display</label>
                                        <div class="grid grid-cols-2 gap-2 mt-2">
                                            <button class="px-3 py-2 bg-purple-900 border border-purple-700 rounded text-center text-purple-400">
                                                Windowed
                                            </button>
                                            <button class="px-3 py-2 bg-slate-900 border border-slate-700 rounded text-center hover:bg-slate-700">
                                                Fullscreen
                                            </button>
                                        </div>
                                    </div>
                                </div>

                                <div class="mt-auto">
                                    <button class="w-full py-3 bg-purple-600 hover:bg-purple-700 rounded font-bold text-white flex items-center justify-center">
                                        <span>Save Settings</span>
                                    </button>
                                </div>

                            {:else if selectedMenu === "Tutorial"}
                                <div class="text-lg font-bold mb-3">Tutorial</div>
                                <div class="text-sm text-slate-300 mb-4">
                                    Learn how to play DevCorp Sim and master the art of software company management.
                                </div>

                                <div class="space-y-3 mb-6">
                                    <div class="p-3 bg-slate-900 border border-slate-700 rounded hover:bg-slate-700 cursor-pointer">
                                        <div class="font-medium">Getting Started</div>
                                        <div class="text-xs text-slate-400 mt-1">Learn the basics of company
                                            management
                                        </div>
                                    </div>
                                    <div class="p-3 bg-slate-900 border border-slate-700 rounded hover:bg-slate-700 cursor-pointer">
                                        <div class="font-medium">Employee Management</div>
                                        <div class="text-xs text-slate-400 mt-1">How to hire, train and manage your
                                            team
                                        </div>
                                    </div>
                                    <div class="p-3 bg-slate-900 border border-slate-700 rounded hover:bg-slate-700 cursor-pointer">
                                        <div class="font-medium">Project Development</div>
                                        <div class="text-xs text-slate-400 mt-1">Taking on and completing software
                                            projects
                                        </div>
                                    </div>
                                    <div class="p-3 bg-slate-900 border border-slate-700 rounded hover:bg-slate-700 cursor-pointer">
                                        <div class="font-medium">Financial Management</div>
                                        <div class="text-xs text-slate-400 mt-1">Budgeting, investments and profit
                                            strategies
                                        </div>
                                    </div>
                                </div>

                                <div class="mt-auto">
                                    <button class="w-full py-3 bg-amber-600 hover:bg-amber-700 rounded font-bold text-white flex items-center justify-center">
                                        <span>Start Interactive Tutorial</span>
                                    </button>
                                </div>

                            {:else if selectedMenu === "Exit"}
                                <div class="text-lg font-bold mb-3">Exit Game</div>
                                <div class="text-sm text-slate-300 mb-4">
                                    Are you sure you want to exit DevCorp Sim?
                                </div>

                                <div class="flex-1 flex items-center justify-center">
                                    <div class="text-center">
                                        <div class="text-5xl mb-4">👋</div>
                                        <div class="text-slate-400">Thanks for playing!</div>
                                    </div>
                                </div>

                                <div class="mt-auto grid grid-cols-2 gap-3">
                                    <button class="py-3 bg-slate-700 hover:bg-slate-600 rounded font-medium text-slate-300">
                                        Cancel
                                    </button>
                                    <button class="py-3 bg-red-600 hover:bg-red-700 rounded font-bold text-white">
                                        Exit Game
                                    </button>
                                </div>
                            {/if}
                        </div>
                    </div>
                </div>

                <!-- Footer -->
                <div class="bg-slate-900 border-t border-slate-700 px-4 py-2 flex justify-between items-center text-xs text-slate-500">
                    <div class="version-number">v1.0.42 build 2025.05.17</div>
                    <div class="flex items-center space-x-4">
                        <span>© 2025 DevCorp Sim</span>
                        <a href="#" class="hover:text-slate-300">Credits</a>
                        <a href="#" class="hover:text-slate-300">Support</a>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    :global(body) {
        font-family: 'IBM Plex Mono', monospace;
        background-color: #1e293b; /* slate-800 */
    }

    .grid-bg {
        background-image: url("images/bg/bg.png");

    }

    .menu-item {
        transition: all 0.2s ease;
    }

    .menu-item:hover {
        transform: translateX(8px);
    }

    .version-number {
        font-family: 'IBM Plex Mono', monospace;
        font-size: 10px;
        opacity: 0.6;
    }

    .terminal-cursor {
        display: inline-block;
        width: 10px;
        height: 18px;
        background-color: #e2e8f0;
        animation: blink 1s step-end infinite;
        vertical-align: middle;
        margin-left: 4px;
    }

    @keyframes blink {
        0%, 100% {
            opacity: 1;
        }
        50% {
            opacity: 0;
        }
    }
</style>