<script lang="ts">
  import Button from "#lib/ui/Button.svelte";
  import Field from "#lib/ui/Field.svelte";
  import Input from "#lib/ui/Input.svelte";
  import type { EventHandler } from "svelte/elements";
  import { createUser } from "#lib/client/sdk.gen.ts";
  import { invalidate } from "$app/navigation";
  import Meta from "#lib/Meta.svelte";

  const { data } = $props();

  let creating = $state(false);
  let createError: string | undefined = $state(undefined);

  const handleCreateUserSubmit: EventHandler<SubmitEvent, HTMLFormElement> = async (event) => {
    event.preventDefault();

    const form = event.currentTarget;
    const formData = new FormData(form);
    const username = formData.get("username")?.toString() ?? "";
    const displayName = formData.get("display_name")?.toString() ?? "";

    creating = true;

    try {
      await createUser({
        body: {
          name: username,
          display_name: displayName,
        },
        throwOnError: true,
      });

      await invalidate("app:users");

      form.reset();
    } catch (error) {
      console.error(error);
      createError = "Could not create user.";
    }

    creating = false;
  };
</script>

<Meta title="Users" />

<h1>Users</h1>

{#each data.users as user}
  <article>
    <h2>
      <a href="/users/{user.id}">@{user.name}</a>
      {#if user.is_admin}
        <div class="admin-badge">Admin</div>
      {/if}
    </h2>
    <div class="description">{user.display_name} (ID: {user.id})</div>
  </article>
{/each}

<form onsubmit={handleCreateUserSubmit}>
  <h2>Add a user</h2>
  <Field label="Username">
    <Input name="username" required />
  </Field>
  <Field label="Display name">
    <Input name="display_name" required />
  </Field>
  <div class="row">
    <Button loading={creating}>Create</Button>
    {#if createError !== undefined}
      <div class="error">
        {createError}
      </div>
    {/if}
  </div>
</form>

<style>
  h1 {
    font-size: var(--font-xl-size);
    line-height: var(--font-xl-line-height);
    font-weight: 600;
    margin-bottom: 2rem;
  }

  h2 {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;

    a {
      color: var(--color-on-primary-container);
    }
  }

  .admin-badge {
    font-size: var(--font-xs-size);
    line-height: var(--font-xs-line-height);
    background-color: var(--color-primary-container);
    color: var(--color-on-primary-container);
    padding-inline: 0.5rem;
    padding-block: 0.125rem;
    border-radius: var(--border-radius);
  }

  article {
    margin-bottom: 1rem;

    .description {
      font-size: var(--font-sm-size);
      line-height: var(--font-sm-line-height);
      color: var(--color-text-secondary);
    }
  }

  form {
    display: flex;
    flex-direction: column;
    margin-block: 2rem;
    gap: 0.5rem;
    padding: 1rem;
    margin-inline: -1rem;
    background-color: var(--color-surface-secondary);
    border-radius: var(--border-radius-lg);
    border: 1px solid var(--color-border);

    .row {
      display: flex;
      gap: 1rem;
      align-items: center;
    }

    .error {
      color: var(--color-danger);
      font-size: var(--font-sm-size);
      line-height: var(--font-sm-line-height);
    }
  }
</style>
