<script lang="ts">
    import { ChevronLeft, ChevronRight, Plus } from 'lucide-svelte';

    // Props
    export let currentYear: number = new Date().getFullYear();
    export let currentWeek: number = 1; // 1-based week number

    // Event callback props
    export let onWeekChange: (detail: { year: number, week: number }) => void = () => {};
    export let onScheduleEvent: (detail: { year: number, week: number, day: number, eventId?: string, startTime?: string }) => void = () => {};

    interface CalendarEvent {
        id: string;
        title: string;
        startTime: string; // Format: "HH:mm", e.g., "09:00"
        endTime: string;   // Format: "HH:mm", e.g., "10:30"
        description?: string;
        color?: string; // e.g., 'bg-blue-100 text-blue-700 border-blue-300'
    }

    // --- Configuration for the day timeline ---
    const DISPLAY_START_HOUR = 7; // Hour the timeline view starts (e.g., 7 AM)
    const DISPLAY_END_HOUR = 19;   // Hour the timeline view ends (e.g., 7 PM, exclusive for last slot)
    const HOUR_HEIGHT_PX = 60;    // Visual height of one hour in pixels
    const DAY_HEADER_HEIGHT_PX = 48;
    // ---

    const TOTAL_HOURS_DISPLAYED = DISPLAY_END_HOUR - DISPLAY_START_HOUR;
    const DAYS_IN_WEEK = 7;

    const dayLabels = Array.from({ length: DAYS_IN_WEEK }, (_, i) => `Day ${i + 1}`);
    const timeGutterLabels = Array.from({ length: TOTAL_HOURS_DISPLAYED }, (_, i) => {
        const hour = DISPLAY_START_HOUR + i;
        return `${String(hour).padStart(2, '0')}:00`;
    });

    // Helper to convert "HH:mm" to total minutes from midnight
    function timeToMinutes(timeStr: string): number {
        const [hours, minutes] = timeStr.split(':').map(Number);
        return hours * 60 + minutes;
    }

    // Generates style for positioning and sizing an event
    function getEventStyle(event: CalendarEvent): string {
        const eventStartMinutes = timeToMinutes(event.startTime);
        const eventEndMinutes = timeToMinutes(event.endTime);

        const viewStartTotalMinutes = DISPLAY_START_HOUR * 60;
        const viewEndTotalMinutes = DISPLAY_END_HOUR * 60;

        // Clip event times to the visible range
        const effectiveStartMinutes = Math.max(eventStartMinutes, viewStartTotalMinutes);
        const effectiveEndMinutes = Math.min(eventEndMinutes, viewEndTotalMinutes);

        if (effectiveStartMinutes >= effectiveEndMinutes) {
            return 'display: none;'; // Event is outside the view or has no duration in view
        }

        const topOffsetMinutes = effectiveStartMinutes - viewStartTotalMinutes;
        const durationMinutesInView = effectiveEndMinutes - effectiveStartMinutes;

        const topPx = (topOffsetMinutes / 60) * HOUR_HEIGHT_PX;
        const heightPx = (durationMinutesInView / 60) * HOUR_HEIGHT_PX;

        return `top: ${topPx}px; height: ${heightPx}px;`;
    }

    // Reactive calculation for days and their events
    $: daysToDisplay = Array.from({ length: DAYS_IN_WEEK }, (_, i) => {
        const dayNumber = i + 1;
        // --- Populate with actual events for currentYear, currentWeek, dayNumber ---
        // This is placeholder data. You'll need to fetch/filter your events.
        let events: CalendarEvent[] = [];
        if (currentWeek === 10) { // Example: only show events for week 10
            if (dayNumber === 1) {
                events = [
                    { id: 'evt1', title: 'Team Standup', startTime: '09:00', endTime: '09:30', color: 'bg-sky-100 text-sky-700 border-sky-300' },
                    { id: 'evt2', title: 'Project Alpha Planning', startTime: '10:00', endTime: '11:30', description: 'Detailed planning session', color: 'bg-amber-100 text-amber-700 border-amber-300' },
                    { id: 'evt5', title: 'Lunch', startTime: '12:00', endTime: '13:00', color: 'bg-slate-100 text-slate-600 border-slate-300'},
                    { id: 'evt3', title: 'Client Demo Prep', startTime: '14:00', endTime: '16:00', color: 'bg-indigo-100 text-indigo-700 border-indigo-300' },
                ];
            } else if (dayNumber === 3) {
                events = [
                    { id: 'evt4', title: 'Code Review Session', startTime: '15:00', endTime: '16:30', color: 'bg-emerald-100 text-emerald-700 border-emerald-300' },
                    { id: 'evt6', title: 'Quick Sync', startTime: '08:30', endTime: '09:00', color: 'bg-rose-100 text-rose-700 border-rose-300'},

                ];
            }
        }
        // Filter events to only those that might appear in the current view window
        // (More sophisticated filtering might be needed if events can span across view boundaries significantly)
        events = events.filter(e => timeToMinutes(e.endTime) > DISPLAY_START_HOUR * 60 && timeToMinutes(e.startTime) < DISPLAY_END_HOUR * 60);

        return {
            year: currentYear,
            week: currentWeek,
            day: dayNumber,
            events: events
        };
    });

    // Note: getWeekNumberForYear is complex for true ISO 8601.
    // For production, consider a date library like date-fns for robust week calculations.
    function getWeekNumberForYear(year: number): number {
        const d = new Date(Date.UTC(year, 0, 1));
        const dayNum = d.getUTCDay() || 7;
        d.setUTCDate(d.getUTCDate() + 4 - dayNum);
        const yearStart = new Date(Date.UTC(d.getUTCFullYear(), 0, 1));
        return Math.ceil((((d.valueOf() - yearStart.valueOf()) / 86400000) + 1) / 7);
    }

    function goToPreviousWeek() {
        if (currentWeek > 1) {
            currentWeek--;
        } else {
            currentYear--;
            currentWeek = getWeekNumberForYear(currentYear);
        }
        if (onWeekChange) onWeekChange({ year: currentYear, week: currentWeek });
    }

    function goToNextWeek() {
        const maxWeeksThisYear = getWeekNumberForYear(currentYear);
        if (currentWeek < maxWeeksThisYear) {
            currentWeek++;
        } else {
            currentYear++;
            currentWeek = 1;
        }
        if (onWeekChange) onWeekChange({ year: currentYear, week: currentWeek });
    }

    function handleEventClick(year: number, week: number, day: number, eventId?: string) {
        if (onScheduleEvent) onScheduleEvent({ year, week, day, eventId });
    }

    function handleTimeSlotClick(year: number, week: number, day: number, hour: number) {
        const startTime = `${String(hour).padStart(2, '0')}:00`;
        if (onScheduleEvent) onScheduleEvent({ year, week, day, startTime });
    }

</script>

<style>
    :root {
        --hour-height: 60px; /* Default, overridden by const HOUR_HEIGHT_PX in JS context */
        --day-header-height: 48px; /* Default, overridden by const DAY_HEADER_HEIGHT_PX */
    }
    .time-gutter {
        padding-top: var(--day-header-height); /* Align with start of day content */
    }
    .day-header {
        height: var(--day-header-height);
    }
    .events-area {
        min-height: calc(v-bind(TOTAL_HOURS_DISPLAYED) * v-bind(HOUR_HEIGHT_PX) * 1px); /* Svelte way to use JS const in CSS */
    }
    .time-gutter-slot {
        height: v-bind(HOUR_HEIGHT_PX) * 1px;
    }
    .background-hour-line {
        height: v-bind(HOUR_HEIGHT_PX) * 1px;
    }
</style>

<div class="p-4 md:p-6 border rounded-lg shadow bg-white mt-4 font-mono text-sm flex flex-col"
     style="--hour-height: {HOUR_HEIGHT_PX}px; --day-header-height: {DAY_HEADER_HEIGHT_PX}px;">
    <!-- Header: Week Navigation and Actions -->
    <div class="flex items-center justify-between mb-4">
        <button on:click={goToPreviousWeek} class="p-2 rounded hover:bg-slate-200 text-slate-600" aria-label="Previous week">
            <ChevronLeft size={20} />
        </button>
        <h2 class="text-xl font-bold text-slate-700">
            Week {currentWeek}, Year {currentYear}
        </h2>
        <button on:click={goToNextWeek} class="p-2 rounded hover:bg-slate-200 text-slate-600" aria-label="Next week">
            <ChevronRight size={20} />
        </button>
    </div>
    <div class="flex items-center justify-end mb-4">
        <button
                on:click={() => handleEventClick(currentYear, currentWeek, 0)}
                class="flex items-center px-4 py-2 bg-blue-600 text-white text-xs sm:text-sm rounded hover:bg-blue-700 transition-colors"
        >
            <Plus size={18} class="mr-2" />
            Schedule Event
        </button>
    </div>

    <!-- Calendar Main Area -->
    <div class="calendar-body flex flex-grow overflow-x-auto">
        <!-- Time Gutter (fixed width) -->
        <div class="time-gutter w-20 flex-shrink-0 border-r border-slate-200">
            {#each timeGutterLabels as label, i (label)}
                <div class="time-gutter-slot text-right pr-2 text-xs text-slate-500 relative flex items-end justify-end">
                    <span class="transform -translate-y-1/2">{label}</span>
                </div>
            {/each}
        </div>

        <!-- Days Grid (takes remaining width) -->
        <div class="days-grid flex-grow grid grid-cols-7 min-w-[calc(7*150px)]" >
                {#each daysToDisplay as day (day.day)}
            <div class="day-column-container border-r border-slate-200 flex flex-col">
            <!-- Day Header -->
            <div class="day-header p-2 border-b border-slate-200 flex justify-between items-center bg-slate-50 sticky top-0 z-10">
            <span class="font-semibold text-slate-700 text-xs sm:text-sm">{dayLabels[day.day - 1]}</span>
            <span class="text-xs text-slate-500">W{day.week} D{day.day}</span>
            </div>

            <!-- Events Area for the Day -->
            <div class="events-area relative flex-grow"
            on:click={(e) => {
            // Basic click on empty slot to get time
            if ((e.target as HTMLElement).classList.contains('events-area') || (e.target as HTMLElement).classList.contains('background-hour-line')) {
            const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
            const y = e.clientY - rect.top;
            const hourClicked = Math.floor(y / HOUR_HEIGHT_PX) + DISPLAY_START_HOUR;
            handleTimeSlotClick(day.year, day.week, day.day, hourClicked);
        }
        }}
            >
            <!-- Background Hour Lines for this day column -->
            {#each Array.from({ length: TOTAL_HOURS_DISPLAYED }, (_, i) => i) as hourIndex (hourIndex)}
            <div class="background-hour-line absolute w-full border-b border-dashed border-slate-100 pointer-events-none"
            style="top: {hourIndex * HOUR_HEIGHT_PX}px; z-index: 0;">
            </div>
            {/each}

            <!-- Events -->
                {#each day.events as event (event.id)}
            <div class="event-item absolute left-1 right-1 p-1.5 rounded border text-[0.7rem] leading-tight cursor-pointer {event.color || 'bg-blue-100 text-blue-700 border-blue-300'} shadow-sm hover:shadow-md hover:z-20"
            style="{getEventStyle(event)}; z-index: 5;"
            on:click|stopPropagation={() => handleEventClick(day.year, day.week, day.day, event.id)}
            title={event.description || `${event.startTime}-${event.endTime}: ${event.title}`}>
            <div class="font-semibold">{event.startTime}-{event.endTime}</div>
            <div>{event.title}</div>
            </div>
                {/each}
            </div>
            </div>
                {/each}
            </div>
            </div>
            </div>