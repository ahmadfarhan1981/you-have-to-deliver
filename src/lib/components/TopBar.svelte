<script>
    import {Bell, Calendar, Clock, CreditCard, User, Users} from "lucide-svelte";
    import {onMount} from "svelte";
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