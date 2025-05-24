<script>
import {tabState} from "$lib/stores/TabStore.js";
import {Plus, X} from "lucide-svelte";
</script>

<div class="bg-slate-700 border-b border-slate-300 flex overflow-x-auto">
    {#each $tabState.tabs as tab}
        <div
                class="px-4 py-2 {tab.isActive ? 'bg-slate-900 text-zinc-400' : ''} border-r border-slate-300 font-medium flex items-center whitespace-nowrap hover:cursor-default"
                on:click={() =>  tabState.setActiveTab(tab)}
                on:mousedown|stopPropagation={(e) => {
    if (e.button === 1) {
      tabState.removeTab(tab)
    }
  }}
        >
            <span>{tab.title}</span>
            <button
                    class="ml-2 text-slate-400 hover:text-slate-600"
                    on:click|stopPropagation={() => tabState.removeTab(tab)}
            >
                <X size={14} class="hover:cursor-pointer" />
            </button>
        </div>
    {/each}
    <button class="px-3 py-2 text-slate-600 hover:bg-slate-300" on:click={()=>tabState.addSystemTab("overview")}>
        <Plus size={16} />
    </button>
</div>