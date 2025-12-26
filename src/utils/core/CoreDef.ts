import {XraySchema} from "@/utils/core/configurator/schema/xray.schema.ts";
import {createConfigurationSchema} from "@/utils/core/configurator/schema/schema.ts";
import {V2flySchema} from "@/utils/core/configurator/schema/v2fly.schema.ts";

export const CoreTypes = [
     createConfigurationSchema("xray", XraySchema),
     createConfigurationSchema("v2fly", V2flySchema),
]