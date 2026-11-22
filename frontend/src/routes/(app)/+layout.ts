import { getUserById } from "#lib/client/sdk.gen.ts";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async () => {
  const { data: user } = await getUserById({
    path: {
      user_id: "me",
    },
    throwOnError: true,
  });

  return {
    user,
  };
};
