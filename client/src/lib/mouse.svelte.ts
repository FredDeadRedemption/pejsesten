export const mouse = $state({ x: 0 as number, y: 0 as number });

export const updateMouse = (x: number, y: number) => {
	mouse.x = x;
	mouse.y = y;
};
