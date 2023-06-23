import { type RequestHandler, error } from "@sveltejs/kit";
import { variables } from '../../../variables';
export const GET = ( async ({ url }) => {

    if (!url.searchParams.has("id")) {
        throw error(400, "Missing id parameter");
    }

    const id = url.searchParams.get("id");
    console.log(id);

    const response = await fetch(`${variables.basePath}/todo/${id}`);

    var todo = await response.json();
    console.log(todo);

    return new Response(JSON.stringify(todo));
}) satisfies RequestHandler;


export const POST = (async ({ request }) => {
    const todo = await request.json();

    console.log(todo);

    const response = await fetch(`${variables.basePath}/todo`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
          },
        body: JSON.stringify(todo)
    });

    return new Response(JSON.stringify(await response.json()));
}) satisfies RequestHandler;

export const PUT = ( async ({ url }) => {

    if (!url.searchParams.has("id")) {
        throw error(400, "Missing id parameter");
    }

    if (!url.searchParams.has("status")) {
        throw error(400, "Missing status parameter");
    }

    const id = url.searchParams.get("id");
    const status = url.searchParams.get("status");
    const response = await fetch(`${variables.basePath}/todo/${id}/${status}`, { method: "PUT" });

    var todo = await response.json();
    console.log(todo);

    return new Response(JSON.stringify(todo));
}) satisfies RequestHandler;

export const DELETE = ( async ({ url }) => {

    if (!url.searchParams.has("id")) {
        throw error(400, "Missing id parameter");
    }

    const id = url.searchParams.get("id");

    const response = await fetch(`${variables.basePath}/todo/${id}`, { method: "DELETE" });

    var todo = await response.json();
    console.log(todo);

    return new Response(JSON.stringify(todo));
}) satisfies RequestHandler;