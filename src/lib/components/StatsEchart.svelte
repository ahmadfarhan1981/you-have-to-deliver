<script lang="ts">
    import {onMount} from 'svelte';
    import * as echarts from 'echarts';
    import type {ECharts, EChartsOption} from 'echarts';
    import {
        getGroupedStatSnapshot,
        type GroupedStatDefinition,
        stats_definition,
        type StatsSnapshot
    } from "$lib/models/stats";


    export let statsSnapshot: StatsSnapshot;

    interface RadarIndicator {
        name: string;
        max: number;
        category?: string;
    }

    interface ChartDataItem {
        name: string;
        value: number[];
        itemStyle: {
            color: string;
        };
    }

    interface TooltipParams {
        seriesName: string;
        dataIndex: number;
        value: number;
        color: string;
    }

    let chartContainer: HTMLDivElement;
    let chart: ECharts;
    $: isDetailed = true; // Toggle between detailed (10) and consolidated (5) view

    $: groupedStats = getGroupedStatSnapshot(statsSnapshot)

    // Detailed 10-stat data
    const detailedChartData: ChartDataItem[] = [
        // {
        //     name: 'Profile A',
        //     value: [85, 72, 90, 68, 78, 82, 91, 75, 88, 69],
        //     itemStyle: {
        //         color: '#5470c6'
        //     }
        // },
        // {
        //     name: 'Profile B',
        //     value: [75, 85, 70, 92, 65, 88, 73, 95, 71, 86],
        //     itemStyle: {
        //         color: '#91cc75'
        //     }
        // }
    ];

    // Consolidated 5-stat data (averaged/consolidated from the 10 stats)
    const consolidatedChartData: ChartDataItem[] = [
        // {
        //     name: 'Profile A',
        //     value: [78, 79, 80, 83, 78], // Performance, Control, Reliability, Experience, Features
        //     itemStyle: {
        //         color: '#5470c6'
        //     }
        // },
        // {
        //     name: 'Profile B',
        //     value: [80, 81, 76, 84, 78],
        //     itemStyle: {
        //         color: '#91cc75'
        //     }
        // }
    ];




    // let indicators: RadarIndicator[] = stats_definition.map(group => ({
    //     name: group.group,
    //     max: 100
    // }));
    let indicators: RadarIndicator[] = [];
    let detailedRadarIndicators: RadarIndicator[] = [];
    console.log(JSON.stringify(stats_definition))
    stats_definition.forEach( (group:GroupedStatDefinition) => {
        console.log(JSON.stringify(group))
        indicators.push({name: group.group, max: 100});
        group.items.forEach((statDef) => {
            console.log(JSON.stringify(statDef))
            detailedRadarIndicators.push({name: statDef.label, max: 100})

        });
    });




    // Consolidated 5 categories
    const consolidatedRadarIndicators: RadarIndicator[] = indicators;


    // Reactive values that change based on toggle
    $: currentChartData = isDetailed ? detailedChartData : consolidatedChartData;
    $: currentRadarIndicators = isDetailed ? detailedRadarIndicators : consolidatedRadarIndicators;

    // let option :EChartsOption;
    $: option = {
        // title: {
        //     text: `${isDetailed ? 'Detailed' : 'Consolidated'} Attribute Radar Chart`,
        //     left: 'center',
        //     textStyle: {
        //         fontSize: 18,
        //         fontWeight: 'bold'
        //     }
        // },
        tooltip: {
            trigger: 'item',
            formatter: function (params: TooltipParams): string {
                const dataIndex: number = params.dataIndex;
                const indicator: RadarIndicator = currentRadarIndicators[dataIndex];
                const value: number = params.value;

                return `
          <div style="padding: 8px;">
            <strong>${params.seriesName}</strong><br/>
            <span style="color: ${params.color};">●</span>
            ${indicator.name}: <strong>${value}</strong><br/>
            ${indicator.category ? `<small>Category: ${indicator.category}</small>` : ''}
          </div>
        `;
            }
        },
        // legend: {
        //     data: currentChartData.map((item: ChartDataItem) => item.name),
        //     bottom: 10
        // },
        radar: {
            indicator: currentRadarIndicators.map((item: RadarIndicator) => ({
                name: item.name,
                max: item.max
            })),
            radius: '55%',
            axisName: {
                fontSize: 12,
                color: '#666'
            },
            splitLine: {
                lineStyle: {
                    color: '#e0e0e0'
                }
            },
            splitArea: {
                show: true,
                areaStyle: {
                    color: ['rgba(250,250,250,0.1)', 'rgba(200,200,200,0.1)']
                }
            },
            axisLine: {
                lineStyle: {
                    color: '#ccc'
                }
            }
        },
        series: [{
            name: 'Attributes',
            type: 'radar',
            data: currentChartData.map((item: ChartDataItem) => ({
                ...item,
                symbol: 'circle',
                symbolSize: 6,
                lineStyle: {
                    width: 2
                },
                areaStyle: {
                    opacity: 0.1
                }
            }))
        }]
    } ;

    // Function to toggle between views
    function toggleView(): void {
        isDetailed = !isDetailed;
        updateChart();
    }

    // Update chart when data changes
    function updateChart(): void {
        if (chart) {
            chart.setOption(option, true); // true = notMerge for complete refresh
        }
    }

    $: if (chart && isDetailed !== undefined) {
        updateChart();
    }



    onMount(() => {
        chart = echarts.init(chartContainer);
        chart.setOption(option);

        // Handle window resize
        const handleResize = (): void => {
            chart.resize();
        };
        window.addEventListener('resize', handleResize);

        // Cleanup
        return (): void => {
            window.removeEventListener('resize', handleResize);
            chart.dispose();
        };
    });
</script>

<div class="chart-wrapper ">
    <div class="h-[200px]"></div>
    <div class="toggle-container">
        <button
                class="toggle-btn"
                class:active={isDetailed}
                on:click={toggleView}
        >
            {isDetailed ? 'Switch to Consolidated View (5 stats)' : 'Switch to Detailed View (10 stats)'}
        </button>
    </div>
    <div bind:this={chartContainer} class="chart-container"></div>
</div>

<style>
    .chart-wrapper {
        justify-items: center;
        width: 100%;
        height: 200px;
        position: relative;
        /*overflow: hidden;*/
        box-sizing: border-box;
    }

    .toggle-container {
        /*position: absolute;*/
        top: 10px;
        right: 10px;
        z-index: 10;
    }

    .toggle-btn {
        background: #5470c6;
        color: white;
        border: none;
        padding: 4px 8px;
        border-radius: 6px;
        cursor: pointer;
        font-size: 10px;
        font-weight: 500;
        transition: all 0.3s ease;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .toggle-btn:hover {
        background: #4c63d2;
        transform: translateY(-1px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
    }

    .toggle-btn.active {
        background: #91cc75;
    }

    .toggle-btn.active:hover {
        background: #7cb568;
    }

    .chart-container {
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    }
</style>