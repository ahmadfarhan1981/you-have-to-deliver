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

  import Sidebar from "$lib/components/Sidebar.svelte"
  import StatusBar from "$lib/components/StatusBar.svelte";
  import ReportDashboard from "$lib/components/ReportDashboard.svelte";
  import EmployeeRow from "$lib/components/EmployeeRow.svelte";
  import EmployeeDetails from "$lib/components/EmployeeDetails.svelte";
  import TopBar from "$lib/components/TopBar.svelte";
</script>

<div class="flex h-screen w-full bg-slate-100 font-mono text-sm">
  <!-- Sidebar -->
  <Sidebar activeView={activeView} navigateTo={navigateTo}/>

  <!-- Main Content -->
  <div class="flex-1 flex flex-col overflow-hidden">
    <!-- Top Bar -->
    <TopBar />

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
                  <EmployeeRow employee={employee} openEmployeeTab={openEmployeeTab}/>
                {/each}
              </div>
            {/if}
          </div>
        {/each}
      {:else if activeTabId === "reports"}
        <ReportDashboard />
      {:else}
        <!-- Employee Detail Tab -->
        {#if tabs.find(tab => tab.id === activeTabId)?.type === "employee"}
          {@const currentTab = tabs.find(tab => tab.id === activeTabId)}
          {@const employee = currentTab.employee}
          
          <EmployeeDetails employee={employee} />
        {:else}
          <!-- Empty tab or custom tab content -->
          <div class="p-6">
            <h2 class="text-xl font-bold mb-4">New Tab</h2>
            <p class="text-slate-600">This is a new tab. You can customize its content.</p>
          </div>
        {/if}
      {/if}
    </div>

    <StatusBar tick={tick}/>
  </div>
</div>