<script lang="ts">
    import {onMount} from 'svelte';

    import {invoke} from "@tauri-apps/api/core";
    import {goto} from "$app/navigation";
    import {gameSpeed} from "$lib/stores/gameSpeed.js";
    import NewGamePanel from "$lib/components/NewGamePanel.svelte";
    import { Toaster, toast } from "svelte-hot-french-toast";
    import RecentGamePanel from "$lib/components/RecentGamePanel.svelte";
    import MainMenuWelcomPanel from "$lib/components/MainMenuWelcomPanel.svelte";
    import LoadGamePanel from "$lib/components/LoadGamePanel.svelte";
    import SettingsPanel from "$lib/components/SettingsPanel.svelte";
    import TutorialPanel from "$lib/components/TutorialPanel.svelte";
    import ExitGamePanel from "$lib/components/ExitGamePanel.svelte";

    // State management
    let selectedMenu = "New Game";


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


</script>
<div class="grid-bg min-h-screen flex flex-col ">
    <div class="flex-1 flex items-center justify-center ">
        <div class="  w-[90%] mx-auto p-6 ">
            <div class="bg-slate-900 border border-slate-700 rounded-lg shadow-2xl overflow-hidden w-full">
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
                    <MainMenuWelcomPanel />

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
                            <RecentGamePanel />
                        </div>

                        <!-- Right Column - Game Info -->
                        <div class="col-span-3 bg-slate-800 rounded-lg border border-slate-700 p-5 flex flex-col">
                            {#if selectedMenu === "New Game"}
                                <NewGamePanel />
                            {:else if selectedMenu === "Load Game"}
                                <LoadGamePanel />
                            {:else if selectedMenu === "Settings"}
                                <SettingsPanel />

                            {:else if selectedMenu === "Tutorial"}
<TutorialPanel />

                            {:else if selectedMenu === "Exit"}
                               <ExitGamePanel />
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

     .my-custom-toast-style {
         background-color: #1e293b; /* bg-slate-800 */
         border-width: 1px;         /* border */
         border-style: solid;       /* border */
         border-color: #334155;     /* border-slate-700 */
         border-radius: 0.25rem;    /* rounded */
         font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace; /* font-mono */
         font-size: 0.875rem;       /* text-sm (14px) */
         line-height: 1.25rem;      /* text-sm (20px) */
         color: white;              /* To ensure text is visible on dark background */
         padding: 0.75rem 1rem;     /* Optional: Add some padding for better appearance */
     }
</style>