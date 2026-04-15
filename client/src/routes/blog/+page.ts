export const load = async ({ fetch }) => {
    const res = await fetch('/blogs/index.json');
    const blogs = await res.json();
    return { blogs };
};