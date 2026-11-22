<script lang="ts">
  import favicon from "#lib/assets/favicon.svg";
  import { page } from "$app/state";

  type Props = {
    title: string;
  };

  const { title }: Props = $props();

  const fullTitle = $derived(`${title} - ${page.data.configuration.name}`);
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
  <title>{fullTitle}</title>
</svelte:head>
