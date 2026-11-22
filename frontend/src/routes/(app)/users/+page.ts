import { getUsers } from "#lib/client/index.ts";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ depends }) => {
  const { data: users } = await getUsers({
    throwOnError: true,
  });

  depends("app:users");

  return { users };
};
