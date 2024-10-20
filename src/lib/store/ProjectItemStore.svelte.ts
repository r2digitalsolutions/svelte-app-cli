import type { IProject, TProjectDepency, TProjects } from "$lib/types/project";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";

export class ProjectItemStore implements IProject {
  id: number;
  name: string;
  path: string;
  version: string;
  directory_name: string;

  constructor(id: number, name: string, path: string, directory_name: string, version: string) {
    this.id = id;
    this.name = name;
    this.path = path;
    this.directory_name = directory_name;
    this.version = version;
  }

  static fromJSON(data: TProjects): ProjectItemStore {
    return new ProjectItemStore(
      Number(data.id),
      data.name,
      data.path,
      data.directory_name,
      data.version,
    );
  }

  openDirectory(): void {
    open(this.path);
  }

  async getLocales(): Promise<string[]> {
    return await invoke("get_locales", { path: this.path });
  }

  async getDependencies(): Promise<TProjectDepency[]> {
    return await invoke("get_dependencies", { path: this.path });
  }

  async createLocale(locale: string): Promise<boolean> {
    return await invoke("create_locale", { path: this.path, locale: locale });
  }
}