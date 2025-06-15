<script>
    import {invoke} from "@tauri-apps/api/core";
</script>

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
    <button class="py-3 bg-red-600 hover:bg-red-700 rounded font-bold text-white" on:click={()=>invoke("exit_app")}>
        Exit Game
    </button>
</div>