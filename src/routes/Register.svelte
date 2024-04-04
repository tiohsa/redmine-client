<script>
	import {
		TextInput,
		TextArea,
		Form,
		DatePicker,
		DatePickerInput,
		Select,
		SelectItem,
		Button,
		Loading,
		ToastNotification
	} from 'carbon-components-svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	let project_id = 'test';
	let subject = '';
	let priorities = [1, 2, 3, 4, 5];
	let priority_names = ['低め', '通常', '高め', '急いで', '今すぐ'];
	let priority_id = 2;
	let status_id = 1;
	let tracker_id = 2;
	let notes = '';
	let result = '';
	let start_date = new Date().toLocaleDateString('sv-SE');
	let due_date = new Date().toLocaleDateString('sv-SE');
	let loading = false;
	let response_status = '';

	let config = '';
	invoke('read')
		.then((json) => {
			config = json;
		})
		.catch((e) => (error = e));

	async function create_issue() {
		// POST / issues.json;
		// "Content-Type: application/json" -H "X-Redmine-API-Key: 7de23362bf2dfc965dec1413d72bf8daf337681b" -d @issue_test.json http://127.0.0.1/redmine/issues.json
		let data = {
			issue: {
				project_id: project_id,
				subject: subject,
				priority_id: priority_id,
				notes: notes,
				start_date: start_date,
				due_date: due_date
			}
		};
		console.log(data);
		try {
			loading = true;
			response_status = '';
			result = await invoke('issue', {
				json: JSON.stringify(data),
				config: config
			});
			if (result.startsWith('201')) {
				response_status = 'success';
			} else {
				response_status = 'error';
			}
		} catch (error) {
			result = error;
			console.error(error);
		} finally {
			loading = false;
		}
	}
</script>

<Form
	on:submit={(e) => {
		e.preventDefault();
		create_issue();
	}}
>
	<Button type="submit">Register</Button>
	<TextInput bind:value={subject} labelText="Subject" placeholder="Subject" required />
	<TextArea bind:value={notes} labelText="Notes" placeholder="Notes" />
	<Select bind:selected={priority_id} labelText="Priority" required>
		{#each priorities as priority}
			<SelectItem value={priority} text={priority_names[priority - 1]} />
		{/each}
	</Select>
	<DatePicker
		datePickerType="range"
		dateFormat="Y-m-d"
		bind:valueFrom={start_date}
		bind:valueTo={due_date}
	>
		<DatePickerInput
			labelText="Start Date"
			placeholder="yyyy-mm-dd"
			pattern={'\\d{4}-\\d{2}-\\d{2}'}
			required
		/>
		<DatePickerInput
			labelText="Due Date"
			placeholder="yyyy-mm-dd"
			pattern={'\\d{4}-\\d{2}-\\d{2}'}
			required
		/>
	</DatePicker>
</Form>
{#if loading}
	<Loading />
{/if}
{#if response_status}
	<ToastNotification
		lowContrast
		title="Result"
		fullWidth={true}
		kind={response_status}
		subtitle={result}
		caption={new Date().toLocaleString()}
	/>
{/if}
