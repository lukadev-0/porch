<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    label: string;
    children: Snippet;
    error?: string;
  };

  const { label, children, error }: Props = $props();
</script>

<div data-porch-component="Field">
  <label>
    <div class="label-text">
      {label}
    </div>
    {@render children()}
  </label>
  {#if error !== undefined}
    <div class="error">
      {error}
    </div>
  {/if}
</div>

<style>
  @layer component {
    label {
      display: block;
    }

    .label-text {
      color: var(--color-text);
      font-size: var(--font-sm-size);
      line-height: var(--font-sm-line-height);
      font-weight: 500;
      margin-block: 0.25rem;
    }

    .error {
      margin-block: 0.125rem;
      color: var(--color-danger);
      font-size: var(--font-sm-size);
    }
  }
</style>
