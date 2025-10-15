import {z} from "zod";
import {getDatabase} from "@/utils/db.ts";
import {DBDefaultDateTime} from "@/utils/zodCommon.ts";

export const NodeSchema = z.object({
    id: z.number().nullable(),
    alias: z.string(),
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
    group_id: z.number(),
});
export type Node = z.infer<typeof NodeSchema>;

export class NodeRepository {
    async findAll(): Promise<Node[]> {
        const db = await getDatabase();
        const rows = await db.select(`SELECT *
                                      FROM "node"
                                      ORDER BY id DESC`);
        return NodeSchema.array().parse(rows);
    }

    async insert(node: Node): Promise<void> {
        const db = await getDatabase();
        await db.execute(`INSERT INTO "node" (alias, arguments, group_id)
                          VALUES (?, ?, ?)`, [
            node.alias,
            JSON.stringify(node.arguments),
            node.group_id
        ]);
    }

    async findByAlias(alias: string): Promise<Node[]> {
        const db = await getDatabase();
        const rows = await db.select(`SELECT *
                                      FROM "node"
                                      WHERE alias = ?`, [alias]);
        return NodeSchema.array().parse(rows);
    }

}

export const nodeRepository = new NodeRepository();
