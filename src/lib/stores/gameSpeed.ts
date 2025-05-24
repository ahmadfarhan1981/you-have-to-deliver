import { writable } from 'svelte/store';

export interface TickSnapshot {
    tick: number;
    year: number;
    week: number;
    day: number;
    quarter_tick: number;
}
export const  gameSpeedUpdateEventName = "game_speed_snapshot";

export interface GameSpeedSnapshot {
    tick: TickSnapshot;
    game_speed: number;
}

export const gameSpeed = writable<GameSpeedSnapshot>({
    tick: {
        tick: 0,
        year: 0,
        week: 0,
        day: 0,
        quarter_tick: 0,
    },
    game_speed: 0,
});


export function getGameSpeedText(speed: number): string {
    switch (speed) {
        case 2:
            return "Slow";
        case 3:
            return "Normal";
        case 4:
            return "Fast";
        case 5:
            return "Max";
        default:
            return "Stopped";
    }
}