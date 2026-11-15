import type { PageLoad } from "./$types";
import { getUserById, getUserPasskeys } from "#lib/client/sdk.gen.ts";

export const load: PageLoad = async () => {
  const [{ data: user }, { data: passkeys }] = await Promise.all([
    getUserById({
      path: {
        user_id: "me",
      },
      throwOnError: true,
    }),
    getUserPasskeys({
      path: {
        user_id: "me",
      },
      throwOnError: true,
    }),
  ]);

  return {
    user,
    passkeys,
  };
};
