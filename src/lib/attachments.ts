// moves the element to document.body, escaping any parent stacking context
export const portal = (node: HTMLElement) => {
	document.body.appendChild(node)
	return () => node.remove()
}