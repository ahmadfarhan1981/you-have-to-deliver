export type SimDate = {
    year: number;         // starts at 1
    week: number;         // 1–52
    day: number;          // 1–7
    quarter_tick: number; // 1–96
};

// --- Constants (consistent with your Rust backend and updated calendar) ---
const TICKS_PER_DAY = 96;
const DAYS_PER_WEEK = 7;
const WEEKS_PER_YEAR = 52;
const DAYS_PER_YEAR = DAYS_PER_WEEK * WEEKS_PER_YEAR;
const TICKS_PER_YEAR = DAYS_PER_YEAR * TICKS_PER_DAY;
// -------------------------------------------------------------------------

export function simDateToString(date: SimDate): string {
    return `Day ${date.day}, Week ${date.week}, Year ${date.year}`;
}

// Helper for padding numbers (used by simDateToVerboseString)
function padded(n: number): string {
    return n.toString().padStart(2, "0");
}

export function simDateToVerboseString(date: SimDate): string {
    const minutes = (date.quarter_tick - 1) * 15;
    const hour = Math.floor(minutes / 60);
    const minute = minutes % 60;

    return `Day ${date.day}, Week ${date.week}, Year ${date.year} — ${padded(hour)}:${padded(minute)}`;
}


export function simDateFromTick(tick: number): SimDate {
    const year = Math.floor(tick / TICKS_PER_YEAR) + 1;
    const remaining = tick % TICKS_PER_YEAR;

    const dayIndex = Math.floor(remaining / TICKS_PER_DAY);
    const quarter_tick = (remaining % TICKS_PER_DAY) + 1;

    const week = Math.floor(dayIndex / DAYS_PER_WEEK) + 1;
    const day = (dayIndex % DAYS_PER_WEEK) + 1;

    return {
        year,
        week,
        day,
        quarter_tick
    };
}

export function tickFromSimDate(date: SimDate): number {
    const yearPart = (date.year - 1) * TICKS_PER_YEAR;
    const weekPart = (date.week - 1) * DAYS_PER_WEEK * TICKS_PER_DAY;
    const dayPart = (date.day - 1) * TICKS_PER_DAY;
    const tickPart = date.quarter_tick - 1;

    return yearPart + weekPart + dayPart + tickPart;
}

// Helper for pluralization (e.g., "1 day" vs "2 days")
function pluralize(count: number, unit: string): string {
    return `${count} ${unit}${count === 1 ? '' : 's'}`;
}

/**
 * Calculates a human-readable, general relative time string between two SimDates.
 * Rounds to the nearest day, week, or year.
 * Examples: "Recently", "3 days ago", "In 4 weeks", "5 years ago".
 * @param from The reference SimDate.
 * @param to The target SimDate.
 * @returns A relative time string.
 */
export function simDateToRelativeString(from: SimDate, to: SimDate): string {
    const fromTick = tickFromSimDate(from);
    const toTick = tickFromSimDate(to);
    const diff = toTick - fromTick;
    const absDiffTicks = Math.abs(diff);

    if (absDiffTicks === 0) {
        return "Right now"; // Exact match
    }

    const suffix = diff > 0 ? "In " : " ago"; // Direction indicator

    // Less than one full day
    if (absDiffTicks < TICKS_PER_DAY) {
        return "Recently";
    }

    // Days (rounded)
    const days = Math.round(absDiffTicks / TICKS_PER_DAY);
    if (days < DAYS_PER_WEEK) { // Less than a week
        return `${pluralize(days, "day")}${suffix}`;
    }

    // Weeks (rounded)
    const weeks = Math.round(absDiffTicks / (DAYS_PER_WEEK * TICKS_PER_DAY));
    if (weeks < WEEKS_PER_YEAR) { // Less than a year
        return `${pluralize(weeks, "week")}${suffix}`;
    }

    // Years (rounded)
    const years = Math.round(absDiffTicks / TICKS_PER_YEAR);
    return `${pluralize(years, "year")}${suffix}`;
}

/**
 * Calculates a human-readable, precise relative time string between two SimDates.
 * Shows hours/minutes if less than a day, otherwise whole days.
 * Examples: "15 minutes ago", "In 6 hours", "1 day ago", "In 3 days".
 * @param from The reference SimDate.
 * @param to The target SimDate.
 * @returns A precise relative time string.
 */
export function simDateToPreciseRelativeString(from: SimDate, to: SimDate): string {
    const fromTick = tickFromSimDate(from);
    const toTick = tickFromSimDate(to);
    const diff = toTick - fromTick;
    const absDiffTicks = Math.abs(diff);

    if (absDiffTicks === 0) {
        return "Right now"; // Exact match
    }

    const suffix = diff > 0 ? "In " : " ago"; // Direction indicator

    // If less than one day, calculate in hours and minutes
    if (absDiffTicks < TICKS_PER_DAY) {
        const totalMinutes = absDiffTicks * 15;
        const hours = Math.floor(totalMinutes / 60);
        const minutes = totalMinutes % 60;

        let timeString = "";
        if (hours > 0) {
            timeString += pluralize(hours, "hour");
        }
        if (minutes > 0) {
            // Add a space only if there are also hours
            if (hours > 0) {
                timeString += " ";
            }
            timeString += pluralize(minutes, "minute");
        }
        // This 'if (!timeString)' check handles potential 0-minute/0-hour scenarios,
        // although 'Right now' should cover absDiffTicks === 0
        if (!timeString) {
            // This case should ideally not be hit if absDiffTicks > 0
            return "Just now"; // Fallback for very small non-zero diffs if they somehow produce empty string
        }
        return `${timeString}${suffix}`;

    } else {
        // One day or more, show in whole days
        const days = Math.floor(absDiffTicks / TICKS_PER_DAY);
        return `${pluralize(days, "day")}${suffix}`;
    }
}