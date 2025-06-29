<script lang="ts">
	import { onMount } from 'svelte';
	// const timesOfDay = Array.from({ length: 96 }, (_, i) => i + 1); // This will be replaced by hourly logic

	type CalendarEvent = {
		title: string;
		startTime: number; // 1-96, where 1 is 00:00-00:15
		duration: number; // in 15-minute blocks
		type: string; // e.g., 'meeting', 'call', 'break'
	};

	type SlotState =
		| { status: 'empty' }
		| { status: 'start'; event: CalendarEvent }
		| { status: 'continuation' };

	const sampleEvents: CalendarEvent[] = [
		// 9:00 AM Block
		{ title: 'Daily Standup', startTime: 37, duration: 1, type: 'green' }, // 9:00 - 9:15
		{ title: 'Quick Sync', startTime: 39, duration: 1, type: 'blue' }, // 9:30 - 9:45
		// 10:00 AM Block
		{ title: 'Code Review Session', startTime: 41, duration: 3, type: 'purple' }, // 10:00 - 10:45
		{ title: 'Project Sync', startTime: 44, duration: 2, type: 'yellow' }, // 10:45 - 11:15 (Cross-hour)
		// 12:00 PM Block
		{ title: 'Lunch Break', startTime: 49, duration: 4, type: 'gray' }, // 12:00 - 13:00
		// Multi-hour event
		{ title: 'Deep Work Session', startTime: 53, duration: 8, type: 'indigo' }, // 13:00 - 15:00
		// 4:00 PM Block
		{ title: 'Client Call', startTime: 65, duration: 1, type: 'orange' }, // 16:00 - 16:15
		{ title: 'Follow-up Notes', startTime: 66, duration: 1, type: 'red' }, // 16:15 - 16:30
		{ title: 'Email Review', startTime: 67, duration: 1, type: 'indigo' }, // 16:30 - 16:45
		{ title: 'Tomorrow Planning', startTime: 68, duration: 1, type: 'pink' } // 16:45 - 17:00
	];

	const getEventTypeStyles = (type: string) => {
		const styles: { [key: string]: { container: string; text: string } } = {
			green: { container: 'bg-green-100 border-green-500', text: 'text-green-700' },
			blue: { container: 'bg-blue-100 border-blue-500', text: 'text-blue-700' },
			purple: { container: 'bg-purple-100 border-purple-500', text: 'text-purple-700' },
			gray: { container: 'bg-gray-100 border-gray-400', text: 'text-gray-600' },
			yellow: { container: 'bg-yellow-100 border-yellow-500', text: 'text-yellow-700' },
			orange: { container: 'bg-orange-100 border-orange-500', text: 'text-orange-700' },
			red: { container: 'bg-red-100 border-red-500', text: 'text-red-700' },
			indigo: { container: 'bg-indigo-100 border-indigo-500', text: 'text-indigo-700' },
			pink: { container: 'bg-pink-100 border-pink-500', text: 'text-pink-700' }
		};

		return styles[type] || styles['gray'];
	};

	const formatTime = (timeSlot: number): string => {
		const index = timeSlot - 1;
		const totalMinutes = index * 15;
		const hour = Math.floor(totalMinutes / 60);
		const minute = totalMinutes % 60;

		const paddedHour = String(hour).padStart(2, '0');
		const paddedMinute = String(minute).padStart(2, '0');

		return `${paddedHour}:${paddedMinute}`;
	};

	let currentTime = 1; // 1-96, represents the current 15-minute block

	onMount(() => {
		const timer = setInterval(() => {
			currentTime = (currentTime % 96) + 1;
		}, 500); // Update every second for testing

		return () => {
			clearInterval(timer);
		};
	});

	/**
	 * Calculates the dynamic height for an event block to span multiple time slots.
	 * It accounts for the height of each slot (h-3 -> 0.75rem) and the space between them (space-y-1 -> 0.25rem).
	 * @param duration - The number of 15-minute slots the event occupies.
	 * @returns A string for the inline style property, e.g., "height: 2.75rem;".
	 */
	const calculateEventHeight = (duration: number): string => {
		const slotHeightRem = 0.75; // from h-3
		const gapHeightRem = 0.25; // from space-y-1
		const totalHeight = duration * slotHeightRem + (duration - 1) * gapHeightRem;
		return `height: ${totalHeight}rem;`;
	};

	export let events: CalendarEvent[] = sampleEvents;

	// Reactive declaration: This block re-runs whenever `events` changes.
	// It creates a 96-slot representation of the day.
	let slots: SlotState[] = [];
	$: {
		// 1. Initialize an array of 96 empty slots.
		const newSlots: SlotState[] = Array(96)
			.fill(null)
			.map(() => ({ status: 'empty' }));

		// 2. Populate the slots array based on the events.
		for (const event of events) {
			if (event.startTime >= 1 && event.startTime <= 96) {
				// Mark the start of the event.
				newSlots[event.startTime - 1] = { status: 'start', event: event };

				// 3. Mark subsequent slots as 'continuation'.
				for (let i = 1; i < event.duration; i++) {
					const continuationIndex = event.startTime - 1 + i;
					if (continuationIndex < 96) {
						newSlots[continuationIndex] = { status: 'continuation' };
					}
				}
			}
		}
		slots = newSlots;
	}
</script>
 <!-- <div class="h-3 bg-green-100 border-l-2 border-green-500 rounded-r flex items-center px-1">
                            <span class="text-xs text-green-700 font-medium truncate">Daily Standup</span>
                        </div> -->

<div class="bg-white rounded-lg shadow-sm border border-gray-200 font-sans">
    <!-- Panel Header -->
    <div class="p-4 border-b border-gray-200">
        <div class="flex items-center justify-between">
            <div>
                <h3 class="text-lg font-semibold text-gray-800">Today's Schedule</h3>
                <p class="text-sm text-gray-500">Tuesday, January 9, 2024</p>
            </div>
            <div class="flex items-center space-x-2">
                <button class="p-1 hover:bg-gray-100 rounded text-gray-600">
                    <span class="text-sm">←</span>
                </button>
                <button class="p-1 hover:bg-gray-100 rounded text-gray-600">
                    <span class="text-sm">→</span>
                </button>
            </div>
        </div>
    </div>

    <!-- Time Slots with 15-minute support -->
    <div class="max-h-80 overflow-y-auto">
	{#each Array(24) as _, hour}
		{@const startSlotOfHour = hour * 4 + 1}
		{@const isCurrentHour = currentTime >= startSlotOfHour && currentTime < startSlotOfHour + 4}

		<div class="flex border-b border-gray-100" class:bg-blue-50={isCurrentHour}>
			<!-- Time Label -->
			<div
				class="w-16 p-2 text-xs flex items-start border-r border-gray-100"
				class:text-blue-600={isCurrentHour}
				class:font-medium={isCurrentHour}
				class:text-gray-400={!isCurrentHour}
			>
				{formatTime(startSlotOfHour)}
			</div>

			<!-- Slots Container -->
			<div class="flex-1 p-1 relative">
				<div class="space-y-1">
					{#each Array(4) as __, i}
						{@const timeSlot = startSlotOfHour + i}
						{@const slotState = slots[timeSlot - 1]}

						{#if slotState.status === 'start'}
							{@const event = slotState.event}
							{@const styles = getEventTypeStyles(event.type)}
							<!-- Event Block -->
							<div
								class="absolute left-1 right-1 border-l-2 rounded-r flex items-center {styles.container}"
								style="top: {i * 1}rem; {calculateEventHeight(event.duration)}"
							>
								<div class="flex items-center justify-between w-full px-2">
									<div class="flex-1 min-w-0">
										<div class="text-xs font-medium truncate {styles.text}">{event.title}</div>
										{#if event.duration > 2}
											<div class="text-xs text-gray-500">{event.duration * 15}min</div>
										{/if}
									</div>
									{#if currentTime >= event.startTime && currentTime < event.startTime + event.duration}
										<div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
									{/if}
								</div>
							</div>
							<!-- Placeholder for layout -->
							<div class="h-3"></div>
						{:else if slotState.status === 'continuation'}
							<!-- Placeholder for layout -->
							<div class="h-3"></div>
						{:else}
							<!-- Empty Slot -->
							<div class="h-3 bg-gray-50 rounded flex items-center justify-end">
								{#if timeSlot === currentTime}
									<div class="w-2 h-2 bg-green-500 rounded-full animate-pulse mr-1"></div>
								{/if}
							</div>
						{/if}
					{/each}
				</div>
			</div>
		</div>
	{/each}
</div>


    <!-- Panel Footer -->
    <div class="p-3 border-t border-gray-200 bg-gray-50">
        <div class="flex items-center justify-between text-xs">
            <div class="flex items-center space-x-3">
                <div class="flex items-center space-x-1">
                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                    <span class="text-gray-600">6 meetings</span>
                </div>
                <div class="flex items-center space-x-1">
                    <div class="w-2 h-2 bg-orange-500 rounded-full"></div>
                    <span class="text-gray-600">2 calls</span>
                </div>
            </div>
            <button class="text-blue-600 hover:text-blue-700 font-medium">
                + Add Event
            </button>
        </div>
    </div>
</div>