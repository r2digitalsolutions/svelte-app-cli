<script lang="ts">
  import { Input } from "@r2digisolutions/svelte-ui";
  import TreeContent from "./TreeContent.svelte";

  type Props = {
    data: Record<string, any>;
  };

  let { data = $bindable({}) }: Props = $props();
</script>

<ul class="flex flex-col gap-2">
  {#each Object.entries(data) as [key, value]}
    <li class=" bg-gray-100 dark:bg-gray-800 p-2 rounded-lg shadow-md">
      <div class="flex flex-col gap-2">
        {key}
        {#if typeof value === "object"}
          <div
            class="bg-gray-100 dark:bg-gray-900 p-4 py-8 rounded-lg shadow-md"
          >
            <TreeContent data={value} />
          </div>
        {:else}
          <Input
            {value}
            onchange={(e) => {
              data[key] = e.target.value;
            }}
            placeholder="Value"
          />
        {/if}
      </div>
    </li>
  {:else}
    <li>No data</li>
  {/each}
</ul>
