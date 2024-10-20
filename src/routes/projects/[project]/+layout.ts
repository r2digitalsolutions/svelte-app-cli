import type { LayoutLoad } from "./$types";
import { error } from "@sveltejs/kit";
import { ProjectService } from "$lib/services/ProjectService.svelte";
import { invoke } from "@tauri-apps/api/core";
import type { TProjects } from "$lib/types/project";

export const prerender = false;
export const ssr = false

export const load = (async ({ params }) => {
  if (!params.project) {
    return error(404, "Project not found");
  }

  const project_service = new ProjectService();
  const project = await project_service.getProject(Number(params.project));

  if (!project) {
    return error(404, "Project not found");
  }

  const data_project = await invoke("get_info_project", { path: project.path }) as TProjects;

  if (!data_project) {
    return error(404, "Project not found");
  }

  return {
    project: project,
    project_info: data_project,
  }
}) satisfies LayoutLoad;