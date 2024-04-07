// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;

import IssueCategory from '$lib/models/issue_category';
import Config from '../lib/models/config';

/** @type {import('./$types').LayoutLoad} */
export async function load() {
    let config = await new Config().read()
    let issueCategories = [];
    if (config.isRight) {
        issueCategories = (await new IssueCategory({}).getIssueCategories(config.right)).right;
    }
    return {
        config: config,
        issueCategories: issueCategories
    };
}