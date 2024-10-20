<script lang="ts">
  import { project_store } from "$lib/store";
  import { Button, HeaderSection } from "@r2digisolutions/svelte-ui";
  import { ExternalLink, ArrowLeftCircleIcon } from "lucide-svelte";
  import { page } from "$app/stores";

  const { data, children } = $props();
  const project = $derived(project_store.getProject(Number(data.project.id)));

  const actions = {
    openDirectory: () => project?.openDirectory(),
  };

  $inspect($page.route.id);
</script>

<HeaderSection
  class="bg-gray-100 dark:!bg-gray-800 p-4"
  title="Project - ({data.project.name})"
>
  <div class="flex flex-row gap-2 items-center justify-center mt-4">
    {#if ($page.route.id ?? "") != "/projects/[project]"}
      <Button theme="secondary" onclick={() => window.history.back()}>
        <ArrowLeftCircleIcon />
        Back
      </Button>
    {/if}
    <Button onclick={() => actions.openDirectory()}>
      <ExternalLink />
      Open Project
    </Button>
  </div>
</HeaderSection>

{@render children()}
