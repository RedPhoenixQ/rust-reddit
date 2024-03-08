import type { Van } from "./van-1.3.0.min";
import * as VanEType from "./van-element.browser";
import * as VanXType from "./van-x.nomodule.min";

declare global {
	const van: Van;
	const vanE: typeof VanEType;
	const vanX: typeof VanXType;
}
