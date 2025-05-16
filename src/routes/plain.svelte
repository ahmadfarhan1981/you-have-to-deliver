<script lang="ts">
    import EmployeeList from '$lib/components/plain/EmployeeList.svelte';
    import EmployeeDetail from '$lib/components/plain/EmployeeDetail.svelte';
    import Sidebar from "$lib/components/Sidebar.svelte";
    import {tick} from "$lib/stores/tick";
    import StatusBar from "$lib/components/StatusBar.svelte";


    let activeView = "Dashboard";

    // Navigation
    function navigateTo(view) {
        activeView = view;
        // In a real app, this would change the content displayed
    }

</script>


    <!-- Sidebar -->

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <EmployeeList/>
        <EmployeeDetail/>
    </div>
