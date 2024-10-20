<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import { project_store } from "$lib/store";
  import { Button, FormGroup, Input } from "@r2digisolutions/svelte-ui";
  import { Save } from "lucide-svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";

  let loading = $state(false);

  const project = $derived(
    project_store.getProject(Number($page.params.project)),
  );

  const initialSave = async (locale: string) => {
    loading = true;
    const locales = await project?.getLocales();

    if (locales?.includes(locale)) {
      return;
    }

    if (!project) {
      return;
    }

    const rs = await project.createLocale(locale);

    if (!rs) {
      return;
    }

    goto(`/projects/${project.id}`);
  };

  const onSubmit = async (e) => {
    e.preventDefault();
    const form = new FormData(e.target);
    const locale = form.get("locale") as string;

    if (!locale) return;

    initialSave(locale).finally(() => {
      loading = false;
    });
  };
</script>

<div class="p-10 flex flex-col gap-4">
  <div class="mx-auto max-w-lg w-full">
    <Card title="New locale">
      <form onsubmit={onSubmit} class="flex flex-col gap-4 items-start">
        <FormGroup label="Locale" name="locale">
          <Input placeholder="Locale" name="locale" />
        </FormGroup>
        <Button type="submit" {loading}>
          <Save />
          Create
        </Button>
      </form>
    </Card>
  </div>
</div>
