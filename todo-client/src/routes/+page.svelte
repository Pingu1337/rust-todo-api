<script lang="ts">
  import "../@types/todo.d.ts";
  import Board from "../components/board.svelte";
  import { UseTodos } from "../hooks/useTodos.js";
  import type { PageData } from "./$types";
  import AddTodo from "../components/modal.svelte";

  export let data: PageData;

  let todoBoard: TodoList[] = UseTodos(data.todos);
</script>

<header class="flex justify-center mt-3">
  <h1 class="text-4xl font-bold">TODO: Kanban board</h1>
</header>

<div class="absolute left-4 ms-9 mt-9">
  <AddTodo />
</div>
<div class="flex justify-center">
  <Board columnItems={todoBoard} />
</div>

<style>
  :global(*) {
    box-sizing: border-box;
    margin: 0;
  }
</style>
