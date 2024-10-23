<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import Card from "$lib/components/Card.svelte";
  import {
    Button,
    Container,
    Input,
    FormGroup,
  } from "@r2digisolutions/svelte-ui";
  import { invoke } from "@tauri-apps/api/core";
  import { Save } from "lucide-svelte";

  let loading = $state(false);
  let { data } = $props();

  const onSave = async (e) => {
    const form = new FormData(e.target);
    const name = form.get("name") as string;

    if (!name) return;

    loading = true;

    const rs = await invoke("create_key_json_file", {
      project: data.project_info.path,
      locale: $page.params.locale,
      file: $page.params.file,
      name: name,
    });

    loading = false;

    if (!rs) {
      return;
    }

    goto("/");
  };
</script>

<Container class="p-4">
  <Card title="New file" class="max-w-lg mx-auto">
    <form onsubmit={onSave} class="flex flex-col gap-4 items-start">
      <FormGroup label="Key" name="name">
        <Input name="name" placeholder="New key" />
        <span class="text-xs text-gray-500 dark:text-gray-400">
          Separate keys with a point (.)</span
        >
      </FormGroup>
      <Button type="submit" {loading}>
        <Save />
        Create
      </Button>
    </form>
  </Card>
</Container>
