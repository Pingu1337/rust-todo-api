import type { RequestHandler, error } from "@sveltejs/kit";
import { variables } from '../../../variables';

export const GET = ( async ({ url }) => {

    console.log(variables.basePath);
    const response = await fetch(`${variables.basePath}/todos`);

    var todos = await response.json();
    console.log(todos);

    return new Response(JSON.stringify(todos));
}) satisfies RequestHandler;