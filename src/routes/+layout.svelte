<script>
	import '$lib/array.extensions';
	import Nav from '$lib/components/nav.svelte';
	import '../style.scss';

	let { children } = $props();
</script>

<Nav />
{@render children()}
