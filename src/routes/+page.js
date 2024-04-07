// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;

import Config from '../lib/models/config';

/** @type {import('./$types').LayoutLoad} */
export async function load() {
    return {
        config: await new Config().read()
    };
}