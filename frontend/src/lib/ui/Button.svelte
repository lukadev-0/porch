<script lang="ts">
  import type { Snippet } from "svelte";
  import Spinner from "./Spinner.svelte";

  type Props = {
    children: Snippet;
    variant?: "default" | "primary";
    loading?: boolean;
    disabled?: boolean;
    type?: "button" | "submit" | "reset";
  };

  const {
    children,
    variant = "default",
    loading = false,
    disabled = false,
    ...props
  }: Props = $props();
</script>

<button
  data-porch-component="Button"
  data-variant={variant}
  data-loading={loading}
  disabled={disabled || loading}
  {...props}
>
  <span class="text">
    {@render children()}
  </span>
  {#if loading}
    <div class="loading-indicator">
      <Spinner />
    </div>
  {/if}
</button>

<style>
  @layer component {
    button {
      --_text-color: var(--button-text-color);
      --_background-color: var(--button-background-color);
      --_overlay-strength: var(--button-overlay-strength);
      --_background-overlay-opacity: 0;
      --_focus-ring: var(--_text-color);

      &[data-variant="primary"] {
        --_text-color: var(--button-primary-text-color);
        --_background-color: var(--button-primary-background-color);
        --_overlay-strength: var(--button-primary-overlay-strength);
        --_focus-ring: var(--_background-color);
      }

      &[data-variant="danger"] {
        --_text-color: var(--button-danger-text-color);
        --_background-color: var(--button-danger-background-color);
        --_overlay-strength: var(--button-danger-overlay-strength);
        --_focus-ring: var(--_background-color);
      }

      position: relative;
      color: var(--_text-color);
      height: 2.25rem;
      font-size: var(--font-sm-size);
      line-height: var(--font-sm-line-height);
      font-weight: 600;
      background:
        linear-gradient(hsl(0deg 0% 100% / var(--_background-overlay-opacity))),
        linear-gradient(hsl(0deg 0% 100% / var(--_overlay-strength)), transparent),
        linear-gradient(var(--_background-color));
      border: none;
      border-radius: var(--border-radius);
      box-shadow: inset 0 0 0 1px hsl(0deg 0% 100% / calc(var(--_overlay-strength) * 1.5));

      &[disabled] {
        opacity: 0.7;
      }

      &:not([disabled]):hover {
        --_background-overlay-opacity: calc(var(--_overlay-strength) * 0.5);
      }

      &:not([disabled]):active {
        --_background-overlay-opacity: var(--_overlay-strength);
      }

      &:not([disabled]):focus-visible {
        outline: 2px solid var(--_focus-ring);
        outline-offset: 2px;
      }

      &[data-loading="true"] .text {
        opacity: 0;
      }
    }

    .loading-indicator {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
    }
  }
</style>
