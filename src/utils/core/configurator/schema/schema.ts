import {ZodObject} from "zod";

export class ConfigurationSchema {
    constructor(
        public readonly name: string,
        public readonly schema: ZodObject<any>
    ) {
    }
}