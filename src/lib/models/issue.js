import Result from './result';
import { invoke } from '@tauri-apps/api/tauri';

//$result = sendPostRequest($url, [
//   'issue' => [
//     'project_id' => 1,          // プロジェクト
//     'subject'    => 'APIテスト',  // 題名
//     // 以下は任意で指定する
//     //'description'      => '',  // 説明文
//     //'tracker_id'       => '',  // トラッカー
//     //'status_id'        => '',  // ステータス
//     //'priority_id'      => '',  // 優先度
//     //'category_id'      => '',  // カテゴリ
//     //'fixed_version_id' => '',  // 対象バージョン
//     //'assigned_to_id'   => '',  // 担当者
//     //'parent_issue_id'  => '',  // 親チケット
//     //'custom_fields'    => '',  // カスタムフィールド
//     //'watcher_user_ids' => '',  // ウォッチャー(ユーザーID) v2.3.0〜
//     //'is_private'       => '',  // プライベートチケットにするか true or false
//     //'estimated_hours'  => ''   // 予定工数
//   ]

export default class Issue {

    priorities = [1, 2, 3, 4, 5];
    priority_names = ['Low', 'Normal', 'Hight', 'Urgent ', 'Immediate'];

    constructor({
        id = undefined,
        project_id = '',
        project_name = undefined,
        subject = '',
        priority_id = 2,
        priority_name = undefined,
        status_id = 1,
        status_name = undefined,
        tracker_id = 2,
        tracker_name = undefined,
        category_id = undefined,
        category_name = undefined,
        notes = '',
        start_date = new Date().toLocaleDateString('sv-SE'),
        due_date = new Date().toLocaleDateString('sv-SE')
    }) {
        this.id = id;
        this.project_id = project_id;
        this.project_name = project_name;
        this.subject = subject;
        this.priority_id = priority_id;
        this.priority_name = priority_name;
        this.status_id = status_id;
        this.status_name = status_name;
        this.tracker_id = tracker_id;
        this.tracker_name = tracker_name;
        this.category_id = category_id;
        this.category_name = category_name;
        this.notes = notes;
        this.start_date = start_date;
        this.due_date = due_date;
    }

    create(config) {
        let issues = {}
        issues["issue"] = this;
        issues["issue"]["project_id"] = config.project_id
        const data_json = JSON.stringify(issues);
        const config_json = JSON.stringify(config);
        let result =
            invoke('post_issues', {
                issue: data_json,
                config: config_json
            }).then((response) => {
                if (response.startsWith('201')) {
                    return new Result(undefined, response);
                } else {
                    return new Result(response, undefined);
                }
            }).catch((e) => {
                console.error(e);
                return new Result(e, undefined);
            });
        return result;
    }

    getIssues(parameters, config) {
        const config_json = JSON.stringify(config);
        const parameters_json = JSON.stringify(parameters);
        const result =
            invoke('get_issues', {
                parameters: parameters_json,
                config: config_json
            })
                .then((response) => {
                    const json = JSON.parse(response);
                    console.log(json);
                    const categories = json.map((value) =>
                        new Issue({
                            id: value.id,
                            project_id: value.project?.id,
                            project_name: value.project?.name,
                            subject: value.subject,
                            priority_id: value.priority?.id,
                            priority_name: value.priority?.name,
                            status_id: value.status?.id,
                            status_name: value.status?.name,
                            category_id: value.category?.id,
                            category_name: value.category?.name,
                            due_date: value.due_date
                        })
                    );
                    return new Result(undefined, categories);
                })
                .catch((e) => {
                    console.error(e);
                    return new Result(e, undefined);
                });
        return result;
    }
}