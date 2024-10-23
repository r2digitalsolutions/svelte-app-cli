<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolveRoute } from "$app/paths";
  import { page } from "$app/stores";
  import TreeContent from "$lib/components/TreeContent.svelte";
  import { Container, Button } from "@r2digisolutions/svelte-ui";
  import { invoke } from "@tauri-apps/api/core";
  import { ArrowLeftCircleIcon, Save, CirclePlus } from "lucide-svelte";

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
        project: data.project_info.id as string,
        locale: data.locale,
      }),
    );
  };
</script>

<Container>
  <div class="p-4">
    <Button
      class="mb-4"
      href={resolveRoute("/projects/[project]/locales/[locale]/[file]/new", {
        project: $page.params.project as string,
        locale: data.locale,
        file: $page.params.file as string,
      })}
    >
      <CirclePlus />
      New translation
    </Button>
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
