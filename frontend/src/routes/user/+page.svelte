<script lang="ts">
  import { finishPasskeyRegistration, startPasskeyRegistration } from "#lib/client/sdk.gen.ts";
  import { refreshAll } from "$app/navigation";
  import type { EventHandler } from "svelte/elements";
  import type { PageProps } from "./$types";

  const { data }: PageProps = $props();

  let registering = $state(false);
  let registerError = $state<string | undefined>();

  const registerPasskey = async (name: string) => {
    const { data: registration } = await startPasskeyRegistration({
      path: {
        user_id: "me",
      },
      throwOnError: true,
    });

    const credential = (await navigator.credentials.create({
      publicKey: PublicKeyCredential.parseCreationOptionsFromJSON(
        registration.credential_options.publicKey as PublicKeyCredentialCreationOptionsJSON,
      ),
    })) as PublicKeyCredential | null;

    if (credential === null) throw "Could not create credential.";

    await finishPasskeyRegistration({
      path: {
        user_id: "me",
      },
      body: {
        name,
        credential,
        registration_id: registration.registration_id,
      },
      throwOnError: true,
    });

    await refreshAll();
  };

  const handleRegisterPasskey: EventHandler<SubmitEvent, HTMLFormElement> = async (event) => {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);
    const name = formData.get("name")?.toString() ?? "";

    registering = true;
    registerError = undefined;
    try {
      await registerPasskey(name);
    } catch (error) {
      const message =
        error !== null && typeof error === "object" && "message" in error
          ? String(error.message)
          : String(error);
      console.error(message);
      registerError = message;
    }
    registering = false;
  };
</script>

<section>
  <h1>Hello, {data.user.display_name}!</h1>
  <p>
    Username: {data.user.name}<br />
    ID: {data.user.id}
  </p>
</section>

<hr />

<section>
  <h2>Passkeys</h2>
  {#if data.passkeys.length === 0}
    <p>No passkeys yet.</p>
  {:else}
    <ul>
      {#each data.passkeys as passkey}
        <li>{passkey.name} (ID: {passkey.id})</li>
      {/each}
    </ul>
  {/if}
  <form onsubmit={handleRegisterPasskey}>
    <label>
      Name:
      <input name="name" defaultValue="Passkey" />
    </label>
    <button disabled={registering}>Register a passkey</button>
  </form>
  {#if registerError !== undefined}
    <p>{registerError}</p>
  {/if}
</section>
