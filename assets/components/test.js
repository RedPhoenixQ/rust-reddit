import { addGlobalStylesToShadowRoot } from "../js/global-styles.mjs";

const { div, input } = van.tags;

vanE.define("test-element", ({ attr, $this }) => {
	addGlobalStylesToShadowRoot($this.shadowRoot);
	const inp = van.state("yeet");
	return div(
		input({
			value: inp.val,
			onchange: (e) => (inp.val = e.target.value),
		}),
		div({ class: "bg-red-500 p-2" }, inp),
		() => `Hello ${attr("test").val}`
	);
});
