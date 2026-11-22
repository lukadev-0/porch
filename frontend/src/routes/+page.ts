import type { PageLoad } from "./$types";
import { getUserById } from "#lib/client/sdk.gen.ts";

export const load: PageLoad = async () => {
  const { data: user } = await getUserById({
    path: {
      user_id: "me",
    },
  });

  return {
    user,
  };
};
