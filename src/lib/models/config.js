import Result from './result';
import { invoke } from '@tauri-apps/api/tauri';

export default class Config {

    constructor(url, token, project_id) {
        this.url = url;
        this.token = token;
        this.project_id = project_id;
    }

    read() {
        const result =
            invoke('read_config')
                .then((response) => {
                    const json = JSON.parse(response);
                    const conf = Object.assign(new Config(), json);
                    return new Result(undefined, conf);
                })
                .catch((e) => {
                    console.error(e);
                    return new Result(e, undefined);
                });
        return result;
    }


    save(config) {
        const config_json = JSON.stringify(config)
        const result =
            invoke('save_config', { config: config_json })
                .then((_response) => {
                    return new Result(undefined, "Save Config");
                })
                .catch((e) => {
                    console.error(e);
                    return new Result(e, undefined);
                });
        return result;
    }
}