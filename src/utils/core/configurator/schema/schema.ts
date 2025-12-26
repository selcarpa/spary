import {ZodObject} from "zod";
import { markRaw } from 'vue';

// Use a simple plain object type to avoid Proxy issues
export type ConfigurationSchema = {
    name: string;
    schema: ZodObject<any>;
};

export function createConfigurationSchema(name: string, schema: ZodObject<any>): ConfigurationSchema {
    // Use markRaw to mark the schema object to prevent Vue from converting it to reactive, thus avoiding Proxy issues
    return { name, schema: markRaw(schema) };
}