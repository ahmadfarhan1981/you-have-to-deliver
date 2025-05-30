
export type StatKey =
    | 'judgement'
    | 'creativity'
    | 'systems'
    | 'precision'
    | 'focus'
    | 'discipline'
    | 'empathy'
    | 'communication'
    | 'resilience'
    | 'adaptability';

export type StatGroup =
    | 'Cognition'
    | 'Perception'
    | 'Drive'
    | 'Social'
    | 'Defense';

export type StatGroupMetadata = {
    name: StatGroup;
    total: number;
    average: number;
};

export type StatDefinition = {
    label: string;
    key: StatKey;
    class: string;
};

// ✅ Extend it cleanly
export type StatWithValue = StatDefinition & {
    value: number;
};

export type GroupedStatDefinition = {
    group: StatGroup;
    items: StatDefinition[];
};

export type GroupedStatSnapshot = {
    group: StatGroupMetadata;
    items: StatWithValue[];
};

export type StatsSnapshot = {
    judgement: number;
    creativity: number;
    systems: number;
    precision: number;
    focus: number;
    discipline: number;
    empathy: number;
    communication: number;
    resilience: number;
    adaptability: number;
};

export const stats_definition:GroupedStatDefinition[] = [
    {
        group: 'Cognition',
        items: [
            { label: 'Judgement', key: 'judgement', class: 'bg-blue-500' },
            { label: 'Creativity', key: 'creativity', class: 'bg-pink-500' }
        ]
    },
    {
        group: 'Perception',
        items: [
            { label: 'Systems Thinking', key: 'systems', class: 'bg-green-500' },
            { label: 'Precision', key: 'precision', class: 'bg-purple-500' }
        ]
    },
    {
        group: 'Drive',
        items: [
            { label: 'Focus', key: 'focus', class: 'bg-blue-500' },
            { label: 'Discipline', key: 'discipline', class: 'bg-amber-500' }
        ]
    },
    {
        group: 'Social',
        items: [
            { label: 'Empathy', key: 'empathy', class: 'bg-rose-500' },
            { label: 'Communication', key: 'communication', class: 'bg-cyan-500' }
        ]
    },
    {
        group: 'Defense',
        items: [
            { label: 'Resilience', key: 'resilience', class: 'bg-emerald-500' },
            { label: 'Adaptability', key: 'adaptability', class: 'bg-indigo-500' }
        ]
    }
];

export function getGroupedStatSnapshot(
    snapshot: StatsSnapshot
): GroupedStatSnapshot[] {
    return stats_definition.map(group => {
        const items = group.items.map(stat => ({
            ...stat,
            value: snapshot[stat.key] ?? 0
        }));

        const total = items.reduce((sum, stat) => sum + stat.value, 0);
        const average = items.length > 0 ? total / items.length : 0;

        return {
            group: {
                name: group.group,
                total,
                average
            },
            items
        };
    });
}

