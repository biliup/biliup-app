<script lang="ts">
    import Sidebar from './Sidebar.svelte';
    import Upload from './Upload.svelte';
    import { fade } from 'svelte/transition';

</script>

<div class="flex items-start">
    <Sidebar let:current>
            <Upload selected={current}/>
    </Sidebar>
</div>

