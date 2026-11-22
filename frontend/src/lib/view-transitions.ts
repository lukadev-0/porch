import { tick } from "svelte";

export async function withViewTransition(fn: () => void | Promise<void>) {
  if ("startViewTransition" in document) {
    const transition = document.startViewTransition(async () => {
      await fn();
      await tick();
    });
    await transition.finished;
  }

  await fn();
}
