<script lang="ts">
  import { refreshAll } from "$app/navigation";
  import { page } from "$app/state";

  const { children, data } = $props();

  const navItems = $derived([
    { label: "Home", href: "/" },
    ...(data.user.is_admin ? [{ label: "Users", href: "/users" }] : []),
  ]);

  const signOut = async () => {
    await fetch("/api/auth/sign-out", { method: "POST" });
    refreshAll();
  };
</script>

<div class="root">
  <header class="sidebar">
    <div class="site-name">{data.configuration.name}</div>

    <nav class="nav">
      {#each navItems as navItem}
        <a class="nav-item" href={navItem.href} data-active={navItem.href === page.url.pathname}>
          {navItem.label}
        </a>
      {/each}
    </nav>

    <div class="sign-out">
      <button onclick={signOut}>Sign out</button>
    </div>
  </header>
  <main>
    {@render children()}
  </main>
</div>

<style>
  .root {
    --sidebar-width: 12rem;
    display: flex;
    min-height: 100vh;
    padding: 0.5rem;
    padding-left: var(--sidebar-width);
  }

  .sidebar {
    --inline-padding: 0.75rem;
    position: fixed;
    left: 0;
    top: 0;
    bottom: 0;
    display: flex;
    flex-direction: column;
    width: var(--sidebar-width);
    padding: 0.5rem;
    flex-shrink: 0;
  }

  .site-name {
    font-weight: 600;
    padding-inline: var(--inline-padding);
    padding-block: 0.5rem;
  }

  .nav {
    display: flex;
    flex-direction: column;
    padding-block: 0.5rem;
  }

  .nav-item {
    --background-color: transparent;
    position: relative;
    display: flex;
    align-items: center;
    color: var(--color-text-secondary);
    text-decoration: none;
    height: 2.25rem;
    padding-inline: var(--inline-padding);
    font-size: var(--font-sm-size);
    font-weight: 500;

    &::after {
      position: absolute;
      inset-block: 0.125rem;
      inset-inline: 0;
      content: "";
      border-radius: var(--border-radius);
      z-index: -1;
      background-color: var(--background-color);
    }

    &:hover {
      --background-color: var(--color-surface-secondary);
      color: var(--color-text);
    }

    &[data-active="true"] {
      --background-color: var(--color-primary-container);
      color: var(--color-on-primary-container);
    }
  }

  .sign-out {
    margin-block-start: auto;
    padding-inline: var(--inline-padding);
    padding-block: 0.25rem;

    button {
      width: 100%;
      background: none;
      border: none;
      font-size: var(--font-sm-size);
      font-weight: 500;
      height: 2.25rem;
      color: var(--color-text);

      &:hover {
        color: var(--color-on-primary-container);
      }
    }
  }

  main {
    display: flex;
    flex-direction: column;
    width: 100%;
    background-color: var(--color-surface);
    border-radius: var(--border-radius-lg);
    padding-inline: calc(max(2rem, 50vw - 30rem));
    padding-block: 2rem;
  }
</style>
