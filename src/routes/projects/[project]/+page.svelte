<script lang="ts">
  import { resolveRoute } from "$app/paths";
  import Card from "$lib/components/Card.svelte";
  import { project_store } from "$lib/store/index.js";
  import {
    Container,
    IconLoading,
    ButtonIcon,
  } from "@r2digisolutions/svelte-ui";
  import { RefreshCw, PlusCircle } from "lucide-svelte";
  const { data } = $props();

  const project = $derived(project_store.getProject(Number(data.project.id)));
</script>

<Container class="p-4">
  <div class="bg-gray-100 dark:!bg-gray-800 p-4 rounded-lg shadow-md">
    <p>Project: {data.project.name}</p>
    <p>Version: {data.project_info.version}</p>
    <p>Path: {data.project.path}</p>
    <p>Directory name: {data.project.directory_name}</p>
  </div>

  <div class="grid grid-cols-12 gap-2 mt-4">
    <Card title="Locales" class="col-span-6">
      {#snippet actions()}
        <ButtonIcon
          href={resolveRoute("/projects/[project]/locales/new", {
            project: data.project.id.toString(),
          })}
        >
          <PlusCircle />
        </ButtonIcon>
        <ButtonIcon>
          <RefreshCw />
        </ButtonIcon>
      {/snippet}
      <div class="flex flex-col gap-2">
        {#await project?.getLocales()}
          <IconLoading />
        {:then locales}
          {#each locales || [] as locale}
            <a
              class="bg-gray-100 dark:bg-gray-800 p-2 rounded-lg shadow-md hover:bg-gray-200 hover:dark:bg-gray-900"
              href="/projects/{data.project.id}/locales/{locale}">{locale}</a
            >
          {/each}
        {/await}
      </div>
    </Card>
    <Card title="Dependencies" class="col-span-6">
      {#snippet actions()}
        <ButtonIcon>
          <PlusCircle />
        </ButtonIcon>
        <ButtonIcon>
          <RefreshCw />
        </ButtonIcon>
      {/snippet}
      <div class="flex flex-col gap-2">
        {#await project?.getDependencies()}
          <IconLoading />
        {:then dependencies}
          {#each dependencies?.toSorted( (a, b) => a.name.localeCompare(b.name), ) || [] as dependency}
            <a
              class="bg-gray-100 dark:bg-gray-800 flex items-center justify-between p-2 rounded-lg shadow-md hover:bg-gray-200 hover:dark:bg-gray-900"
              href="/projects/{data.project.id}/dependencies/{dependency.name}"
            >
              {dependency.name}
              <span>
                [{dependency.version}]
              </span>
            </a>
          {/each}
        {/await}
      </div>
    </Card>
  </div>
</Container>
