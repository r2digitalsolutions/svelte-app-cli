<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolveRoute } from "$app/paths";
  import TreeContent from "$lib/components/TreeContent.svelte";
  import { Container, Button } from "@r2digisolutions/svelte-ui";
  import { invoke } from "@tauri-apps/api/core";
  import { ArrowLeftCircleIcon, Save } from "lucide-svelte";

  let { data } = $props();

  let vdata = $state(data.data);

  const onSave = async () => {
    const rs = await invoke("save_json_file", {
      project: data.project_info.path,
      locale: data.locale,
      file: data.file,
      data: vdata,
    });

    if (!rs) {
      return;
    }

    goto(
      resolveRoute("/projects/[project]/locales/[locale]", {
        project: data.project,
        locale: data.locale,
      }),
    );
  };
</script>

<Container>
  <div class="p-4">
    <TreeContent data={vdata ?? {}} />
    <footer
      class="sticky bottom-0 flex justify-center mt-4 gap-2 items-center bg-gray-100 dark:bg-gray-800 p-4 rounded-lg shadow-xl border-t border-gray-200 dark:border-gray-700"
    >
      <Button theme="secondary" onclick={() => window.history.back()}>
        <ArrowLeftCircleIcon />
        Back
      </Button>
      <Button onclick={onSave}>
        <Save />
        Save
      </Button>
    </footer>
  </div>
</Container>
