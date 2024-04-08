import Result from './result';
import { invoke } from '@tauri-apps/api/tauri';

export default class IssueCategory {

    constructor({ id = undefined, name = '' }) {
        this.id = id;
        this.name = name;
    }

    getIssueCategories(config) {
        const config_json = JSON.stringify(config);
        const result =
            invoke('get_issue_categories', {
                config: config_json
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