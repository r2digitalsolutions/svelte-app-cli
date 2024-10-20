<script lang="ts">
  import { Button } from "@r2digisolutions/svelte-ui";
  import { invoke } from "@tauri-apps/api/core";
  import { ProjectService } from "../lib/services/ProjectService.svelte";

  let name = $state("");
  let greetMsg = $state("");

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }

  const project = new ProjectService();
</script>

<div class="container">
  <Button onclick={() => project.addProject()}>Open directory</Button>
</div>
