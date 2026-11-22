<script lang="ts">
  import Meta from "#lib/Meta.svelte";
  import { invalidate } from "$app/navigation";
  import PasskeyList from "../../PasskeyList.svelte";

  const { data } = $props();
</script>

<Meta title="@{data.thisUser.name} - Users" />

<section>
  <h1>@{data.thisUser.name}</h1>
  <p><strong>Display name:</strong> {data.thisUser.display_name}</p>
  <p><strong>ID:</strong> {data.thisUser.id}</p>
  <p><strong>Admin:</strong> {data.thisUser.is_admin ? "Yes" : "No"}</p>
</section>

<PasskeyList
  userId={data.thisUser.id}
  passkeys={data.passkeys}
  invalidate={() => invalidate(`user:${data.thisUser.id}:passkeys`)}
/>

<style>
  h1 {
    font-size: var(--font-xl-size);
    line-height: var(--font-xl-line-height);
    font-weight: 600;
    margin-bottom: 1rem;
  }

  section {
    margin-bottom: 2rem;
  }
</style>
