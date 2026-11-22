<script lang="ts">
  import { createApiReference } from "@scalar/api-reference";
  import type { Attachment } from "svelte/attachments";

  import "@scalar/api-reference/style.css";

  const apiReference: Attachment = (element) => {
    const reference = createApiReference(element, {
      url: "/openapi.json",
      featuredClients: ["shell/curl", "rust/reqwest", "js/fetch"],
      agent: { disabled: true },
      mcp: { disabled: true },
      hideClientButton: true,
    });

    return () => reference.destroy();
  };
</script>

<div {@attach apiReference}></div>
