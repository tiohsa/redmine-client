import Result from './result';
import { invoke } from '@tauri-apps/api/tauri';

export default class IssueCategory {

    constructor({ id = undefined, name = '' }) {
        this.id = id;
        this.name = name;
    }

    getIssueCategories(conf) {
        const config = JSON.stringify(conf);
        const result =
            invoke('get_issue_categories', {
                config: config
            })
                .then((response) => {
                    const json = JSON.parse(response);
                    const categories = json.map((value) => new IssueCategory({ id: value.id, name: value.name }));
                    return new Result(undefined, categories);
                })
                .catch((e) => {
                    console.error(e);
                    return new Result(e, undefined);
                });
        return result;
    }
}