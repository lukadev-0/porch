import { getUserById, getUserPasskeys } from "#lib/client/sdk.gen.ts";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params, depends }) => {
  const [{ data: user }, { data: passkeys }] = await Promise.all([
    getUserById({
      path: {
        user_id: params.userId,
      },
      throwOnError: true,
    }),
    getUserPasskeys({
      path: {
        user_id: params.userId,
      },
      throwOnError: true,
    }),
  ]);

  depends(`user:${user.id}:passkeys`);

  return {
    thisUser: user,
    passkeys,
  };
};
