import Database from "@tauri-apps/plugin-sql";
import type { LayoutLoad } from "./$types";

export const prerender = true
export const ssr = false

export const load = (async ({ fetch }) => {
  const db = await Database.load("sqlite:mydatabase.db");

  const projects = await db.select(
    "SELECT * FROM projects"
  );

  return {
    projects
  }
}) satisfies LayoutLoad;