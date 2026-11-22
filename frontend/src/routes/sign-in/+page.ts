import { getUserById } from "#lib/client/sdk.gen.ts";
import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  const { data: user } = await getUserById({
    path: {
      user_id: "me",
    },
  });

  if (user !== undefined) {
    redirect(303, "/");
  }
};
