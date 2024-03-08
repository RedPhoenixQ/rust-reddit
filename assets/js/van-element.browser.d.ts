import { State, ChildDom } from "./van-1.3.0.min";

export type ElementProps = {
	/** Get the value of an attribute */
	attr: (name: string, defaultValue?: string | number) => State<string>;
	/** Registers a callback that is called when the element connects to the DOM */
	mount: (
		/** Callback when the element connects to the DOM
		 * @returns An optional dismount callback
		 */
		mount: () => (() => void) | void
	) => void;
	/** Instance of the custom element */
	$this: HTMLElement;
	children: ChildDom;
};

/**
 * Defines a VanJS custom element.
 */
export declare const define: (
	/** Name of the custom element */
	name: string,
	/** VanJS functional component */
	element: (
		/** Attributes of the custom element */
		attributes: ElementProps
	) => ChildDom,
	options?: ShadowRootInit | false
) => void;