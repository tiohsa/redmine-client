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
	import { getContext } from 'svelte';
	import Issue from './models/issue';

	const config = getContext('config');

	let result = undefined;
	let loading = false;

	let issue = new Issue({});

	async function create_issue() {
		loading = true;
		result = undefined;
		try {
			result = await issue.create($config);
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
	<TextInput bind:value={issue.subject} labelText="Subject" placeholder="Subject" required />
	<TextArea bind:value={issue.notes} labelText="Notes" placeholder="Notes" />
	<Select bind:selected={issue.priority_id} labelText="Priority" required>
		{#each issue.priorities as priority}
			<SelectItem value={priority} text={issue.priority_names[priority - 1]} />
		{/each}
	</Select>
	<DatePicker
		datePickerType="range"
		dateFormat="Y-m-d"
		bind:valueFrom={issue.start_date}
		bind:valueTo={issue.due_date}
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
{#if result}
	<ToastNotification
		lowContrast
		title={result.isRight ? 'Success' : 'Error'}
		fullWidth={true}
		kind={result.isRight ? 'success' : 'error'}
		subtitle={result.isRight ? '' : result.left}
		caption={new Date().toLocaleString()}
	/>
{/if}
