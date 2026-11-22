<script lang="ts">
  import Meta from "#lib/Meta.svelte";
  import { invalidate } from "$app/navigation";
  import PasskeyList from "./PasskeyList.svelte";

  const { data } = $props();
</script>

<Meta title="Home" />

<h1>Hello, {data.user.display_name}!</h1>

<PasskeyList
  userId={data.user.id}
  passkeys={data.passkeys}
  invalidate={() => invalidate("me:passkeys")}
/>

<style>
  h1 {
    font-size: var(--font-xl-size);
    line-height: var(--font-xl-line-height);
    font-weight: 600;
    margin-bottom: 2rem;
  }
</style>
