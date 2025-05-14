<script>

    export let tick;
</script>
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
        <div>Tick #{$tick}/4,230</div>
        <div>v1.0.42</div>
    </div>
</div>