<script lang="ts">
	let { data } = $props();
</script>

<article class="post">
	<h1>{data.title}</h1>
	<div class="content">{@html data.content}</div>
</article>

<style lang="scss">
	.post {
		max-width: 680px;
		margin: 0 auto;
		padding: 2rem 1rem;
		color: rgb(149, 165, 185);

		h1 {
			font-size: 2rem;
			margin-bottom: 2rem;
		}

		.content {
			:global(hr) {
				margin-top: 30px;
			}
			:global(input[type='checkbox']:disabled) {
				appearance: none;
				-webkit-appearance: none;
				width: 16px;
				height: 16px;
				border: 1px solid $primary;
				border-radius: 3px;
				position: relative;
			}
			:global(input[type='checkbox']:disabled:checked) {
				background-color: $primary;
				&::after {
					content: '✓';
					position: absolute;
					color: white;
					font-size: 12px;
					top: -1px;
					left: 2px;
				}
			}
			:global(h2) {
				font-size: 1.75rem;
				margin-top: 2.5rem;
			}
			:global(p) {
				line-height: 2.125;
			}
			:global(code) {
				background: #1e1e1e;
				padding: 0.25em 0.5em;
				border-radius: 4px;
			}
			:global(pre) {
				background: #1e1e1e;
				padding: 1.25rem;
				border-radius: 8px;
				overflow-x: auto;
			}
		}
	}
</style>
