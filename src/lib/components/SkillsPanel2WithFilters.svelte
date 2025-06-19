<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Employee Skills Panel - Advanced Filtering</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        tailwind.config = {
            theme: {
                extend: {
                    colors: {
                        'game-bg': '#4a5568',
                        'panel-bg': '#f7fafc',
                        'skill-high': '#48bb78',
                        'skill-medium': '#ed8936',
                        'skill-low': '#e53e3e'
                    }
                }
            }
        }
    </script>
</head>
<body class="bg-game-bg p-4">
<!-- Enhanced Skills Panel with Multi-Dimensional Filtering -->
<div class="bg-panel-bg rounded-lg p-6 shadow-lg max-w-2xl">
    <div class="flex items-center justify-between mb-6">
        <h3 class="text-xl font-semibold text-gray-800">Technical Skills Dashboard</h3>
        <button class="text-blue-600 hover:text-blue-800 text-sm font-medium">Export Skills</button>
    </div>

    <!-- Search Bar -->
    <div class="mb-4">
        <div class="relative">
            <input
                    type="text"
                    placeholder="Search skills..."
                    class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    id="skillSearch"
                    oninput="filterSkills()"
            >
            <svg class="absolute right-3 top-2.5 h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
            </svg>
        </div>
    </div>

    <!-- Primary Filters (Abstraction Level & Domain) -->
    <div class="mb-6 p-4 bg-gradient-to-r from-blue-50 to-indigo-50 rounded-xl border border-blue-100">
        <div class="space-y-4">
            <div>
                <label class="flex items-center text-sm font-semibold text-gray-800 mb-3">
                    <svg class="w-4 h-4 mr-2 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path>
                    </svg>
                    Abstraction Level
                </label>
                <div class="flex flex-wrap gap-2">
                    <button class="filter-btn abstraction-filter active" data-filter="all" data-type="abstraction" onclick="toggleFilter(this)">
                            <span class="flex items-center">
                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                                </svg>
                                All Levels
                            </span>
                    </button>
                    <button class="filter-btn abstraction-filter" data-filter="foundational" data-type="abstraction" onclick="toggleFilter(this)">
                            <span class="flex items-center">
                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                    <path d="M10.394 2.08a1 1 0 00-.788 0l-7 3a1 1 0 000 1.84L5.25 8.051a.999.999 0 01.356-.257l4-1.714a1 1 0 11.788 1.838L7.667 9.088l1.94.831a1 1 0 00.787 0l7-3a1 1 0 000-1.838l-7-3zM3.31 9.397L5 10.12v4.102a8.969 8.969 0 00-1.05-.174 1 1 0 01-.89-.89 11.115 11.115 0 01.25-3.762zM9.3 16.573A9.026 9.026 0 007 14.935v-3.957l1.818.78a3 3 0 002.364 0l5.508-2.361a11.026 11.026 0 01.25 3.762 1 1 0 01-.89.89 8.968 8.968 0 00-5.35 2.524 1 1 0 01-1.4 0zM6 18a1 1 0 001-1v-2.065a8.935 8.935 0 00-2-.712V17a1 1 0 001 1z"></path>
                                </svg>
                                Foundational
                            </span>
                    </button>
                    <button class="filter-btn abstraction-filter" data-filter="conceptual" data-type="abstraction" onclick="toggleFilter(this)">
                            <span class="flex items-center">
                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"></path>
                                </svg>
                                Conceptual
                            </span>
                    </button>
                    <button class="filter-btn abstraction-filter" data-filter="applied" data-type="abstraction" onclick="toggleFilter(this)">
                            <span class="flex items-center">
                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd"></path>
                                </svg>
                                Applied
                            </span>
                    </button>
                </div>
            </div>

            <div>
                <label class="flex items-center text-sm font-semibold text-gray-800 mb-3">
                    <svg class="w-4 h-4 mr-2 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
                    </svg>
                    Domain
                </label>
                <div class="flex flex-wrap gap-2">
                    <button class="filter-btn domain-filter active" data-filter="all" data-type="domain" onclick="toggleFilter(this)">
                            <span class="flex items-center">
                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                                </svg>
                                All Domains
                            </span>
                    </button>
                    <button class="filter-btn domain-filter" data-filter="execution" data-type="domain" onclick="toggleFilter(this)">
                            <span class="flex items-center">
                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd"></path>
                                </svg>
                                Execution
                            </span>
                    </button>
                    <button class="filter-btn domain-filter" data-filter="coordination" data-type="domain" onclick="toggleFilter(this)">
                            <span class="flex items-center">
                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                    <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3z"></path>
                                </svg>
                                Coordination
                            </span>
                    </button>
                    <button class="filter-btn domain-filter" data-filter="interpersonal" data-type="domain" onclick="toggleFilter(this)">
                            <span class="flex items-center">
                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                    <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                Interpersonal
                            </span>
                    </button>
                    <button class="filter-btn domain-filter" data-filter="contextual" data-type="domain" onclick="toggleFilter(this)">
                            <span class="flex items-center">
                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M3 3a1 1 0 000 2v8a2 2 0 002 2h2.586l-1.293 1.293a1 1 0 101.414 1.414L10 15.414l2.293 2.293a1 1 0 001.414-1.414L12.414 15H15a2 2 0 002-2V5a1 1 0 100-2H3zm11.707 4.707a1 1 0 00-1.414-1.414L10 9.586 8.707 8.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                                </svg>
                                Contextual
                            </span>
                    </button>
                </div>
            </div>
        </div>
    </div>

    <!-- Secondary Filter (Skill Type) -->
    <div class="mb-4 p-3 bg-gray-50 rounded-lg border border-gray-200">
        <details class="group">
            <summary class="flex items-center justify-between cursor-pointer text-sm font-semibold text-gray-700 hover:text-gray-900 py-1">
                    <span class="flex items-center">
                        <svg class="w-4 h-4 mr-2 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"></path>
                        </svg>
                        Skill Type (Optional Filter)
                    </span>
                <svg class="w-4 h-4 transition-transform group-open:rotate-180 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                </svg>
            </summary>
            <div class="mt-3 flex flex-wrap gap-2">
                <button class="filter-btn type-filter active" data-filter="all" data-type="skilltype" onclick="toggleFilter(this)">
                        <span class="flex items-center">
                            <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                            </svg>
                            All Types
                        </span>
                </button>
                <button class="filter-btn type-filter" data-filter="programming" data-type="skilltype" onclick="toggleFilter(this)">
                        <span class="flex items-center">
                            <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M12.316 3.051a1 1 0 01.633 1.265l-4 12a1 1 0 11-1.898-.632l4-12a1 1 0 011.265-.633zM5.707 6.293a1 1 0 010 1.414L3.414 10l2.293 2.293a1 1 0 11-1.414 1.414l-3-3a1 1 0 010-1.414l3-3a1 1 0 011.414 0zm8.586 0a1 1 0 011.414 0l3 3a1 1 0 010 1.414l-3 3a1 1 0 11-1.414-1.414L16.586 10l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                            </svg>
                            Programming
                        </span>
                </button>
                <button class="filter-btn type-filter" data-filter="design" data-type="skilltype" onclick="toggleFilter(this)">
                        <span class="flex items-center">
                            <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z" clip-rule="evenodd"></path>
                            </svg>
                            Design
                        </span>
                </button>
                <button class="filter-btn type-filter" data-filter="analysis" data-type="skilltype" onclick="toggleFilter(this)">
                        <span class="flex items-center">
                            <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M2 11a1 1 0 011-1h2a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1v-5zM8 7a1 1 0 011-1h2a1 1 0 011 1v9a1 1 0 01-1 1H9a1 1 0 01-1-1V7zM14 4a1 1 0 011-1h2a1 1 0 011 1v12a1 1 0 01-1 1h-2a1 1 0 01-1-1V4z"></path>
                            </svg>
                            Analysis
                        </span>
                </button>
                <button class="filter-btn type-filter" data-filter="collaboration" data-type="skilltype" onclick="toggleFilter(this)">
                        <span class="flex items-center">
                            <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z"></path>
                            </svg>
                            Collaboration
                        </span>
                </button>
                <button class="filter-btn type-filter" data-filter="process" data-type="skilltype" onclick="toggleFilter(this)">
                        <span class="flex items-center">
                            <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd"></path>
                            </svg>
                            Process
                        </span>
                </button>
            </div>
        </details>
    </div>

    <!-- Active Filters Display -->
    <div id="activeFilters" class="mb-4 hidden">
        <div class="flex items-center gap-2 text-sm">
            <span class="text-gray-600">Active filters:</span>
            <div id="filterTags" class="flex flex-wrap gap-1"></div>
            <button onclick="clearAllFilters()" class="text-blue-600 hover:text-blue-800 text-xs">Clear all</button>
        </div>
    </div>

    <!-- Skills Overview Stats -->
    <div class="grid grid-cols-4 gap-2 mb-4 text-center">
        <div class="bg-blue-50 rounded-lg p-2">
            <div class="text-lg font-bold text-blue-600" id="totalSkills">24</div>
            <div class="text-xs text-blue-600">Total</div>
        </div>
        <div class="bg-green-50 rounded-lg p-2">
            <div class="text-lg font-bold text-green-600" id="expertSkills">8</div>
            <div class="text-xs text-green-600">Expert</div>
        </div>
        <div class="bg-yellow-50 rounded-lg p-2">
            <div class="text-lg font-bold text-yellow-600" id="proficientSkills">12</div>
            <div class="text-xs text-yellow-600">Proficient</div>
        </div>
        <div class="bg-red-50 rounded-lg p-2">
            <div class="text-lg font-bold text-red-600" id="learningSkills">4</div>
            <div class="text-xs text-red-600">Learning</div>
        </div>
    </div>

    <!-- Skills List Container -->
    <div class="skills-container max-h-96 overflow-y-auto">
        <div class="space-y-2" id="skillsList">
            <!-- Skills will be populated by JavaScript -->
        </div>
    </div>

    <!-- Results Summary -->
    <div class="mt-4 text-center">
        <span id="resultsCount" class="text-sm text-gray-600">Showing 24 skills</span>
    </div>
</div>

<style>
    .filter-btn {
        @apply px-4 py-2 text-sm font-medium rounded-lg border-2 border-gray-200 text-gray-700 hover:border-blue-300 hover:bg-blue-50 hover:text-blue-700 transition-all duration-200 cursor-pointer shadow-sm;
        background: linear-gradient(135deg, #ffffff 0%, #f8fafc 100%);
    }
    .filter-btn.active {
        @apply bg-blue-600 text-white border-blue-600 hover:bg-blue-700 hover:border-blue-700 shadow-md;
        background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
        transform: translateY(-1px);
    }
    .filter-btn:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    }
    .filter-btn.active:hover {
        box-shadow: 0 6px 16px rgba(59, 130, 246, 0.3);
    }
    .filter-tag {
        @apply px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded-full flex items-center gap-1;
    }
    .skill-item {
        @apply flex items-center justify-between py-2 px-3 rounded-lg hover:bg-gray-50 transition-colors;
    }
    .skill-item.hidden {
        display: none !important;
    }
</style>

<script>
    // Skills data with multi-dimensional categorization
    const skillsData = [
        // Programming Skills
        { name: "JavaScript", level: 92, abstraction: "applied", domain: "execution", type: "programming" },
        { name: "Python", level: 88, abstraction: "applied", domain: "execution", type: "programming" },
        { name: "React", level: 85, abstraction: "applied", domain: "execution", type: "programming" },
        { name: "Node.js", level: 72, abstraction: "applied", domain: "execution", type: "programming" },
        { name: "TypeScript", level: 68, abstraction: "conceptual", domain: "execution", type: "programming" },
        { name: "SQL", level: 90, abstraction: "foundational", domain: "execution", type: "programming" },
        { name: "Git", level: 95, abstraction: "foundational", domain: "coordination", type: "programming" },

        // Design Skills
        { name: "UI/UX Design", level: 78, abstraction: "applied", domain: "execution", type: "design" },
        { name: "Design Systems", level: 82, abstraction: "conceptual", domain: "coordination", type: "design" },
        { name: "Prototyping", level: 75, abstraction: "applied", domain: "execution", type: "design" },
        { name: "User Research", level: 70, abstraction: "foundational", domain: "contextual", type: "design" },

        // Analysis Skills
        { name: "Data Analysis", level: 85, abstraction: "applied", domain: "execution", type: "analysis" },
        { name: "Requirements Analysis", level: 88, abstraction: "conceptual", domain: "contextual", type: "analysis" },
        { name: "Performance Analysis", level: 80, abstraction: "applied", domain: "execution", type: "analysis" },
        { name: "Risk Assessment", level: 75, abstraction: "conceptual", domain: "contextual", type: "analysis" },

        // Collaboration Skills
        { name: "Team Leadership", level: 73, abstraction: "applied", domain: "interpersonal", type: "collaboration" },
        { name: "Mentoring", level: 81, abstraction: "applied", domain: "interpersonal", type: "collaboration" },
        { name: "Cross-functional Communication", level: 85, abstraction: "foundational", domain: "interpersonal", type: "collaboration" },
        { name: "Conflict Resolution", level: 68, abstraction: "conceptual", domain: "interpersonal", type: "collaboration" },

        // Process Skills
        { name: "Agile Methodology", level: 89, abstraction: "conceptual", domain: "coordination", type: "process" },
        { name: "Code Review", level: 92, abstraction: "applied", domain: "coordination", type: "process" },
        { name: "Documentation", level: 78, abstraction: "foundational", domain: "coordination", type: "process" },
        { name: "Quality Assurance", level: 83, abstraction: "applied", domain: "coordination", type: "process" },
        { name: "Project Planning", level: 76, abstraction: "conceptual", domain: "coordination", type: "process" },
        { name: "Stakeholder Management", level: 71, abstraction: "applied", domain: "contextual", type: "process" }
    ];

    let currentFilters = {
        abstraction: 'all',
        domain: 'all',
        skilltype: 'all',
        search: ''
    };

    function initializeSkills() {
        renderSkills();
        updateStats();
    }

    function renderSkills() {
        const skillsList = document.getElementById('skillsList');
        const filteredSkills = getFilteredSkills();

        skillsList.innerHTML = filteredSkills.map(skill => {
            const levelColor = skill.level >= 80 ? 'green' : skill.level >= 60 ? 'yellow' : 'red';
            return `
                    <div class="skill-item" data-skill='${JSON.stringify(skill)}'>
                        <div class="flex items-center space-x-3">
                            <div class="w-2 h-2 bg-${levelColor}-500 rounded-full"></div>
                            <div>
                                <span class="text-sm font-medium text-gray-700">${skill.name}</span>
                                <div class="flex items-center space-x-2 text-xs text-gray-500">
                                    <span class="px-1.5 py-0.5 bg-gray-100 rounded">${skill.abstraction}</span>
                                    <span class="px-1.5 py-0.5 bg-blue-100 rounded">${skill.domain}</span>
                                    <span class="px-1.5 py-0.5 bg-purple-100 rounded">${skill.type}</span>
                                </div>
                            </div>
                        </div>
                        <div class="flex items-center space-x-2">
                            <div class="w-16 bg-gray-200 rounded-full h-1.5">
                                <div class="bg-${levelColor}-500 h-1.5 rounded-full" style="width: ${skill.level}%"></div>
                            </div>
                            <span class="text-xs text-gray-600 w-8">${skill.level}</span>
                        </div>
                    </div>
                `;
        }).join('');

        updateResultsCount(filteredSkills.length);
    }

    function getFilteredSkills() {
        return skillsData.filter(skill => {
            const matchesAbstraction = currentFilters.abstraction === 'all' || skill.abstraction === currentFilters.abstraction;
            const matchesDomain = currentFilters.domain === 'all' || skill.domain === currentFilters.domain;
            const matchesType = currentFilters.skilltype === 'all' || skill.type === currentFilters.skilltype;
            const matchesSearch = currentFilters.search === '' || skill.name.toLowerCase().includes(currentFilters.search.toLowerCase());

            return matchesAbstraction && matchesDomain && matchesType && matchesSearch;
        });
    }

    function toggleFilter(button) {
        const filterType = button.dataset.type;
        const filterValue = button.dataset.filter;

        // Remove active class from siblings
        document.querySelectorAll(`[data-type="${filterType}"]`).forEach(btn => {
            btn.classList.remove('active');
        });

        // Add active class to clicked button
        button.classList.add('active');

        // Update current filters
        currentFilters[filterType] = filterValue;

        // Re-render skills and update stats
        renderSkills();
        updateStats();
        updateActiveFilters();
    }

    function updateActiveFilters() {
        const activeFiltersDiv = document.getElementById('activeFilters');
        const filterTags = document.getElementById('filterTags');

        const activeFilters = [];

        if (currentFilters.abstraction !== 'all') {
            activeFilters.push({ type: 'Abstraction', value: currentFilters.abstraction });
        }
        if (currentFilters.domain !== 'all') {
            activeFilters.push({ type: 'Domain', value: currentFilters.domain });
        }
        if (currentFilters.skilltype !== 'all') {
            activeFilters.push({ type: 'Type', value: currentFilters.skilltype });
        }

        if (activeFilters.length > 0) {
            activeFiltersDiv.classList.remove('hidden');
            filterTags.innerHTML = activeFilters.map(filter => `
                    <span class="filter-tag">
                        ${filter.type}: ${filter.value}
                        <button onclick="removeFilter('${filter.type.toLowerCase()}')" class="ml-1 hover:text-red-600">×</button>
                    </span>
                `).join('');
        } else {
            activeFiltersDiv.classList.add('hidden');
        }
    }

    function removeFilter(filterType) {
        const mappedType = filterType === 'type' ? 'skilltype' : filterType;
        currentFilters[mappedType] = 'all';

        // Update button states
        document.querySelectorAll(`[data-type="${mappedType}"]`).forEach(btn => {
            btn.classList.remove('active');
            if (btn.dataset.filter === 'all') {
                btn.classList.add('active');
            }
        });

        renderSkills();
        updateStats();
        updateActiveFilters();
    }

    function clearAllFilters() {
        currentFilters = {
            abstraction: 'all',
            domain: 'all',
            skilltype: 'all',
            search: ''
        };

        // Reset all filter buttons
        document.querySelectorAll('.filter-btn').forEach(btn => {
            btn.classList.remove('active');
            if (btn.dataset.filter === 'all') {
                btn.classList.add('active');
            }
        });

        // Clear search
        document.getElementById('skillSearch').value = '';

        renderSkills();
        updateStats();
        updateActiveFilters();
    }

    function filterSkills() {
        const searchTerm = document.getElementById('skillSearch').value;
        currentFilters.search = searchTerm;
        renderSkills();
        updateStats();
    }

    function updateStats() {
        const filteredSkills = getFilteredSkills();
        const expertCount = filteredSkills.filter(s => s.level >= 80).length;
        const proficientCount = filteredSkills.filter(s => s.level >= 60 && s.level < 80).length;
        const learningCount = filteredSkills.filter(s => s.level < 60).length;

        document.getElementById('totalSkills').textContent = filteredSkills.length;
        document.getElementById('expertSkills').textContent = expertCount;
        document.getElementById('proficientSkills').textContent = proficientCount;
        document.getElementById('learningSkills').textContent = learningCount;
    }

    function updateResultsCount(count) {
        document.getElementById('resultsCount').textContent = `Showing ${count} skill${count !== 1 ? 's' : ''}`;
    }

    // Initialize the skills panel
    document.addEventListener('DOMContentLoaded', initializeSkills);
</script>
</body>
</html>