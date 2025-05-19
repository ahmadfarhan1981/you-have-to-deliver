import { readable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// Poll every 50ms (matches sim rate)
export const tick = readable(
    { year: 0, week: 0, day: 0, quarter: 0, tick: 0 },
    (set) => {
        let interval = setInterval(async () => {
            try {
                const [year, week, day, quarter, tick] = await invoke<[number, number, number, number, number]>('get_tick');
                set({ year, week, day, quarter, tick });
            } catch (e) {
                console.error('Failed to fetch tick:', e);
            }
        }, 50);

        return () => clearInterval(interval);
    }
);