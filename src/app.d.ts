declare global {
	namespace App {
		// interface Error {}
		interface Locals {}
		interface PageData {}
		// interface PageState {}
		// interface Platform {}
		interface Array<T> {
			shuffle(): Array<T>;
			draw(n: number): Array<T>;
			dredge(n: number): Array<T>;
		}
	}
}

export {};
