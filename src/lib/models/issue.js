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
    priority_names = ['低め', '通常', '高め', '急いで', '今すぐ'];

    constructor({
        project_id = '',
        subject = '',
        priority_id = 2,
        status_id = 1,
        tracker_id = 2,
        notes = '',
        start_date = new Date().toLocaleDateString('sv-SE'),
        due_date = new Date().toLocaleDateString('sv-SE')
    }) {
        this.project_id = project_id;
        this.subject = subject;
        this.priority_id = priority_id;
        this.status_id = status_id;
        this.tracker_id = tracker_id;
        this.notes = notes;
        this.start_date = start_date;
        this.due_date = due_date;
    }

    create(conf) {
        let issues = {}
        issues["issue"] = this;
        issues["issue"]["project_id"] = conf.project_id
        const data = JSON.stringify(issues);
        const config = JSON.stringify(conf);
        let result =
            invoke('issue', {
                issue: data,
                config: config
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
}