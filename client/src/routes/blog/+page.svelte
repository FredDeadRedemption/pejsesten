<script lang="ts">
    let { data } = $props();
</script>

<div class="blog-list">
    {#each data.blogs as blog}
        <a href="/blog/{blog.slug}" class="post">
            <h2>{blog.title}</h2>
        </a>
    {/each}
</div>

<style lang="scss">
    .blog-list {
        max-width: 680px;
        margin: 0 auto;
        padding: 2rem 1rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .post {
        text-decoration: none;
        color: inherit;
        padding: 1rem;
        border-radius: 8px;
        border: 1px solid #333;
        transition: border-color 0.2s;

        &:hover {
            border-color: #666;
        }

        h2 {
            margin: 0;
            font-size: 1.2rem;
        }
    }
</style>