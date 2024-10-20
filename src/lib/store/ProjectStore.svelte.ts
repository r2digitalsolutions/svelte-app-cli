import type { IProject, TProjects } from "../types/project";
import { ProjectItemStore } from "./ProjectItemStore.svelte";

export class ProjectStore {
  projects: IProject[] = $state([]);

  init(data: TProjects[]) {
    this.projects = data.map(ProjectItemStore.fromJSON);
  }

  getProject(project_id: number): IProject | undefined {
    return this.projects.find((project) => project.id === project_id);
  }
}