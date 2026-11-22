<script lang="ts">
  import { finishPasskeyRegistration, startPasskeyRegistration } from "#lib/client/sdk.gen.ts";
  import type { UserPasskey } from "#lib/client/types.gen.ts";
  import Button from "#lib/ui/Button.svelte";
  import Field from "#lib/ui/Field.svelte";
  import Input from "#lib/ui/Input.svelte";
  import type { EventHandler } from "svelte/elements";

  type Props = {
    userId: string;
    passkeys: UserPasskey[];
    invalidate: () => void | Promise<void>;
  };

  const { userId, passkeys, invalidate }: Props = $props();

  let registering = $state(false);
  let registerError: string | undefined = $state(undefined);

  const registerPasskey = async (name: string) => {
    const { data: registration } = await startPasskeyRegistration({
      path: {
        user_id: userId,
      },
      throwOnError: true,
    });

    const credential = (await navigator.credentials.create({
      publicKey: PublicKeyCredential.parseCreationOptionsFromJSON(
        registration.credential_options.publicKey as PublicKeyCredentialCreationOptionsJSON,
      ),
    })) as PublicKeyCredential | null;

    if (credential === null) throw new Error("Credential is null.");

    await finishPasskeyRegistration({
      path: {
        user_id: userId,
      },
      body: {
        registration_id: registration.registration_id,
        name,
        credential: credential,
      },
      throwOnError: true,
    });

    invalidate();
  };

  const handleRegisterPasskeySubmit: EventHandler<SubmitEvent, HTMLFormElement> = async (event) => {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);
    const name = formData.get("name")?.toString() ?? "";

    registering = true;

    try {
      await registerPasskey(name);
    } catch (error) {
      console.error(error);
      registerError = "Could not register passkey.";
    }

    registering = false;
  };
</script>

<section>
  <h2>Passkeys</h2>

  {#each passkeys as passkey}
    <article class="passkey">
      <h3>{passkey.name}</h3>
      <p>ID: {passkey.id}</p>
    </article>
  {:else}
    <p>No passkeys yet.</p>
  {/each}

  <form onsubmit={handleRegisterPasskeySubmit}>
    <h3>Add a passkey</h3>
    <Field label="Name">
      <Input name="name" required />
    </Field>
    <div class="row">
      <Button loading={registering}>Register</Button>
      {#if registerError !== undefined}
        <div class="error">
          {registerError}
        </div>
      {/if}
    </div>
  </form>
</section>

<style>
  h2 {
    font-size: var(--font-lg-size);
    line-height: var(--font-lg-line-height);
    font-weight: 600;
    margin-bottom: 1rem;
  }

  h3 {
    font-weight: 500;
  }

  .passkey {
    margin-bottom: 1rem;

    p {
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
