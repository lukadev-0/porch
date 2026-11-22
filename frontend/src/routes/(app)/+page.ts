import { getUserPasskeys } from "#lib/client/sdk.gen.ts";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ depends }) => {
  const { data: passkeys } = await getUserPasskeys({
    path: {
      user_id: "me",
    },
    throwOnError: true,
  });

  depends("me:passkeys");

  return {
    passkeys,
  };
};
