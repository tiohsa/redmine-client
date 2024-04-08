<script>
	import Issue from './Issue.svelte';
	import Config from './Config.svelte';
	import IssueList from './IssueList.svelte';
	import { Tabs, Tab, TabContent, Grid, Row, Column } from 'carbon-components-svelte';

	export let issueCategories;

	let getIssueList;

	function loadIssueList() {
		getIssueList();
	}
</script>

<Tabs autoWidth type="container">
	<Tab label="Issue" />
	<Tab label="Issue List" on:click={() => loadIssueList()} />
	<Tab label="Config" />
	<svelte:fragment slot="content">
		<TabContent><Issue {issueCategories} /></TabContent>
		<TabContent><IssueList bind:getIssueList /></TabContent>
		<TabContent><Config /></TabContent>
	</svelte:fragment>
</Tabs>
