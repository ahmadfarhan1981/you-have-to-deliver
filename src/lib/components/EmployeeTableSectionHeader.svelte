<script lang="ts">



    // function toggleDirection() {
    //     unassignedSort.update(s => ({
    //         ...s,
    //         direction: s.direction === 'asc' ? 'desc' : 'asc'
    //     }));
    // }
    //
    // function setSortKey(key) {
    //     unassignedSort.update(s => ({
    //         key,
    //         direction: s.key === key
    //             ? (s.direction === 'asc' ? 'desc' : 'asc') // toggle if same key
    //             : 'asc' // default to ascending if new key
    //     }));
    // }
</script>
<div class="grid grid-cols-12 gap-1 px-2 py-1 bg-slate-200 text-xs font-bold text-slate-600 border-y border-slate-300">
    <div class="col-span-2 flex items-center" >Employee</div>
    <div class="col-span-2">Current Task</div>
    <div class="col-span-1">Cognition</div>
    <div class="col-span-1">Perception</div>
    <div class="col-span-1">Drive</div>
    <div class="col-span-1">Social</div>
    <div class="col-span-1">Defense</div>
    <div class="col-span-1">Needs</div>
    <div class="col-span-1" >Total stats</div>
    <div class="col-span-1">Actions</div>
</div>