<svelte:options runes={false} />

<script>


  let searchTerm = $state('');
  let activeCategory = $state('programming');
  let skills = $state({});

  // Subscribe to the store
  skillsData.subscribe(value => {
    skills = value;
  });

  // Reactive statements for computed values
  let stats = $derived((() => {
    const allSkills = [...(skills.programming || []), ...(skills.tools || []), ...(skills.soft || [])];
    return {
      expert: allSkills.filter(skill => skill.proficiency === 'expert').length,
      proficient: allSkills.filter(skill => skill.proficiency === 'proficient').length,
      learning: allSkills.filter(skill => skill.proficiency === 'learning').length
    };
  })());

  // Filter skills based on search term
  let filteredSkills = $derived((() => {
    if (!searchTerm || !skills[activeCategory]) return skills[activeCategory] || [];
    return skills[activeCategory].filter(skill =>
            skill.name.toLowerCase().includes(searchTerm.toLowerCase())
    );
  })());

  function showCategory(category) {
    activeCategory = category;
  }

  function getProficiencyColor(proficiency) {
    switch (proficiency) {
      case 'expert': return 'bg-green-500';
      case 'proficient': return 'bg-yellow-500';
      case 'learning': return 'bg-red-500';
      default: return 'bg-gray-500';
    }
  }
</script>

<div class="bg-gray-600 p-4 min-h-screen">
  <!-- Enhanced Skills Panel -->
  <div class="bg-gray-50 rounded-lg p-6 shadow-lg max-w-md mx-auto">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-semibold text-gray-800">Technical Skills</h3>
      <button class="text-blue-600 hover:text-blue-800 text-sm font-medium">View All</button>
    </div>

    <!-- Search Bar -->
    <div class="mb-4">
      <div class="relative">
        <input
                type="text"
                placeholder="Search skills..."
                class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                bind:value={searchTerm}
        />
        <svg class="absolute right-3 top-2.5 h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
        </svg>
      </div>
    </div>

    <!-- Skill Categories Tabs -->
    <div class="flex space-x-1 mb-4 bg-gray-100 rounded-lg p-1">
      <button
              class="px-3 py-1 text-xs font-medium rounded-md {activeCategory === 'programming' ? 'bg-white text-gray-700 shadow-sm' : 'text-gray-600 hover:text-gray-700'}"
              onclick={() => showCategory('programming')}
      >
        Programming
      </button>
      <button
              class="px-3 py-1 text-xs font-medium rounded-md {activeCategory === 'tools' ? 'bg-white text-gray-700 shadow-sm' : 'text-gray-600 hover:text-gray-700'}"
              onclick={() => showCategory('tools')}
      >
        Tools
      </button>
      <button
              class="px-3 py-1 text-xs font-medium rounded-md {activeCategory === 'soft' ? 'bg-white text-gray-700 shadow-sm' : 'text-gray-600 hover:text-gray-700'}"
              onclick={() => showCategory('soft')}
      >
        Soft
      </button>
    </div>

    <!-- Skills Overview Stats -->
    <div class="grid grid-cols-3 gap-2 mb-4 text-center">
      <div class="bg-green-50 rounded-lg p-2">
        <div class="text-lg font-bold text-green-600">{stats.expert}</div>
        <div class="text-xs text-green-600">Expert</div>
      </div>
      <div class="bg-yellow-50 rounded-lg p-2">
        <div class="text-lg font-bold text-yellow-600">{stats.proficient}</div>
        <div class="text-xs text-yellow-600">Proficient</div>
      </div>
      <div class="bg-red-50 rounded-lg p-2">
        <div class="text-lg font-bold text-red-600">{stats.learning}</div>
        <div class="text-xs text-red-600">Learning</div>
      </div>
    </div>

    <!-- Skills List Container -->
    <div class="skills-container max-h-64 overflow-y-auto">
      <div class="space-y-2">
        {#each filteredSkills as skill (skill.name)}
          <div class="flex items-center justify-between py-1">
            <div class="flex items-center space-x-2">
              <div class="w-2 h-2 {getProficiencyColor(skill.proficiency)} rounded-full"></div>
              <span class="text-sm font-medium text-gray-700">{skill.name}</span>
            </div>
            <div class="flex items-center space-x-2">
              <div class="w-16 bg-gray-200 rounded-full h-1.5">
                <div class="{getProficiencyColor(skill.proficiency)} h-1.5 rounded-full" style="width: {skill.level}%"></div>
              </div>
              <span class="text-xs text-gray-600 w-8">{skill.level}</span>
            </div>
          </div>
        {/each}

        {#if filteredSkills.length === 0 && searchTerm}
          <div class="text-center text-gray-500 py-4">
            No skills found matching "{searchTerm}"
          </div>
        {/if}
      </div>
    </div>

    <!-- Show More Button -->
    <div class="mt-4 text-center">
      <button class="text-blue-600 hover:text-blue-800 text-sm font-medium">
        Show {(skills[activeCategory] || []).length - filteredSkills.length} more skills...
      </button>
    </div>
  </div>
</div>
