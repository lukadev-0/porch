<script lang="ts">
  import "../styles/globals.css";
  import "#lib/client-config.ts";

  import favicon from "#lib/assets/favicon.svg";
  import type { LayoutProps } from "./$types";

  let { children, data }: LayoutProps = $props();

  $effect(() => {
    document.documentElement.style.setProperty(
      "--accent-hue",
      `${data.configuration.accentHue}deg`,
    );
  });
</script>

{@render children()}
