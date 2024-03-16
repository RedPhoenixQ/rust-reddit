import { addGlobalStylesToShadowRoot } from "../js/global-styles.mjs";

const { div, button } = van.tags;

vanE.define("custom-gallery", ({ attr, mount, children, $this }) => {
	addGlobalStylesToShadowRoot($this.shadowRoot);

	/**@param {-1 | 1} to direction */
	function scroll(to) {
		const current_index = Math.round(gallery.scrollLeft / gallery.offsetWidth);
		const next = current_index + to;
		if (next < 0 || next >= $this.childElementCount) return;
		gallery.scroll({
			behavior: "smooth",
			left: gallery.offsetWidth * next,
		});
	}

	$this.style.position = "relative";

	mount(() => {
		for (const child of $this.children) {
			child.style["scroll-snap-align"] = "center";
		}
	});

	const gallery = div(
		{
			style: "position: relative; display: flex; scroll-snap-type: x mandatory; overflow-y: auto; width: 100%;",
		},
		children
	);
	return [
		gallery,
		button(
			{
				className: "size-10 absolute left-2 top-1/2 rounded-full bg-black/50",
				onclick: () => scroll(-1),
			},
			"<"
		),
		button(
			{
				className: "size-10 absolute right-2 top-1/2 rounded-full bg-black/50",
				onclick: () => scroll(1),
			},
			">"
		),
	];
});
