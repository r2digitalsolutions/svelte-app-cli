<script lang="ts">
  import { type TProjects } from "$lib/types/project";
  import { project_store } from "../store";
</script>

{#snippet Item(project: TProjects, home=false)}
  <a
    class="text-gray-700 dark:text-gray-200 bg-gray-100 dark:bg-gray-900 w-full hover:bg-gray-200 dark:hover:bg-gray-800 p-2 rounded-md"
    href="{home? '/': "/projects/"+project.id}"
  >
    {project.name}
  </a>
{/snippet}

<aside
  class="sidebar hidden sm:flex flex-col w-full sm:col-span-2 bg-gray-100 dark:bg-gray-950 p-4"
>
  <h1 class="mb-4 text-2xl font-bold">Projects</h1>

  <nav class="flex flex-col w-full gap-2">
    {@render Item({
      name:"Home",
      id:"/",
      path:"/",
      directory_name:"home"
    }, true)}
    {#each project_store.projects as project}
      {@render Item(project)}
    {:else}
      <a href="/new">New project</a>
    {/each}
  </nav>
</aside>
