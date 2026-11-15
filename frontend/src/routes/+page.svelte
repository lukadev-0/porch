<script lang="ts">
  import { finishPasskeyAuthentication, startPasskeyAuthentication } from "#lib/client/sdk.gen.ts";
  import type { EventHandler } from "svelte/elements";

  let authenticating = $state(false);
  let authenticationMessage = $state<string | undefined>();

  const authenticate = async (username: string) => {
    const {
      data: authentication,
      error,
      response,
    } = await startPasskeyAuthentication({
      body: {
        username,
      },
    });

    if (response?.status === 404) {
      throw "User does not exist.";
    } else if (error !== undefined) {
      throw error;
    }

    const credential = (await navigator.credentials.get({
      publicKey: PublicKeyCredential.parseRequestOptionsFromJSON(
        authentication.credential_options.publicKey as PublicKeyCredentialRequestOptionsJSON,
      ),
    })) as PublicKeyCredential | null;

    if (credential === null) throw "Could not get credential.";

    const {
      data: { user_id: userId },
    } = await finishPasskeyAuthentication({
      body: {
        authentication_id: authentication.authentication_id,
        credential,
      },
      throwOnError: true,
    });

    authenticationMessage = `Authenticated! (User ID: ${userId})`;
  };

  const handleAuthenticate: EventHandler<SubmitEvent, HTMLFormElement> = async (event) => {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);
    const username = formData.get("username")?.toString() ?? "";

    authenticating = true;
    authenticationMessage = undefined;
    try {
      await authenticate(username);
    } catch (error) {
      const message =
        error !== null && typeof error === "object" && "message" in error
          ? String(error.message)
          : String(error);
      console.error(message);
      authenticationMessage = message;
    }
    authenticating = false;
  };
</script>

<p>To register passkeys, go to <a href="/user">the user</a>.</p>
<hr />
<form onsubmit={handleAuthenticate}>
  <label>
    Username:
    <input name="username" autocomplete="username" />
  </label>
  <button disabled={authenticating}>Authenticate</button>
  {#if authenticationMessage !== undefined}
    <p>{authenticationMessage}</p>
  {/if}
</form>
