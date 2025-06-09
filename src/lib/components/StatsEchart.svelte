<script lang="ts">
    import {onMount} from 'svelte';
    import type {ECharts} from 'echarts';
    import * as echarts from 'echarts';
    import {
        getGroupedStatSnapshot,
        type GroupedStatSnapshot, type StatDefinition, type StatGroupMetadata, type StatKey,
        type StatsSnapshot,
        type StatWithValue
    } from "$lib/models/stats";
    import type {Readable} from "svelte/store";
    import type {PersonSnapshot} from "$lib/stores/persons";

    export let personStore:Readable<PersonSnapshot>;
    let statsSnapshot:StatsSnapshot;
    $: statsSnapshot = $personStore.stats;
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

    let statRadarOrder: StatKey[];
    statRadarOrder = [ "judgement", "systems", "focus", "empathy","resilience", "creativity","precision","discipline","communication", "adaptability" ].reverse();
    //cognition, perception, drive, social defence
    interface TooltipParams {
        seriesName: string;
        dataIndex: number;
        value: number[];
        color: string;
        name: string;
        dimensionNames?: string[];
        encode?: any;
    }

    let chartContainer: HTMLDivElement;
    let chart: ECharts;
    $: isDetailed = false; // Toggle between detailed (10) and consolidated (5) view
    let groupedStats: GroupedStatSnapshot[];
    $: groupedStats = (()=>{
        //echarts radar graph shows stuff in counterclockwise, we have to reverse the list to achieve the clockwise order.
        let val = getGroupedStatSnapshot(statsSnapshot);
        val.reverse();
        val.map(v => {// sub array also needs to be reversed.
           v.items.reverse();
           return v;
        });
        return val;
    })();
    let detailedChartData: ChartDataItem[] = [];
    let consolidatedChartData: ChartDataItem[] = [];
    let detailedRadarIndicators: RadarIndicator[] = [];
    let consolidatedRadarIndicators: RadarIndicator[] = [];
    let detailedChartDataInfo = [];
    $: {
        consolidatedRadarIndicators = groupedStats.map(stat => ({
            name: stat.group.name,
            max: 100
        })
        );

        consolidatedChartData = [{
            name: 'Stat group average',
            value: groupedStats.map(stat => stat.group.average),
            itemStyle: {
                color: '#5470c6'
            }
        }];
    }

    $:{
        const stats = groupedStats.flatMap(stat =>
            stat.items.map(statDef => (statDef)) );


        detailedRadarIndicators =
            statRadarOrder.map( def => {
                const s =stats.find(s => s.key === def);

                return {
                    name: s.label,
                    max: 100,
                    color: '#ff0000'
                }
            })
            // statRadarOrder.map(stat => {
            //     return {
            //         name: stat,
            //         max: 100
            //     }
            //
            // });
        detailedChartDataInfo = statRadarOrder.map( def => stats.find(s => s.key === def).value);
        detailedChartData = [{
            name: 'Stat value',
            value: detailedChartDataInfo,
            itemStyle: {
                color: '#5470c6'
            }
        }];
    }

    // Reactive values that change based on toggle
    $: currentChartData = isDetailed ? detailedChartData : consolidatedChartData;
    $: currentRadarIndicators = isDetailed ? detailedRadarIndicators : consolidatedRadarIndicators;
    $: startAngle = isDetailed ? 90 + (360 /10 ) : 90 + (360/5);
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
            formatter: function(params: TooltipParams | TooltipParams[]): string {
                // Handle both single item and array cases
                const param = Array.isArray(params) ? params[0] : params;

                // For radar charts, we need to show all dimensions for the hovered series
                const values = param.value as number[];
                const seriesName = param.name;
                const color = param.color;

                let tooltipContent = `
          <div style="padding: 8px;">
            <strong style="color: ${color};">${seriesName}</strong><br/>
        `;

                // Show all attributes for this series
                currentRadarIndicators.forEach((indicator: RadarIndicator, index: number) => {
                    const value = values[index];
                    tooltipContent += `
            <div style="margin: 4px 0;">
              <span style="color: ${color};">●</span>
              ${indicator.name}: <strong>${value}</strong>
              ${indicator.category ? ` <small>(${indicator.category})</small>` : ''}
            </div>
          `;
                });

                tooltipContent += '</div>';
                return tooltipContent;
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
            },
            startAngle:startAngle,


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
        }],
        
    };

    // Function to toggle between views
    function toggleView(): void {
        isDetailed = !isDetailed;
        // updateChart();
    }

    // Update chart when data changes
    function updateChart(): void {
        if (chart && option) { // Ensure option is also ready
            chart.setOption(option, true); // true = notMerge for complete refresh
        }
    }

    // This reactive block will now trigger updateChart whenever 'option' changes,
    // which includes changes originating from 'personStore' or 'isDetailed'.
    $: if (chart && option) {
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