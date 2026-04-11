import { error } from '@sveltejs/kit';
import { marked } from 'marked';

export const load = async ({ fetch, params }) => {
    const res = await fetch(`/blogs/${params.slug}.md`);
    if (!res.ok) error(404, 'Siden du leder efter findes ikke');

    const md = await res.text();

    let title = '';
    const renderer = new marked.Renderer({ breaks: true });
    renderer.heading = ({ depth, text }) => {
        if (depth === 1) { title = text; return ''; }
        return `<h${depth}>${text}</h${depth}>`;
    };

    const content = await marked(md, { renderer });
    return { content, title };
};