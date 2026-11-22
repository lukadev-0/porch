<script lang="ts">
  import "../styles/globals.css";
  import "#lib/client-config.ts";

  let { children, data } = $props();

  $effect(() => {
    document.documentElement.style.setProperty(
      "--accent-hue",
      `${data.configuration.accentHue}deg`,
    );
  });
</script>

{@render children()}
