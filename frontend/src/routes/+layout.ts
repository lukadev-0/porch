import type { LayoutLoad } from "./$types";

export const ssr = false;

export const load: LayoutLoad = () => {
  return {
    configuration: {
      name: "Porch Demo",
      accentHue: 250,
    },
  };
};
