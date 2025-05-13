<script>
  import { tick } from '$lib/stores/tick';

  import { onMount } from 'svelte';
  import { 
    Activity, AlertCircle, BarChart2, Bell, Calendar, 
    ChevronDown, ChevronRight, Clock, Code, Coffee, 
    CreditCard, HelpCircle, Info, LayoutDashboard, 
    MessageSquare, Pause, Play, Plus, Settings, 
    SkipForward, Smile, User, Users, X
  } from 'lucide-svelte';

  // State management
  let tabs = [
    { id: "overview", title: "Overview", type: "system" },
    { id: "reports", title: "Reports", type: "system" }
  ];
  let activeTabId = "overview";
  let expandedTeams = {
    "Development Team": true,
    "QA Team": false,
    "Business Analyst Team": false,
    "Management Team": false
  };
  let isPlaying = true;
  let simulationSpeed = "1x";
  let activeView = "Dashboard";
  let nextTabId = 1;

  // Employee data
  const employees = {
    "Development Team": [
      {
        id: 1,
        initials: "JS",
        name: "John Smith",
        role: "Senior Developer",
        task: "API Integration",
        project: "Project Alpha",
        taskStatus: "green",
        mood: 65,
        energy: 30,
        stress: 80,
        skills: {
          coding: 92,
          design: 45,
          testing: 78,
          teamwork: 65
        },
        details: {
          email: "john.smith@devcorp.com",
          phone: "(555) 123-4567",
          hireDate: "2023-05-15",
          salary: "$120,000",
          projects: ["Project Alpha", "Project Gamma"],
          notes: "John is a skilled developer but has been showing signs of stress lately. Consider reducing workload or scheduling time off."
        }
      },
      {
        id: 2,
        initials: "SL",
        name: "Sarah Lee",
        role: "Frontend Developer",
        task: "UI Components",
        project: "Project Beta",
        taskStatus: "blue",
        mood: 90,
        energy: 60,
        stress: 25,
        skills: {
          coding: 75,
          design: 95,
          testing: 60,
          teamwork: 85
        },
        details: {
          email: "sarah.lee@devcorp.com",
          phone: "(555) 234-5678",
          hireDate: "2023-08-10",
          salary: "$95,000",
          projects: ["Project Beta"],
          notes: "Sarah consistently delivers high-quality work and has excellent design skills. Consider for promotion to Senior Frontend Developer."
        }
      },
      {
        id: 3,
        initials: "MJ",
        name: "Mike Johnson",
        role: "Backend Developer",
        task: "Database Migration",
        project: "Project Alpha",
        taskStatus: "red",
        mood: 20,
        energy: 15,
        stress: 95,
        skills: {
          coding: 88,
          design: 40,
          testing: 92,
          teamwork: 55
        },
        details: {
          email: "mike.johnson@devcorp.com",
          phone: "(555) 345-6789",
          hireDate: "2022-11-20",
          salary: "$105,000",
          projects: ["Project Alpha", "Project Delta"],
          notes: "URGENT: Mike is experiencing burnout symptoms. Schedule immediate meeting with HR and consider mandatory time off."
        },
        warning: true
      }
    ],
    "QA Team": [
      {
        id: 4,
        initials: "AT",
        name: "Amy Thompson",
        role: "QA Lead",
        task: "Test Automation",
        project: "Project Beta",
        taskStatus: "green",
        mood: 85,
        energy: 70,
        stress: 40,
        skills: {
          coding: 65,
          design: 50,
          testing: 98,
          teamwork: 80
        },
        details: {
          email: "amy.thompson@devcorp.com",
          phone: "(555) 456-7890",
          hireDate: "2023-02-15",
          salary: "$110,000",
          projects: ["Project Beta", "Project Alpha"],
          notes: "Amy has been instrumental in improving our test automation framework."
        }
      },
      {
        id: 5,
        initials: "RB",
        name: "Robert Brown",
        role: "QA Engineer",
        task: "Regression Testing",
        project: "Project Alpha",
        taskStatus: "amber",
        mood: 60,
        energy: 55,
        stress: 50,
        skills: {
          coding: 60,
          design: 40,
          testing: 90,
          teamwork: 75
        },
        details: {
          email: "robert.brown@devcorp.com",
          phone: "(555) 567-8901",
          hireDate: "2023-04-10",
          salary: "$85,000",
          projects: ["Project Alpha"],
          notes: "Robert is a thorough tester with a keen eye for detail."
        }
      }
    ],
    "Business Analyst Team": [
      {
        id: 6,
        initials: "EW",
        name: "Emma Wilson",
        role: "Senior BA",
        task: "Requirements Gathering",
        project: "Project Delta",
        taskStatus: "blue",
        mood: 75,
        energy: 65,
        stress: 45,
        skills: {
          coding: 30,
          design: 70,
          testing: 65,
          teamwork: 95
        },
        details: {
          email: "emma.wilson@devcorp.com",
          phone: "(555) 678-9012",
          hireDate: "2022-09-01",
          salary: "$100,000",
          projects: ["Project Delta", "Project Gamma"],
          notes: "Emma excels at stakeholder communication and requirements elicitation."
        }
      }
    ],
    "Management Team": [
      {
        id: 7,
        initials: "DM",
        name: "David Miller",
        role: "Project Manager",
        task: "Sprint Planning",
        project: "All Projects",
        taskStatus: "green",
        mood: 70,
        energy: 75,
        stress: 60,
        skills: {
          coding: 40,
          design: 60,
          testing: 50,
          teamwork: 90
        },
        details: {
          email: "david.miller@devcorp.com",
          phone: "(555) 789-0123",
          hireDate: "2022-06-15",
          salary: "$125,000",
          projects: ["Project Alpha", "Project Beta", "Project Delta", "Project Gamma"],
          notes: "David is an effective manager who keeps projects on track while maintaining team morale."
        }
      }
    ]
  };

  // Event log data
  const eventLog = [
    { time: "09:45", timeAgo: "just now", message: "Sarah completed the login page feature", type: "green" },
    { time: "09:42", timeAgo: "3m ago", message: "John is experiencing high stress levels", type: "amber" },
    { time: "09:30", timeAgo: "15m ago", message: "New project proposal received", type: "blue" },
    { time: "09:15", timeAgo: "30m ago", message: "Server outage affecting development", type: "red" }
  ];

  // Tab management
  function switchTab(tabId) {
    activeTabId = tabId;
  }

  function closeTab(tabId) {
    const index = tabs.findIndex(tab => tab.id === tabId);
    if (index !== -1) {
      tabs = tabs.filter(tab => tab.id !== tabId);
      
      // If we're closing the active tab, switch to another tab
      if (activeTabId === tabId) {
        // Try to activate the tab to the left, or the first tab if there's no tab to the left
        if (index > 0) {
          activeTabId = tabs[index - 1].id;
        } else if (tabs.length > 0) {
          activeTabId = tabs[0].id;
        }
      }
    }
  }

  function addNewTab() {
    const newTabId = `new-tab-${nextTabId++}`;
    tabs = [...tabs, { id: newTabId, title: "New Tab", type: "system" }];
    activeTabId = newTabId;
  }

  // Team expansion toggle
  function toggleTeam(team) {
    expandedTeams[team] = !expandedTeams[team];
  }

  // Employee selection - opens in a new tab
  function openEmployeeTab(employee) {
    // Check if this employee already has an open tab
    const existingTab = tabs.find(tab => tab.type === "employee" && tab.employeeId === employee.id);
    
    if (existingTab) {
      // If tab already exists, just switch to it
      activeTabId = existingTab.id;
    } else {
      // Create a new tab for this employee
      const newTabId = `employee-${employee.id}`;
      tabs = [...tabs, { 
        id: newTabId, 
        title: employee.name, 
        type: "employee", 
        employeeId: employee.id,
        employee: employee
      }];
      activeTabId = newTabId;
    }
  }

  // Simulation controls
  function toggleSimulation() {
    isPlaying = !isPlaying;
  }

  function changeSpeed(event) {
    simulationSpeed = event.target.value;
  }

  // Navigation
  function navigateTo(view) {
    activeView = view;
    // In a real app, this would change the content displayed
  }

  // Helper function to get the current employee for an employee tab
  function getEmployeeById(id) {
    for (const team in employees) {
      const employee = employees[team].find(e => e.id === id);
      if (employee) return employee;
    }
    return null;
  }

  // Clock
  let currentTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  
  onMount(() => {
    const timer = setInterval(() => {
      currentTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }, 60000);
    
    return () => {
      clearInterval(timer);
    };
  });
</script>

<div class="flex h-screen w-full bg-slate-100 font-mono text-sm">
  <!-- Sidebar -->
  <div class="w-64 bg-slate-800 text-slate-200 flex flex-col border-r border-slate-700">
    <div class="p-4 border-b border-slate-700 flex items-center justify-between">
      <h1 class="font-bold text-lg tracking-tight">DevCorp Sim</h1>
      <div class="flex space-x-1">
        <button class="p-1 hover:bg-slate-700 rounded">
          <Settings size={16} />
        </button>
        <button class="p-1 hover:bg-slate-700 rounded">
          <HelpCircle size={16} />
        </button>
      </div>
    </div>

    <div class="flex-1 overflow-auto">
      <div class="p-2">
        <div class="text-xs uppercase text-slate-500 font-bold px-3 py-2">Navigation</div>
        <div class="space-y-1">
          <button 
            class="flex items-center w-full px-3 py-2 rounded {activeView === 'Dashboard' ? 'bg-slate-700 text-slate-100' : 'hover:bg-slate-700 text-slate-300'}"
            on:click={() => navigateTo('Dashboard')}
          >
            <LayoutDashboard size={16} class="mr-2" />
            <span>Dashboard</span>
          </button>
          <button 
            class="flex items-center w-full px-3 py-2 rounded {activeView === 'Personnel' ? 'bg-slate-700 text-slate-100' : 'hover:bg-slate-700 text-slate-300'}"
            on:click={() => navigateTo('Personnel')}
          >
            <Users size={16} class="mr-2" />
            <span>Personnel</span>
          </button>
          <button 
            class="flex items-center w-full px-3 py-2 rounded {activeView === 'Projects' ? 'bg-slate-700 text-slate-100' : 'hover:bg-slate-700 text-slate-300'}"
            on:click={() => navigateTo('Projects')}
          >
            <Code size={16} class="mr-2" />
            <span>Projects</span>
          </button>
          <button 
            class="flex items-center w-full px-3 py-2 rounded {activeView === 'Finances' ? 'bg-slate-700 text-slate-100' : 'hover:bg-slate-700 text-slate-300'}"
            on:click={() => navigateTo('Finances')}
          >
            <CreditCard size={16} class="mr-2" />
            <span>Finances</span>
          </button>
          <button 
            class="flex items-center w-full px-3 py-2 rounded {activeView === 'Analytics' ? 'bg-slate-700 text-slate-100' : 'hover:bg-slate-700 text-slate-300'}"
            on:click={() => navigateTo('Analytics')}
          >
            <BarChart2 size={16} class="mr-2" />
            <span>Analytics</span>
          </button>
        </div>
      </div>

      <!-- Event Log -->
      <div class="p-2 mt-4">
        <div class="flex items-center justify-between px-3 py-2">
          <div class="text-xs uppercase text-slate-500 font-bold">Event Log</div>
          <div class="flex items-center space-x-1">
            <span class="text-xs text-slate-400">12 new</span>
            <div class="w-2 h-2 bg-amber-400 rounded-full"></div>
          </div>
        </div>
        <div class="bg-slate-900 rounded border border-slate-700 max-h-48 overflow-y-auto">
          {#each eventLog as event}
            <div class="p-2 border-l-2 border-{event.type}-500 hover:bg-slate-800">
              <div class="flex justify-between items-start">
                <span class="text-xs text-{event.type}-400">{event.time}</span>
                <span class="text-xs text-slate-500">{event.timeAgo}</span>
              </div>
              <p class="text-xs mt-1">{event.message}</p>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Simulation Controls -->
    <div class="p-3 border-t border-slate-700 bg-slate-900">
      <div class="flex items-center justify-between mb-2">
        <div class="text-xs font-bold">SIMULATION</div>
        <div class="text-xs text-slate-400">Day 42 • Q2 2025</div>
      </div>
      <div class="flex items-center justify-between">
        <div class="flex space-x-1">
          <button 
            class="p-1.5 {!isPlaying ? 'bg-green-700 hover:bg-green-600' : 'bg-slate-800 hover:bg-slate-700'} rounded"
            on:click={toggleSimulation}
          >
            <Pause size={14} />
          </button>
          <button 
            class="p-1.5 {isPlaying ? 'bg-green-700 hover:bg-green-600' : 'bg-slate-800 hover:bg-slate-700'} rounded"
            on:click={toggleSimulation}
          >
            <Play size={14} />
          </button>
          <button class="p-1.5 bg-slate-800 hover:bg-slate-700 rounded">
            <SkipForward size={14} />
          </button>
        </div>
        <div class="flex items-center space-x-2">
          <div class="text-xs">Speed:</div>
          <select 
            class="bg-slate-800 border border-slate-700 rounded text-xs p-1"
            bind:value={simulationSpeed}
            on:change={changeSpeed}
          >
            <option>1x</option>
            <option>2x</option>
            <option>3x</option>
          </select>
        </div>
      </div>
    </div>
  </div>

  <!-- Main Content -->
  <div class="flex-1 flex flex-col overflow-hidden">
    <!-- Top Bar -->
    <div class="bg-slate-800 text-slate-200 p-2 flex items-center justify-between border-b border-slate-700">
      <div class="flex items-center space-x-4">
        <div class="flex items-center space-x-2">
          <Clock size={14} />
          <span class="text-xs">{currentTime}</span>
        </div>
        <div class="flex items-center space-x-2">
          <CreditCard size={14} />
          <span class="text-xs">$124,500</span>
        </div>
        <div class="flex items-center space-x-2">
          <Users size={14} />
          <span class="text-xs">12 Employees</span>
        </div>
      </div>
      <div class="flex items-center space-x-3">
        <button class="relative p-1 hover:bg-slate-700 rounded">
          <Bell size={16} />
          <span class="absolute top-0 right-0 w-2 h-2 bg-red-500 rounded-full"></span>
        </button>
        <button class="p-1 hover:bg-slate-700 rounded">
          <Calendar size={16} />
        </button>
        <button class="flex items-center space-x-1 p-1 hover:bg-slate-700 rounded">
          <User size={16} />
          <span class="text-xs">Admin</span>
        </button>
      </div>
    </div>

    <!-- Tabs -->
    <div class="bg-slate-700 border-b border-slate-300 flex overflow-x-auto">
      {#each tabs as tab}
        <div 
          class="px-4 py-2 {activeTabId === tab.id ? 'bg-slate-900' : ''} border-r border-slate-300 font-medium flex items-center whitespace-nowrap"
          on:click={() => switchTab(tab.id)}
        >
          <span>{tab.title}</span>
          <button 
            class="ml-2 text-slate-400 hover:text-slate-600"
            on:click|stopPropagation={() => closeTab(tab.id)}
          >
            <X size={14} />
          </button>
        </div>
      {/each}
      <button class="px-3 py-2 text-slate-600 hover:bg-slate-300" on:click={addNewTab}>
        <Plus size={16} />
      </button>
    </div>

    <!-- Main Grid Area -->
    <div class="flex-1 overflow-auto bg-slate-400">
      {#if activeTabId === "overview"}
        <!-- Team Sections -->
        {#each Object.keys(employees) as team}
          <div class="border-b border-slate-200">
            <div 
              class="flex items-center p-2 bg-slate-600 cursor-pointer hover:bg-slate-900"
              on:click={() => toggleTeam(team)}
            >
              {#if expandedTeams[team]}
                <ChevronDown size={16} class="mr-2 text-slate-500" />
              {:else}
                <ChevronRight size={16} class="mr-2 text-slate-500" />
              {/if}
              <h2 class="font-bold">{team}</h2>
              <span class="ml-2 text-xs text-slate-500">({employees[team].length} {employees[team].length === 1 ? 'member' : 'members'})</span>
              <div class="ml-auto flex items-center space-x-2">
                <button class="p-1 hover:bg-slate-500 rounded">
                  <Plus size={14} />
                </button>
                <button class="p-1 hover:bg-slate-500 rounded">
                  <Settings size={14} />
                </button>
              </div>
            </div>

            {#if expandedTeams[team]}
              <!-- Table Header -->
              <div class="grid grid-cols-12 gap-1 px-2 py-1 bg-slate-200 text-xs font-bold text-slate-600 border-y border-slate-300">
                <div class="col-span-2 flex items-center">Employee</div>
                <div class="col-span-2">Current Task</div>
                <div class="col-span-1">Mood</div>
                <div class="col-span-1">Energy</div>
                <div class="col-span-1">Stress</div>
                <div class="col-span-1">Coding</div>
                <div class="col-span-1">Design</div>
                <div class="col-span-1">Testing</div>
                <div class="col-span-1">Teamwork</div>
                <div class="col-span-1">Actions</div>
              </div>

              <!-- Employee Rows -->
              <div class="divide-y divide-slate-100">
                {#each employees[team] as employee}
                  <div 
                    class=" bg-slate-50 grid grid-cols-12 gap-1 px-2 py-2 hover:bg-blue-400 cursor-pointer {employee.warning ? 'bg-amber-300' : ''}"
                    on:click={() => openEmployeeTab(employee)}
                  >
                    <div class="col-span-2 flex items-center">
                      <div class="w-6 h-6 rounded-full bg-blue-500 flex items-center justify-center text-slate-800 mr-2">
                        {employee.initials}
                      </div>
                      <div class="flex items-center">
                        <div>
                          <div class="text-slate-800 font-medium">{employee.name}</div>
                          <div class="text-slate-600 text-xs text-slate-500">{employee.role}</div>
                        </div>
                        {#if employee.warning}
                          <AlertCircle size={14} class="ml-2 text-amber-500" />
                        {/if}
                      </div>
                    </div>
                    <div class="col-span-2 flex items-center">
                      <div class="w-2 h-2 rounded-full bg-{employee.taskStatus}-500 mr-2"></div>
                      <div>
                        <div class="text-slate-800  text-xs">{employee.task}</div>
                        <div class="text-xs text-slate-500">{employee.project}</div>
                      </div>
                    </div>
                    <div class="col-span-1 flex items-center">
                      <Smile size={14} class="text-{employee.mood > 70 ? 'green' : employee.mood > 40 ? 'amber' : 'red'}-500 mr-1" />
                      <div class="w-full bg-slate-200 rounded-full h-2">
                        <div class="bg-{employee.mood > 70 ? 'green' : employee.mood > 40 ? 'amber' : 'red'}-500 h-2 rounded-full" style="width: {employee.mood}%"></div>
                      </div>
                    </div>
                    <div class="col-span-1 flex items-center">
                      <Coffee size={14} class="text-{employee.energy > 70 ? 'green' : employee.energy > 40 ? 'amber' : 'red'}-500 mr-1" />
                      <div class="w-full bg-slate-200 rounded-full h-2">
                        <div class="bg-{employee.energy > 70 ? 'green' : employee.energy > 40 ? 'amber' : 'red'}-500 h-2 rounded-full" style="width: {employee.energy}%"></div>
                      </div>
                    </div>
                    <div class="col-span-1 flex items-center">
                      <Activity size={14} class="text-{employee.stress < 30 ? 'green' : employee.stress < 70 ? 'amber' : 'red'}-500 mr-1" />
                      <div class="w-full bg-slate-200 rounded-full h-2">
                        <div class="bg-{employee.stress < 30 ? 'green' : employee.stress < 70 ? 'purple' : 'red'}-500 h-2 rounded-full" style="width: {employee.stress}%"></div>
                      </div>
                    </div>
                    <div class="text-slate-800 col-span-1 text-center">{employee.skills.coding}</div>
                    <div class="text-slate-800 col-span-1 text-center">{employee.skills.design}</div>
                    <div class="text-slate-800 col-span-1 text-center">{employee.skills.testing}</div>
                    <div class="text-slate-800 col-span-1 text-center">{employee.skills.teamwork}</div>
                    <div class="text-slate-800 col-span-1 flex items-center justify-center space-x-1">
                      <button class="p-1 hover:bg-slate-200 rounded">
                        <MessageSquare size={14} />
                      </button>
                      <button class="p-1 hover:bg-{employee.warning ? 'amber' : 'slate'}-200 rounded {employee.warning ? 'text-amber-700' : ''}">
                        <Info size={14} />
                      </button>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {/each}
      {:else if activeTabId === "reports"}
        <div class="p-6">
          <h2 class="text-xl font-bold mb-4">Reports Dashboard</h2>
          <div class="grid grid-cols-2 gap-6">
            <div class="border border-slate-200 rounded-lg p-4 bg-white shadow-sm">
              <h3 class="font-bold mb-2">Team Performance</h3>
              <div class="h-40 bg-slate-100 rounded flex items-center justify-center">
                <span class="text-slate-500">Performance Chart Placeholder</span>
              </div>
              <div class="mt-4 text-xs text-slate-600">
                <p>Development Team: 87% efficiency</p>
                <p>QA Team: 92% efficiency</p>
                <p>Business Analyst Team: 78% efficiency</p>
                <p>Management Team: 85% efficiency</p>
              </div>
            </div>
            <div class="border border-slate-200 rounded-lg p-4 bg-white shadow-sm">
              <h3 class="font-bold mb-2">Project Status</h3>
              <div class="h-40 bg-slate-100 rounded flex items-center justify-center">
                <span class="text-slate-500">Project Timeline Placeholder</span>
              </div>
              <div class="mt-4 text-xs text-slate-600">
                <p>Project Alpha: 65% complete (on schedule)</p>
                <p>Project Beta: 42% complete (2 days behind)</p>
                <p>Project Gamma: 90% complete (ahead of schedule)</p>
                <p>Project Delta: 15% complete (planning phase)</p>
              </div>
            </div>
            <div class="border border-slate-200 rounded-lg p-4 bg-white shadow-sm">
              <h3 class="font-bold mb-2">Resource Allocation</h3>
              <div class="h-40 bg-slate-100 rounded flex items-center justify-center">
                <span class="text-slate-500">Resource Chart Placeholder</span>
              </div>
              <div class="mt-4 text-xs text-slate-600">
                <p>Development: 65% allocated</p>
                <p>QA: 80% allocated</p>
                <p>Business Analysis: 45% allocated</p>
                <p>Management: 90% allocated</p>
              </div>
            </div>
            <div class="border border-slate-200 rounded-lg p-4 bg-white shadow-sm">
              <h3 class="font-bold mb-2">Team Wellness</h3>
              <div class="h-40 bg-slate-100 rounded flex items-center justify-center">
                <span class="text-slate-500">Wellness Chart Placeholder</span>
              </div>
              <div class="mt-4 text-xs text-slate-600">
                <p>Average Mood: 72% (Good)</p>
                <p>Average Energy: 58% (Moderate)</p>
                <p>Average Stress: 45% (Moderate)</p>
                <p>Warning: 1 employee requires attention</p>
              </div>
            </div>
          </div>
        </div>
      {:else}
        <!-- Employee Detail Tab -->
        {#if tabs.find(tab => tab.id === activeTabId)?.type === "employee"}
          {@const currentTab = tabs.find(tab => tab.id === activeTabId)}
          {@const employee = currentTab.employee}
          
          <div class="p-6">
            <div class="flex items-start justify-between mb-6">
              <div class="flex items-center">
                <div class="w-12 h-12 rounded-full bg-blue-500 flex items-center justify-center text-white mr-4 text-lg">
                  {employee.initials}
                </div>
                <div>
                  <h2 class="text-2xl font-bold">{employee.name}</h2>
                  <p class="text-slate-600">{employee.role}</p>
                </div>
              </div>
              {#if employee.warning}
                <div class="bg-amber-100 text-amber-800 px-3 py-1 rounded-full text-xs font-medium flex items-center">
                  <AlertCircle size={12} class="mr-1" />
                  Requires Attention
                </div>
              {/if}
            </div>
            
            <div class="grid grid-cols-3 gap-6">
              <div class="col-span-2">
                <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">
                  <h3 class="text-lg font-bold mb-4">Overview</h3>
                  
                  <div class="grid grid-cols-2 gap-6">
                    <div>
                      <h4 class="font-bold mb-2 text-slate-700">Contact Information</h4>
                      <div class="space-y-2 text-sm">
                        <p><span class="text-slate-500">Email:</span> {employee.details?.email || 'N/A'}</p>
                        <p><span class="text-slate-500">Phone:</span> {employee.details?.phone || 'N/A'}</p>
                        <p><span class="text-slate-500">Hire Date:</span> {employee.details?.hireDate || 'N/A'}</p>
                        <p><span class="text-slate-500">Salary:</span> {employee.details?.salary || 'N/A'}</p>
                      </div>
                    </div>
                    
                    <div>
                      <h4 class="font-bold mb-2 text-slate-700">Current Assignment</h4>
                      <div class="space-y-2 text-sm">
                        <p><span class="text-slate-500">Task:</span> {employee.task}</p>
                        <p><span class="text-slate-500">Project:</span> {employee.project}</p>
                        <p><span class="text-slate-500">Status:</span> <span class="text-{employee.taskStatus}-600 font-medium">{employee.taskStatus === 'green' ? 'On Track' : employee.taskStatus === 'amber' ? 'At Risk' : employee.taskStatus === 'blue' ? 'In Progress' : 'Delayed'}</span></p>
                      </div>
                    </div>
                  </div>
                  
                  {#if employee.details?.notes}
                    <div class="mt-6 p-3 bg-{employee.warning ? 'amber' : 'blue'}-50 border border-{employee.warning ? 'amber' : 'blue'}-200 rounded">
                      <h4 class="font-bold mb-1 text-{employee.warning ? 'amber' : 'blue'}-800">Manager Notes</h4>
                      <p class="text-sm text-{employee.warning ? 'amber' : 'blue'}-800">{employee.details.notes}</p>
                    </div>
                  {/if}
                </div>
                
                <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm">
                  <h3 class="text-lg font-bold mb-4">Performance History</h3>
                  
                  <div class="h-48 bg-slate-100 rounded flex items-center justify-center mb-4">
                    <span class="text-slate-500">Performance Chart Placeholder</span>
                  </div>
                  
                  <div class="grid grid-cols-3 gap-4 text-center">
                    <div class="p-3 bg-slate-50 rounded border border-slate-200">
                      <div class="text-xs text-slate-500">Tasks Completed</div>
                      <div class="text-xl font-bold mt-1">42</div>
                    </div>
                    <div class="p-3 bg-slate-50 rounded border border-slate-200">
                      <div class="text-xs text-slate-500">Avg. Completion Time</div>
                      <div class="text-xl font-bold mt-1">2.3 days</div>
                    </div>
                    <div class="p-3 bg-slate-50 rounded border border-slate-200">
                      <div class="text-xs text-slate-500">Quality Score</div>
                      <div class="text-xl font-bold mt-1">87%</div>
                    </div>
                  </div>
                </div>
              </div>
              
              <div class="col-span-1">
                <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm mb-6">
                  <h3 class="text-lg font-bold mb-4">Skills & Attributes</h3>
                  
                  <div class="space-y-4">
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span>Coding</span>
                        <span>{employee.skills.coding}/100</span>
                      </div>
                      <div class="w-full bg-slate-200 rounded-full h-2">
                        <div class="bg-blue-500 h-2 rounded-full" style="width: {employee.skills.coding}%"></div>
                      </div>
                    </div>
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span>Design</span>
                        <span>{employee.skills.design}/100</span>
                      </div>
                      <div class="w-full bg-slate-200 rounded-full h-2">
                        <div class="bg-pink-500 h-2 rounded-full" style="width: {employee.skills.design}%"></div>
                      </div>
                    </div>
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span>Testing</span>
                        <span>{employee.skills.testing}/100</span>
                      </div>
                      <div class="w-full bg-slate-200 rounded-full h-2">
                        <div class="bg-green-500 h-2 rounded-full" style="width: {employee.skills.testing}%"></div>
                      </div>
                    </div>
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span>Teamwork</span>
                        <span>{employee.skills.teamwork}/100</span>
                      </div>
                      <div class="w-full bg-slate-200 rounded-full h-2">
                        <div class="bg-purple-500 h-2 rounded-full" style="width: {employee.skills.teamwork}%"></div>
                      </div>
                    </div>
                  </div>
                  
                  <h4 class="font-bold mb-2 mt-6 text-slate-700">Wellness</h4>
                  <div class="space-y-4">
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span>Mood</span>
                        <span>{employee.mood}%</span>
                      </div>
                      <div class="w-full bg-slate-200 rounded-full h-2">
                        <div class="bg-{employee.mood > 70 ? 'green' : employee.mood > 40 ? 'amber' : 'red'}-500 h-2 rounded-full" style="width: {employee.mood}%"></div>
                      </div>
                    </div>
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span>Energy</span>
                        <span>{employee.energy}%</span>
                      </div>
                      <div class="w-full bg-slate-200 rounded-full h-2">
                        <div class="bg-{employee.energy > 70 ? 'green' : employee.energy > 40 ? 'amber' : 'red'}-500 h-2 rounded-full" style="width: {employee.energy}%"></div>
                      </div>
                    </div>
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span>Stress</span>
                        <span>{employee.stress}%</span>
                      </div>
                      <div class="w-full bg-slate-200 rounded-full h-2">
                        <div class="bg-{employee.stress < 30 ? 'green' : employee.stress < 70 ? 'purple' : 'red'}-500 h-2 rounded-full" style="width: {employee.stress}%"></div>
                      </div>
                    </div>
                  </div>
                </div>
                
                <div class="border border-slate-200 rounded-lg p-6 bg-white shadow-sm">
                  <h3 class="text-lg font-bold mb-4">Projects</h3>
                  
                  {#if employee.details?.projects}
                    <div class="space-y-2">
                      {#each employee.details.projects as project}
                        <div class="p-3 bg-slate-50 rounded border border-slate-200">
                          <div class="font-medium">{project}</div>
                          <div class="text-xs text-slate-500 mt-1">Active</div>
                        </div>
                      {/each}
                    </div>
                  {:else}
                    <p class="text-slate-500 text-sm">No projects assigned.</p>
                  {/if}
                  
                  <div class="mt-6 flex justify-end space-x-2">
                    <button class="px-3 py-1.5 bg-slate-200 hover:bg-slate-300 rounded text-sm">Schedule Meeting</button>
                    <button class="px-3 py-1.5 bg-blue-500 hover:bg-blue-600 text-white rounded text-sm">Assign Task</button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        {:else}
          <!-- Empty tab or custom tab content -->
          <div class="p-6">
            <h2 class="text-xl font-bold mb-4">New Tab</h2>
            <p class="text-slate-600">This is a new tab. You can customize its content.</p>
          </div>
        {/if}
      {/if}
    </div>

    <!-- Status Bar -->
    <div class="bg-slate-800 text-slate-300 text-xs p-1 flex items-center justify-between border-t border-slate-700">
      <div class="flex items-center space-x-4">
        <div class="flex items-center space-x-1">
          <div class="w-2 h-2 bg-green-500 rounded-full"></div>
          <span>3 Projects Active</span>
        </div>
        <div class="flex items-center space-x-1">
          <div class="w-2 h-2 bg-amber-500 rounded-full"></div>
          <span>2 Deadlines Approaching</span>
        </div>
        <div class="flex items-center space-x-1">
          <div class="w-2 h-2 bg-red-500 rounded-full"></div>
          <span>1 Critical Issue</span>
        </div>
      </div>
      <div class="flex items-center space-x-4">
        <div>Tick #{$tick}/{isPlaying ? '4,231' : '4,230'}</div>
        <div>v1.0.42</div>
      </div>
    </div>
  </div>
</div>