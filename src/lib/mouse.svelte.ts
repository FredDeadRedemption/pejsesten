type MouseState = {
	x: number;
	y: number;
};

export const mouse = $state<MouseState>({ x: 0, y: 0 });

export const updateMouse = (x: number, y: number) => {
	mouse.x = x;
	mouse.y = y;
};
