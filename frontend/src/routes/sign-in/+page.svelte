<script lang="ts">
  import { finishPasskeyAuthentication, startPasskeyAuthentication } from "#lib/client/sdk.gen.ts";
  import Meta from "#lib/Meta.svelte";
  import Button from "#lib/ui/Button.svelte";
  import Field from "#lib/ui/Field.svelte";
  import Input from "#lib/ui/Input.svelte";
  import { withViewTransition } from "#lib/view-transitions.ts";
  import { refreshAll } from "$app/navigation";
  import { page } from "$app/state";
  import type { EventHandler } from "svelte/elements";

  let usernameError: string | undefined = $state(undefined);
  let authError: string | undefined = $state(undefined);
  let pending = $state(false);

  const onsubmit: EventHandler<SubmitEvent, HTMLFormElement> = async (event) => {
    event.preventDefault();

    if (pending) return;

    const formData = new FormData(event.currentTarget);
    const username = formData.get("username")?.toString() ?? "";

    const transition = withViewTransition(() => {
      pending = true;
      usernameError = undefined;
      authError = undefined;
    });

    const {
      data: authentication,
      error,
      response,
    } = await startPasskeyAuthentication({
      body: {
        username,
      },
    });

    await transition;

    if (response?.status === 404)
      return withViewTransition(() => {
        pending = false;
        usernameError = "This user does not exist.";
      });

    if (error !== undefined) {
      console.error(error);
      return withViewTransition(() => {
        pending = false;
        authError = "There was an error whilst signing you in...";
      });
    }

    let credential: PublicKeyCredential;
    try {
      const cred = (await navigator.credentials.get({
        publicKey: PublicKeyCredential.parseRequestOptionsFromJSON(
          authentication.credential_options.publicKey as PublicKeyCredentialCreationOptionsJSON,
        ),
      })) as PublicKeyCredential | null;

      if (cred === null) throw new Error("Credential is null.");

      credential = cred;
    } catch (error) {
      console.error(error);
      return withViewTransition(() => {
        pending = false;
        authError = "Failed to get passkey.";
      });
    }

    {
      const { error, response } = await finishPasskeyAuthentication({
        body: {
          authentication_id: authentication.authentication_id,
          credential,
        },
      });

      if (error !== undefined) {
        console.error(error);
        return withViewTransition(() => {
          pending = false;
          authError = response?.status === 400 ? "The passkey was rejected." : "Unknown error.";
        });
      }
    }

    await refreshAll();
    pending = false;
  };

  const becomeAdmin = async () => {
    await fetch("/api/become-admin", { method: "POST" });
    refreshAll();
  };
</script>

<Meta title="Sign in" />

<div class="root">
  <main class="card">
    <div class="content">
      <div class="header">
        <h1>Sign in</h1>
        <p>Use your {page.data.configuration.name} account.</p>
        {#if authError !== undefined}
          <p class="error">
            {authError}
          </p>
        {/if}
      </div>
      <form class="sign-in-form" {onsubmit}>
        <Field label="Username" error={usernameError}>
          <Input name="username" autocomplete="username" required autofocus />
        </Field>
        <Button type="submit" variant="primary" loading={pending}>Sign in</Button>
      </form>
    </div>
    <div class="footer">
      <button class="become-admin-btn" onclick={becomeAdmin}>Become admin</button>
    </div>
    <div class="footer">Secured by Porch</div>
  </main>
</div>

<style>
  @layer page {
    .root {
      display: flex;
      align-items: center;
      justify-content: center;
      min-height: 100vh;
    }

    .card {
      width: 24rem;
      view-transition-name: match-element;

      --padding: 2rem;
    }

    .content {
      border: 1px solid var(--color-border);
      view-transition-name: sign-in-content;
      background-color: var(--color-surface);
      border-top-left-radius: var(--border-radius-lg);
      border-top-right-radius: var(--border-radius-lg);
      border-bottom: none;
      padding: 0;
    }

    .header {
      padding-inline: var(--padding);
      padding-block: var(--padding) 1rem;
      text-align: center;
      view-transition-name: match-element;

      h1 {
        font-size: var(--font-lg-size);
        line-height: var(--font-lg-line-height);
        font-weight: 600;
        margin-block-end: 0.5rem;
      }

      p {
        font-size: var(--font-sm-size);
        line-height: var(--font-sm-line-height);
        color: var(--color-text-secondary);
      }

      .error {
        background-color: var(--color-danger-container);
        border-radius: var(--border-radius);
        padding: 0.5rem;
        color: var(--color-on-danger-container);
        margin-top: 2rem;
      }
    }

    .sign-in-form {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      padding-inline: var(--padding);
      padding-block-start: 1rem;
      padding-block-end: 3.5rem;

      > :global(*) {
        view-transition-name: match-element;
      }
    }

    .become-admin-btn {
      background: none;
      border: none;
      color: var(--color-text-secondary);
      &:hover {
        color: var(--color-on-primary-container);
      }
    }

    .footer {
      font-size: var(--font-xs-size);
      line-height: var(--font-xs-line-height);
      font-weight: 500;
      color: var(--color-text-tertiary);
      padding: 0.75rem var(--padding);
      background-color: var(--color-surface-secondary);
      text-align: center;
      border: 1px solid var(--color-border);
      border-bottom-width: 0;
      view-transition-name: match-element;

      &:last-child {
        border-bottom-left-radius: var(--border-radius-lg);
        border-bottom-right-radius: var(--border-radius-lg);
        border-bottom-width: 1px;
      }
    }

    ::view-transition-old(sign-in-content),
    ::view-transition-new(sign-in-content) {
      height: 100%;
    }
  }
</style>
