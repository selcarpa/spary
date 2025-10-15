// 自定义日期时间格式 yyyy-MM-dd HH:mm:ss
import {z} from "zod";

export const DBDefaultDateTime = z
    .string()
    .refine(
        val => /^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$/.test(val),
        { message: "Invalid datetime format, expected yyyy-MM-dd HH:mm:ss" }
    );
