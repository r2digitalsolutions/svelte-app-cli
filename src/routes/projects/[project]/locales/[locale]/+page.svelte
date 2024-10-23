<script lang="ts">
  import { resolveRoute } from "$app/paths";
  import { Button, Container } from "@r2digisolutions/svelte-ui";
  import { PlusCircle } from "lucide-svelte";

  let { data } = $props();
</script>

<Container class="p-4">
  <div class="flex flex-col gap-2">
    <Button
      href={resolveRoute("/projects/[project]/locales/[locale]/new", {
        project: data.project,
        locale: data.locale,
      })}
    >
      <PlusCircle />
      New file
    </Button>
    {#each data.files ?? [] as file}
      <a
        class="bg-gray-100 dark:bg-gray-800 p-2 rounded-lg shadow-md hover:bg-gray-200 hover:dark:bg-gray-950"
        href={resolveRoute("/projects/[project]/locales/[locale]/[file]", {
          project: data.project,
          locale: data.locale,
          file,
        })}
      >
        {file}
      </a>
    {/each}
  </div>
</Container>
