import { readable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// Poll every 500ms (matches sim rate)
export const tick = readable(0, (set) => {
    let interval = setInterval(async () => {
        try {
            const value = await invoke<number>('get_tick');
            set(value);
        } catch (e) {
            console.error('Failed to fetch tick:', e);
        }
    }, 50);

    return () => clearInterval(interval); // cleanup
});
