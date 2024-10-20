<script lang="ts">
  import { fnClassNames, type TProps } from "@r2digisolutions/svelte-ui";
  import type { Snippet } from "svelte";

  let {
    title,
    children,
    class: className,
    actions,
  }: TProps & {
    title: string;
    children: Snippet;
    actions?: Snippet;
  } = $props();
</script>

<article
  class={fnClassNames(
    "bg-white dark:bg-gray-700 shadow-md rounded-lg w-full",
    className,
  )}
>
  <header
    class="flex justify-between items-center p-4 bg-gray-100 dark:bg-gray-800 rounded-t-lg h-16"
  >
    <h3>{title}</h3>
    {#if actions}
      <div class="flex gap-1 items-center">
        {@render actions?.()}
      </div>
    {/if}
  </header>

  <section class="p-4 max-h-96 overflow-y-auto">
    {@render children()}
  </section>
</article>
