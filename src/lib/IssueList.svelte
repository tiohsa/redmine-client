<script>
	import Issue from './models/issue';
	import { getContext } from 'svelte';
	import {
		DataTable,
		Button,
		Loading,
		Toolbar,
		ToolbarContent,
		ToolbarSearch,
		Link,
		Tag
	} from 'carbon-components-svelte';
	import WatsonHealthRotate_360 from 'carbon-icons-svelte/lib/WatsonHealthRotate_360.svelte';

	const config = getContext('config');

	let issueList = [];
	let loading = false;
	let filteredRowIds = [];

	export const getIssueList = async () => {
		try {
			loading = true;
			let issue = new Issue({});
			let params = {
				project_id: $config.project_id,
				sort: 'category:desc,du_date:asc,priority:desc'
			};
			const result = await issue.getIssues(params, $config);
			if (result.isRight) {
				issueList = result.right;
			} else {
				console.error(result.left);
			}
		} finally {
			loading = false;
		}
	};

	function getPriorityColor(id) {
		if (id == 1) {
			return 'outline';
		} else if (id == 2) {
			return 'gray';
		} else if (id == 3) {
			return 'blue';
		} else if (id == 4) {
			return 'purple';
		} else {
			return 'red';
		}
	}
</script>

<Button on:click={() => getIssueList()} icon={WatsonHealthRotate_360}>Load</Button>
<DataTable
	style="height: 500px;"
	sortable
	headers={[
		{ key: 'id', value: 'ID' },
		{ key: 'subject', value: 'Subject' },
		{ key: 'category_name', value: 'Category Name' },
		{ key: 'priority_name', value: 'Priority Name' },
		{ key: 'status_name', value: 'Status Name' },
		{ key: 'due_date', value: 'Due Date' }
	]}
	rows={issueList}
>
	<svelte:fragment slot="cell" let:row let:cell>
		{#if cell.value === undefined}
			{''}
		{:else if cell.key === 'subject'}
			<Link href={`${$config.url}/issues/${row.id}`} target="_blank">{cell.value}</Link>
		{:else if cell.key === 'category_name'}
			<Tag type="red">{cell.value}</Tag>
		{:else if cell.key === 'priority_name'}
			<Tag type={getPriorityColor(row.priority_id)}>{cell.value}</Tag>
		{:else}
			{cell.value}
		{/if}
	</svelte:fragment>
	<Toolbar>
		<ToolbarContent>
			<ToolbarSearch persistent value="" shouldFilterRows bind:filteredRowIds />
		</ToolbarContent>
	</Toolbar>
</DataTable>
{#if loading}
	<Loading />
{/if}
