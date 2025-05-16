<script lang="ts">
    export let employee
</script>

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