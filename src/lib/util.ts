import { browser } from '$app/environment';

export const enterFullscreen = (divID: string) => {
	if (!browser) return;
	const elem = document.getElementById(divID);
	if (elem?.requestFullscreen) {
		elem.requestFullscreen();
	}
};

export const getRandomDeckName = () => {
	const names = [
		"Pixie's Rejected Tarot",
		'Rider-Waite Smackdown',
		'Moonlight Malpractice',
		'DMT Elf encounter',
		'Demonology for Dummies'
	];

	return names[Math.floor(Math.random() * names.length)];
};

export const colorMap = {
	white: {
		bg: 'white-bg',
		title: 'white',
		desc: 'white-desc',
		color: '#e9e9e9',
		iconColor: '#434343',
		icon: 'manaWhite'
	},
	black: {
		bg: 'black-bg',
		title: 'black',
		desc: 'black-desc',
		color: '#434343',
		iconColor: '#f3f3f3',
		icon: 'manaBlack'
	}
} as const;
