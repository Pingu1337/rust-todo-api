<script lang="ts">
  import { CloseButton } from "flowbite-svelte";
  import "../styles/todo-card-content.css";
  export let todo: Todo;
  export let user: string;

  const deleteTodo = async () => {
    console.log(todo.id);
    const response = await fetch(`/api/todo/?id=${todo.id}&user=${user}`, {
      method: "DELETE",
    });
    console.log(await response.json());
    window.location.reload();
  };
</script>

<header class="card-header flex justify-between items-center pe-0 pt-1">
  <b>{todo.title}</b>
  <CloseButton on:click={deleteTodo} class="todo-close-btn" />
</header>
<section class="p-4">{todo.content}</section>
