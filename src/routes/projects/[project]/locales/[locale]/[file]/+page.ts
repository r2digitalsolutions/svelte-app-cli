import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { invoke } from "@tauri-apps/api/core";

export const load = (async ({ params, parent }) => {
  const data_project = await parent();
  const { project, locale, file } = params;

  if (!project || !locale || !file) {
    return error(404, "Project not found");
  }

  const data = await invoke("get_json_file", {
    project: data_project.project_info.path,
    locale: locale,
    file: file,
  });

  if (!data) {
    return error(404, "File not found");
  }

  return {
    project,
    locale,
    file,
    data,
  };
}) satisfies PageLoad;