<script lang="ts">
  import "../app.css";
  import Sidebar from "../lib/components/Sidebar.svelte";
  import { project_store } from "../lib/store";
  
  let { data, children } = $props();


  const initializing = (data: any) => {
   project_store.init(data.projects || []);
  }

initializing(data);

$effect(() => {
  initializing(data);
});
</script>

<div class="grid grid-cols-12 h-screen">
  <Sidebar />
  <main class="col-span-full sm:col-span-10 flex flex-col gap-1">
    {@render children()}
  </main>
</div>