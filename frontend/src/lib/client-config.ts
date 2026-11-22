import { error, redirect } from "@sveltejs/kit";
import { client } from "./client/client.gen";

client.interceptors.response.use((response) => {
  if (response.status === 401) {
    redirect(303, "/sign-in");
  }
  if (response.status === 403) {
    error(403, "Access denied");
  }
  return response;
});
