import { redirect } from '@sveltejs/kit';

/**
 * @file This file redirects the user to the DPS tool page.
 */
export function load() {
  redirect(307, '/main/dps');
}
