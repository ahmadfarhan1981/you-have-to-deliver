<script lang="ts">
    // import { createEventDispatcher } from 'svelte'; // REMOVE THIS
    import { ChevronLeft, ChevronRight, Plus } from 'lucide-svelte';

    // Props
    export let currentYear: number = new Date().getFullYear(); // Or your game's starting year
    export let currentMonthView: number = 1; // 1-based: 1st 4-week block, 2nd, etc.

    // Event callback prop for Svelte 5
    export let onMonthChange: (detail: { year: number, monthView: number }) => void = () => {};

    // Constants
    const WEEKS_IN_MONTH_VIEW = 4;
    const DAYS_IN_WEEK = 7;
    const MAX_MONTH_VIEWS_PER_YEAR = 13; // 52 weeks / 4 weeks_per_month_view

    // Reactive calculations for the weeks to display
    $: startWeekOfView = (currentMonthView - 1) * WEEKS_IN_MONTH_VIEW + 1;
    $: weeksToDisplay = Array.from({ length: WEEKS_IN_MONTH_VIEW }, (_, i) => startWeekOfView + i);

    // For display purposes, "Month X" can refer to the Xth 4-week block.
    $: displayMonthName = `Month ${currentMonthView}`;

    // Grid data generation
    $: calendarGrid = weeksToDisplay.map(weekNumber => {
        return Array.from({ length: DAYS_IN_WEEK }, (_, dayIndex) => {
            const dayNumber = dayIndex + 1; // Day 1 to 7
            // Here you would fetch actual events for this SimDate
            // const simDate = { year: currentYear, week: weekNumber, day: dayNumber, quarter_tick: 1 };
            // const events = getEventsForDate(simDate);
            return {
                year: currentYear,
                week: weekNumber,
                day: dayNumber,
                isCurrentMonth: true, // All days in this view are "current"
                events: [] // Placeholder for actual events
            };
        });
    });

    function goToPreviousMonth() {
        if (currentMonthView > 1) {
            currentMonthView--;
        } else if (currentYear > 1) { // Assuming year 1 is the earliest
            currentYear--;
            currentMonthView = MAX_MONTH_VIEWS_PER_YEAR; // Go to the last 4-week block of prev year
        }
        // Dispatch by calling the prop
        if (onMonthChange) {
            onMonthChange({ year: currentYear, monthView: currentMonthView });
        }
    }

    function goToNextMonth() {
        if (currentMonthView < MAX_MONTH_VIEWS_PER_YEAR) {
            currentMonthView++;
        } else {
            currentYear++;
            currentMonthView = 1; // Go to the first 4-week block of next year
        }
        // Dispatch by calling the prop
        if (onMonthChange) {
            onMonthChange({ year: currentYear, monthView: currentMonthView });
        }
    }

    function openScheduleEventModal(year?: number, week?: number, day?: number) {
        // In a real app, this would open a modal and pass the date
        console.log('Open schedule event modal for:', { year, week, day });
        alert(`Schedule event for Year: ${year}, Week: ${week}, Day: ${day}`);
        // You would typically use a store or component prop to show a modal
    }

    const dayLabels = Array.from({ length: DAYS_IN_WEEK }, (_, i) => `Day ${i + 1}`);

</script>

<div class="p-4 md:p-6 border rounded-lg shadow bg-white mt-4 font-mono text-sm">
    <!-- Header: Month Navigation and Actions -->
    <div class="flex items-center justify-between mb-4">
        <button
                on:click={goToPreviousMonth}
                class="p-2 rounded hover:bg-slate-200 text-slate-600"
                aria-label="Previous month"
        >
            <ChevronLeft size={20} />
        </button>
        <h2 class="text-lg font-bold text-slate-700">
            {displayMonthName}, Year {currentYear}
        </h2>
        <button
                on:click={goToNextMonth}
                class="p-2 rounded hover:bg-slate-200 text-slate-600"
                aria-label="Next month"
        >
            <ChevronRight size={20} />
        </button>
    </div>

    <div class="flex items-center justify-end mb-4">
        <button
                on:click={() => openScheduleEventModal()}
                class="flex items-center px-3 py-1.5 bg-blue-600 text-white text-xs rounded hover:bg-blue-700 transition-colors"
        >
            <Plus size={16} class="mr-1.5" />
            Schedule Event
        </button>
    </div>

    <!-- Calendar Grid -->
    <div class="grid grid-cols-7 gap-px border border-slate-200 bg-slate-200">
        <!-- Day Labels -->
        {#each dayLabels as label}
            <div class="text-center py-2 px-1 bg-slate-50 text-slate-600 font-semibold text-xs">
                {label}
            </div>
        {/each}

        <!-- Day Cells -->
        {#each calendarGrid as weekDays, weekIndex}
            {#each weekDays as day}
                <div
                        class="relative bg-white p-2 min-h-[80px] md:min-h-[100px] hover:bg-slate-50 transition-colors cursor-pointer"
                        on:click={() => openScheduleEventModal(day.year, day.week, day.day)}
                        role="button"
                        tabindex="0"
                        aria-label={`Day ${day.day} of Week ${day.week}, Year ${day.year}`}
                >
                    <span class="text-xs font-medium text-slate-700">
                        W{day.week}-D{day.day}
                    </span>
                    <!-- Event placeholders -->
                    <div class="mt-1 space-y-0.5">
                        {#each day.events as event (event.id)}
                            <div class="text-xs p-1 bg-blue-100 text-blue-700 rounded truncate">
                                {event.title}
                            </div>
                        {:else}
                            <!-- <p class="text-xs text-slate-400 italic">No events</p> -->
                        {/each}
                    </div>
                </div>
            {/each}
        {/each}
    </div>
</div>

<!--
    Example of how you might use it in Svelte 5:
    <script lang="ts">
        import CalendarPanel from './CalendarPanel.svelte'; // Adjust path

        let currentYear = 2024; // Your game's current year
        let currentMonthView = 1; // Your game's current 4-week block

        function handleMonthChange(detail: { year: number, monthView: number }) {
            // Logic to fetch events for the new month view
            console.log("Month changed to:", detail);
            // Update state if needed, e.g., to trigger data fetching
            currentYear = detail.year;
            currentMonthView = detail.monthView;
        }
    </script>

    <CalendarPanel
        bind:currentYear
        bind:currentMonthView
        onMonthChange={handleMonthChange}
    />
-->