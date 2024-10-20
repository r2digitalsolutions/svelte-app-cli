import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

export const load = (async ({ params, parent }) => {
  const parent_data = await parent();
  const { project, locale } = params;

  if (!project || !locale) {
    return error(404, "Project not found");
  }

  const files = await invoke("get_locales_files", { project: parent_data.project_info.path, locale }) as string[];

  if (!files) {
    return error(404, "Files not found");
  }

  return {
    project,
    locale,
    files,
  };
}) satisfies PageLoad;