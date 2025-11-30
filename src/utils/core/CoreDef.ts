import {XraySchema} from "@/utils/core/configurator/schema/xray.schema.ts";
import {ConfigurationSchema} from "@/utils/core/configurator/schema/schema.ts";
import {V2flySchema} from "@/utils/core/configurator/schema/v2fly.schema.ts";

export const CoreTypes = [
     new ConfigurationSchema("xray", XraySchema),
     new ConfigurationSchema("v2fly", V2flySchema),
]