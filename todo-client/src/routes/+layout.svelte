<script lang="ts">
  import "@skeletonlabs/skeleton/themes/theme-crimson.css";
  import "@skeletonlabs/skeleton/styles/skeleton.css";
  import "../app.postcss";
  import { LightSwitch, clipboard } from "@skeletonlabs/skeleton";
  import { browser } from "$app/environment";

  let signedIn: boolean | null = null;
  let userCode: string | null = null;
  if (browser) {
    userCode = window.localStorage.getItem("user");
    signedIn = userCode ? true : false;
  }

  const signOut = () => {
    window.localStorage.removeItem("user");
    window.location.replace("/");
  };

  let copied: boolean = false;
</script>

{#if signedIn != null}
  <main>
    <header class="flex justify-end items-center mt-5 me-5 flex-row">
      <div class="flex justify-evenly gap-4 items-center">
        {#if signedIn}
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <small
            class="me-6 select-none cursor-pointer"
            use:clipboard={userCode}
            on:click={() => {
              copied = true;
              setTimeout(() => {
                copied = false;
              }, 1000);
            }}
          >
            {#if copied}
              <code>User code copied to clipboard!</code>
            {:else}
              <code class="flex"
                >User code: <pre> {userCode}</pre></code
              >
            {/if}
          </small>
          <button class="btn variant-soft" on:click={signOut}>Sign out</button>
        {/if}
        <LightSwitch />
      </div>
    </header>
    <slot />
  </main>
{/if}
