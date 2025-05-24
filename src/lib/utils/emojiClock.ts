export function getTimeOfDayEmoji(quarter_tick: number): { emoji: string, isOfficeHours: boolean } {
    const hour = Math.floor(quarter_tick / 4);
    const minute = (quarter_tick % 4) * 15;

    // Convert to 12-hour clock for emoji
    let emoji = "🕛"; // fallback
    const clockHour = hour % 12 === 0 ? 12 : hour % 12;
    const halfHour = minute >= 30;

    const emojiMap: Record<number, string> = {
        1: "🕐", 2: "🕑", 3: "🕒", 4: "🕓", 5: "🕔", 6: "🕕",
        7: "🕖", 8: "🕗", 9: "🕘", 10: "🕙", 11: "🕚", 12: "🕛",
    };

    const halfEmojiMap: Record<number, string> = {
        1: "🕜", 2: "🕝", 3: "🕞", 4: "🕟", 5: "🕠", 6: "🕡",
        7: "🕢", 8: "🕣", 9: "🕤", 10: "🕥", 11: "🕦", 12: "🕧",
    };

    emoji = halfHour ? halfEmojiMap[clockHour] : emojiMap[clockHour];

    // Office hours: 9:00–16:59 (9–16)
    const isOfficeHours = hour >= 9 && hour < 17;

    return { emoji, isOfficeHours };
}

export function formatQuarterTickToTimeString(quarter_tick: number): string {
    const totalMinutes = quarter_tick * 15;
    const hours24 = Math.floor(totalMinutes / 60);
    const minutes = totalMinutes % 60;

    const isPM = hours24 >= 12;
    const hours12 = hours24 % 12 === 0 ? 12 : hours24 % 12;
    const minuteStr = minutes.toString().padStart(2, '0');
    const period = isPM ? 'PM' : 'AM';

    return `${hours12.toString().padStart(2,0)}:${minuteStr} ${period}`;
}
