<script lang="ts">
  import type { FullAutoFill, HTMLInputTypeAttribute } from "svelte/elements";

  type Props = {
    name: string;
    type?: HTMLInputTypeAttribute;
    required?: boolean;
    autocomplete?: FullAutoFill;
    autofocus?: boolean;
  };

  const { ...props }: Props = $props();
</script>

<input data-porch-component="Input" {...props} />

<style>
  @layer component {
    input {
      display: block;
      width: 100%;
      padding: 0 0.75rem;
      height: 2.5rem;
      border-radius: var(--border-radius);
      background-color: var(--field-background-color);
      border: 1px solid var(--field-outline-color);

      &:hover,
      &:focus {
        border-color: var(--field-hovered-outline-color);
        background-color: var(--field-hovered-background-color);
      }

      &:focus {
        outline: 3px solid var(--field-focused-ring);
        border-color: var(--field-focused-outline-color);
      }
    }
  }
</style>
