<script lang="ts">
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {goto} from "$app/navigation";

    // let companyName = "";
    let difficulty = "Standard";
    let startingCapital = "$100,000";

    type EmployeeArchetype =
        | 'Basic'
        | 'Apt'
        | 'Sharp'
        | 'Gifted'
        | 'Brilliant'
        | 'Exceptional';

    type StartingEmployeesConfig = {
        name: string;
        config: [EmployeeArchetype, number][];
    };

    type CompanyPreset = {
        name:string;
        slogan:string;
    }

    let employeeConfigs: StartingEmployeesConfig[] = [];

    let companyName = '';
    let slogan = '';

    // Default all values to 0
    let talentConfig: Record<EmployeeArchetype, number> = {
        Basic: 0,
        Apt: 0,
        Sharp: 0,
        Gifted: 0,
        Brilliant: 0,
        Exceptional: 0
    };
    let companyPresets:CompanyPreset[];
    onMount(async () => {
        employeeConfigs = await invoke("get_starting_employee_configs");
        companyPresets = await invoke("get_company_presets");
        applyConfig(employeeConfigs[0])
        companyName = companyPresets[0].name;
        slogan = companyPresets[0].slogan;
    });

    function applyConfig(config: StartingEmployeesConfig) {
        // Reset all to 0
        for (let key in talentConfig) {
            talentConfig[key as EmployeeArchetype] = 0;
        }
        // Set values from selected config
        for (const [type, count] of config.config) {
            talentConfig[type] = count;
        }
    }
    let selectedEmployeeConfigIndex =0;
    let selectedCompanyIndex = 0;
    function pickRandomCompany() {
        if (companyPresets.length === 0) return;
        selectedCompanyIndex = Math.floor(Math.random() * companyPresets.length);
        const random = companyPresets[selectedCompanyIndex];
        companyName = random.name;
        slogan = random.slogan;
    }

    // Handle game start
    async function startGame() {
        const currentCompanyPreset: CompanyPreset = {
            name: companyName,
            slogan: slogan,
        };

        const currentEmployeeConfig: StartingEmployeesConfig = {
            name: `${companyName} - Custom`, // Or any other naming convention
            config: Object.entries(talentConfig)
                .filter(([, count]) => count > 0) // Optionally filter out archetypes with 0 count
                .map(([type, count]) => [type as EmployeeArchetype, count])
        };

        console.log("Starting new game with compiled settings:");
        console.log("Company Preset:", currentCompanyPreset);
        console.log("Employee Config:", currentEmployeeConfig);
        console.log(`Difficulty: ${difficulty}, Starting Capital: ${startingCapital}`);

        await  invoke("new_sim",{company:currentCompanyPreset, employee: currentEmployeeConfig});
        await  invoke('resume_sim');
        // await goto("/game");
        // Navigate to game screen or trigger game initialization




    }

</script>
<div class="text-lg font-bold mb-3">New Game</div>
<div class="text-sm text-slate-300 mb-4">
    Start a new software company from scratch. Hire talented developers, manage
    projects, and grow your business into a tech empire.
</div>

<!-- Game Options -->
<div class="space-y-4 mb-6">
    <div class="space-y-4">
        <div class="relative">
            <label class="block text-xs text-slate-400 mb-1">Company Name</label>
            <input
                    type="text"
                    bind:value={companyName}
                    placeholder="Enter company name..."
                    class="w-full pr-10 bg-slate-900 border border-slate-700 rounded px-3 py-2 text-slate-200 focus:outline-none focus:ring-1 focus:ring-green-500"
            />
            <button
                    type="button"
                    class="absolute right-2 top-7 text-slate-400 hover:text-green-400"
                    on:click={pickRandomCompany}
                    title="Randomize company"
            >
                🎲
            </button>
        </div>


        <div>
            <label class="block text-xs text-slate-400 mb-1">Slogan</label>
            <input
                    type="text"
                    bind:value={slogan}
                    placeholder="Enter company slogan..."
                    class="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-slate-200 focus:outline-none focus:ring-1 focus:ring-green-500"
            />
        </div>

        <label class="block text-xs text-slate-400 mb-1">Starting employees presets:</label>
        <div class="rounded-lg border border-slate-700 p-3 space-y-2">
            {#each Object.keys(talentConfig) as type}
                <div class="flex justify-between items-center">
                    <label for={type} class="text-slate-300">{type}</label>
                    <input
                            type="number"
                            id={type}
                            name={type}
                            min="0"
                            bind:value={talentConfig[type]}
                            class="w-20 px-2 bg-slate-900 border border-slate-700 rounded text-slate-200 focus:outline-none focus:ring-1 focus:ring-green-500"
                    />
                </div>
            {/each}
            <div>
                <label class="block text-xs text-slate-400 mb-1">Presets:</label>
                <ul class="space-y-1 max-h-48 overflow-auto">
                    {#each employeeConfigs as config, i}
                        <li
                                class="px-3 py-2 border  rounded cursor-pointer hover:bg-slate-700 transition-colors
             {selectedEmployeeConfigIndex === i ? 'bg-green-900 border-green-700 text-green-400' : 'bg-slate-900 text-slate-300 border-slate-700'}"
                                on:click={() => {applyConfig(config);
                                                        selectedEmployeeConfigIndex=i;}}
                        >
                            {config.name}
                        </li>
                    {/each}
                </ul>
            </div>
        </div>


    </div>
    <div>
        <label class="block text-xs text-slate-400 mb-1">Difficulty</label>
        <div class="grid grid-cols-3 gap-2">
            <button
                    class="px-3 py-2 {difficulty === 'Casual' ? 'bg-green-900 border-green-700 text-green-400' : 'bg-slate-900 border-slate-700'} border rounded text-center hover:bg-slate-700"
                    on:click={() => difficulty = 'Casual'}
            >
                Casual
            </button>
            <button
                    class="px-3 py-2 {difficulty === 'Standard' ? 'bg-green-900 border-green-700 text-green-400' : 'bg-slate-900 border-slate-700'} border rounded text-center hover:bg-slate-700"
                    on:click={() => difficulty = 'Standard'}
            >
                Standard
            </button>
            <button
                    class="px-3 py-2 {difficulty === 'Expert' ? 'bg-green-900 border-green-700 text-green-400' : 'bg-slate-900 border-slate-700'} border rounded text-center hover:bg-slate-700"
                    on:click={() => difficulty = 'Expert'}
            >
                Expert
            </button>
        </div>
    </div>

    <div>
        <label class="block text-xs text-slate-400 mb-1">Starting Capital</label>
        <div class="grid grid-cols-3 gap-2">
            <button
                    class="px-3 py-2 {startingCapital === '$50,000' ? 'bg-green-900 border-green-700 text-green-400' : 'bg-slate-900 border-slate-700'} border rounded text-center hover:bg-slate-700"
                    on:click={() => startingCapital = '$50,000'}
            >
                $50,000
            </button>
            <button
                    class="px-3 py-2 {startingCapital === '$100,000' ? 'bg-green-900 border-green-700 text-green-400' : 'bg-slate-900 border-slate-700'} border rounded text-center hover:bg-slate-700"
                    on:click={() => startingCapital = '$100,000'}
            >
                $100,000
            </button>
            <button
                    class="px-3 py-2 {startingCapital === '$200,000' ? 'bg-green-900 border-green-700 text-green-400' : 'bg-slate-900 border-slate-700'} border rounded text-center hover:bg-slate-700"
                    on:click={() => startingCapital = '$200,000'}
            >
                $200,000
            </button>
        </div>
    </div>
</div>

<!-- Start Button -->
<div class="mt-auto">
    <button
            class="w-full py-3 bg-green-600 hover:bg-green-700 rounded font-bold text-white flex items-center justify-center"
            on:click={startGame}
            disabled={!companyName}
    >
        <span class="mr-2">Start New Company</span>
        <span>▶</span>
    </button>
</div>
