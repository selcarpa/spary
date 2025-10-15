import {z} from "zod";
import {getDatabase} from "@/utils/db.ts";
import {DBDefaultDateTime} from "@/utils/common.ts";

export const GroupSchema = z.object({
    id: z.number(),
    name: z.string(),
    url: z.string().nullable().optional(),
    arguments: z
        .union([z.string(), z.record(z.any(), z.any())]) // 兼容 SQLite JSON 字段可能返回 string 或 object
        .transform(val => {
            if (typeof val === "string") {
                try {
                    return JSON.parse(val);
                } catch {
                    return {};
                }
            }
            return val ?? {};
        }),
    created_at: DBDefaultDateTime,
    updated_at: DBDefaultDateTime,
});
export type Group = z.infer<typeof GroupSchema>;

export class GroupRepository {
    async findAll(): Promise<Group[]> {
        const db = await getDatabase();
        const rows = await db.select(`SELECT *
                                      FROM "group"
                                      ORDER BY id DESC`);
        console.log(
            rows
        )
        return GroupSchema.array().parse(rows);
    }

    async insert(group: Group): Promise<void> {
        const db = await getDatabase();
        await db.execute(`INSERT INTO "group" (name, url, arguments)
                          VALUES (?, ?, ?)`, [
            group.name,
            group.url,
            JSON.stringify(group.arguments),
        ]);
    }

    async findByName(name: string): Promise<Group[]> {
        const db = await getDatabase();
        const rows = await db.select(`SELECT *
                                      FROM "group"
                                      WHERE name = ?`, [name]);
        return GroupSchema.array().parse(rows);
    }
}

export const groupRepository = new GroupRepository();
