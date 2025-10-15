import Database from "@tauri-apps/plugin-sql";

export async function getDatabase () {
  return await Database.load('sqlite:spary.db');
}