// moves the element to game-frame, escaping any parent stacking context
export const portal = (node: HTMLElement) => {
	const gameFrame = document.getElementById("game-frame");
	gameFrame?.appendChild(node)
	return () => node.remove()
}