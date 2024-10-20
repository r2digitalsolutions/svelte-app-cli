import { invoke } from "@tauri-apps/api/core";
import { open, message } from "@tauri-apps/plugin-dialog";
import Database from "@tauri-apps/plugin-sql";
import type { TProject, TProjects } from "../types/project";

export class ProjectService {
  loading = $state(false);

  private async getInfoProject(path: string): Promise<TProject> {
    return await invoke("get_info_project", { path });
  }

  private async existPathInDb(path: string): Promise<boolean> {
    const db = await Database.load("sqlite:mydatabase.db");

    const [test] = await db.select(
      "SELECT count(1) as total FROM projects WHERE path = ?",
      [path]
    );

    return test.total > 0;
  }

  async deleteAllProjects(): Promise<void> {
    const db = await Database.load("sqlite:mydatabase.db");
    await db.execute("DELETE FROM projects");
  }

  async addProject(): Promise<void> {
    this.loading = true;

    const dir = await open({
      directory: true,
      multiple: false,
    });

    if (!dir) return;

    await this.saveProject(dir);

    this.loading = false;
  }

  public async getProject(project_id: number): Promise<TProjects> {
    const db = await Database.load("sqlite:mydatabase.db");

    const [test] = await db.select(
      "SELECT * FROM projects where id=? ORDER BY name ASC",
      [project_id]
    );

    return test ?? null;
  }

  private async saveProject(path: string): Promise<void> {
    const { promise, reject, resolve } = await Promise.withResolvers();

    const db = await Database.load("sqlite:mydatabase.db");
    const existPathDirectory = await invoke("exist_directory", { path });

    if (!existPathDirectory) {
      message("Path not exist", {
        title: "Error",
      });
      return reject();
    }

    if (await this.existPathInDb(path)) {
      message("Path already exist", {
        title: "Error",
      });
      return reject();
    }

    const infoProject = await this.getInfoProject(path);

    if (!infoProject) {
      message("Project not exist", {
        title: "Error",
      });
      return reject();
    }

    const test = await db.execute(
      "INSERT INTO projects (name, path, directory_name) VALUES (?, ?, ?)",
      [infoProject.name, infoProject.path, infoProject.directory_name]
    );

    message("Project created successfully");
    return resolve(test);
  }
}