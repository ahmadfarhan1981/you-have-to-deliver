<script lang="ts">
    import { createEventDispatcher, onMount } from 'svelte';

    export let items: { id: string, label: string }[] = [];
    export let activeTabId: string = '';

    const dispatch = createEventDispatcher<{ tabChange: string }>();

    // Initialize activeTabId if not provided and items exist
    onMount(() => {
        if (!activeTabId && items.length > 0) {
            activeTabId = items[0].id;
            // dispatch('tabChange', activeTabId); // Optionally dispatch if you want parent to know initial default
        }
    });

    function selectTab(id: string) {
        if (activeTabId !== id) {
            activeTabId = id; // This updates the bound prop in the parent
            dispatch('tabChange', id);
        }
    }
</script>

<div class="tab-list" role="tablist" aria-label="Content tabs">
    {#each items as item (item.id)}
        <button
                role="tab"
                aria-selected={activeTabId === item.id}
                aria-controls={`panel-${item.id}`}
        id={`tab-button-${item.id}`}
        class="tab-button"
        class:active={activeTabId === item.id}
        on:click={() => selectTab(item.id)}
        >
        {item.label}
        </button>
    {/each}
</div>

<style>
    .tab-list {
        display: flex;
        border-bottom: 2px solid #e5e7eb; /* gray-200 */
        /* margin-bottom: 1rem; Removed, as content spacing is parent's concern */
    }
    .tab-button {
        padding: 0.75rem 1rem;
        font-size: 0.875rem;
        font-weight: 500;
        color: #6b7280;
        background-color: transparent;
        border: none;
        border-bottom: 2px solid transparent;
        margin-bottom: -2px;
        cursor: pointer;
        transition: color 0.2s ease-in-out, border-color 0.2s ease-in-out;
    }
    .tab-button:hover {
        color: #1f2937;
    }
    .tab-button.active {
        color: #3b82f6;
        border-bottom-color: #3b82f6;
        font-weight: 600;
    }
</style>