import type { PageLoad } from './$types';

export const load = (async ({ fetch, params }) => {
    const todoResponse = await fetch("/api/todos");

    const todos = await todoResponse.json();
    return {
        todos
    };

}) satisfies PageLoad;