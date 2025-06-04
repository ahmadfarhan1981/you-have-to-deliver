<script>
    import {
        BarChart2,
        Code,
        CreditCard,
        HelpCircle,
        LayoutDashboard,
        Pause,
        Play,
        Settings,
        SkipForward,
        Users
    } from "lucide-svelte";
    import {tabState, activeTab} from "$lib/stores/TabStore.js";
    import {goto} from "$app/navigation";
    import SimControl from "$lib/components/SimControl.svelte";

    let simulationSpeed = "1x";
    function changeSpeed(event) {
        simulationSpeed = event.target.value;
    }
    let isPlaying = true;
    // Simulation controls
    function toggleSimulation() {
        isPlaying = !isPlaying;
    }

    export let activeView;
    // Event log data
    const eventLog = [
        { time: "09:45", timeAgo: "just now", message: "Sarah completed the login page feature", type: "green" },
        { time: "09:42", timeAgo: "3m ago", message: "John is experiencing high stress levels", type: "amber" },
        { time: "09:30", timeAgo: "15m ago", message: "New project proposal received", type: "blue" },
        { time: "09:15", timeAgo: "30m ago", message: "Server outage affecting development", type: "red" }
    ];
</script>

<div class="w-64 bg-slate-800 text-slate-200 flex flex-col border-r border-slate-700">
    <div class="p-4 border-b border-slate-700 flex items-center justify-between">
        <h1 class="font-bold text-lg tracking-tight">DevCorp Sim</h1>
        <div class="flex space-x-1">
            <button class="p-1 hover:bg-slate-700 rounded">
                <Settings size={16} />
            </button>
            <button class="p-1 hover:bg-slate-700 rounded">
                <HelpCircle size={16} />
            </button>
        </div>
    </div>

    <div class="flex-1 overflow-auto">
        <div class="p-2">
            <div class="text-xs uppercase text-slate-500 font-bold px-3 py-2">Navigation</div>
            <div class="space-y-1">
                <button
                        class="flex items-center w-full px-3 py-2 rounded {activeView === 'Dashboard' ? 'bg-slate-700 text-slate-100' : 'hover:bg-slate-700 text-slate-300'}"
                        on:click={() =>{

                        } }
                >
                    <LayoutDashboard size={16} class="mr-2" />
                    <span>Dashboard</span>
                </button>
                <button
                        class="flex items-center w-full px-3 py-2 rounded {$activeTab?.id === 'overview' ? 'bg-slate-700 text-slate-100' : 'hover:bg-slate-700 text-slate-300'}"
                        on:click={() => tabState.addSystemTab("overview")}
                >
                    <Users size={16} class="mr-2" />
                    <span>Personnel</span>
                </button>
                <button
                        class="flex items-center w-full px-3 py-2 rounded {$activeTab?.id === 'projects' ? 'bg-slate-700 text-slate-100' : 'hover:bg-slate-700 text-slate-300'}"
                        on:click={() => tabState.addSystemTab('projects')}
                >
                    <Code size={16} class="mr-2" />
                    <span>Projects</span>
                </button>
                <button
                        class="flex items-center w-full px-3 py-2 rounded {activeView === 'Finances' ? 'bg-slate-700 text-slate-100' : 'hover:bg-slate-700 text-slate-300'}"
                        on:click={() => goto("/company")}
                >
                    <CreditCard size={16} class="mr-2" />
                    <span>Finances</span>
                </button>
                <button
                        class="flex items-center w-full px-3 py-2 rounded {activeView === 'Analytics' ? 'bg-slate-700 text-slate-100' : 'hover:bg-slate-700 text-slate-300'}"

                >
                    <BarChart2 size={16} class="mr-2" />
                    <span>Analytics</span>
                </button>
            </div>
        </div>

        <!-- Event Log -->
        <div class="p-2 mt-4">
            <div class="flex items-center justify-between px-3 py-2">
                <div class="text-xs uppercase text-slate-500 font-bold">Event Log</div>
                <div class="flex items-center space-x-1">
                    <span class="text-xs text-slate-400">12 new</span>
                    <div class="w-2 h-2 bg-amber-400 rounded-full"></div>
                </div>
            </div>
            <div class="bg-slate-900 rounded border border-slate-700 max-h-48 overflow-y-auto">
                {#each eventLog as event}
                    <div class="p-2 border-l-2 border-{event.type}-500 hover:bg-slate-800">
                        <div class="flex justify-between items-start">
                            <span class="text-xs text-{event.type}-400">{event.time}</span>
                            <span class="text-xs text-slate-500">{event.timeAgo}</span>
                        </div>
                        <p class="text-xs mt-1">{event.message}</p>
                    </div>
                {/each}
            </div>
        </div>
    </div>

    <!-- Simulation Controls -->
    <SimControl />
</div>