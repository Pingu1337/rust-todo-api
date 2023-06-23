<script lang="ts">
  import { CloseButton } from "flowbite-svelte";
  import "../styles/todo-card-content.css";
  export let todo: Todo;

  const deleteTodo = async () => {
    console.log(todo.id);
    const response = await fetch(`/api/todo/?id=${todo.id}`, {
      method: "DELETE",
    });
    console.log(await response.json());
    window.location.reload();
  };
</script>

<div class="flex justify-between items-center">
  <small>{todo.id}</small>
  <CloseButton on:click={deleteTodo} class="todo-close-btn" />
</div>

<header class="card-header"><b>{todo.title}</b></header>
<section class="p-4">{todo.content}</section>
