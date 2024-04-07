<script>
	import Issue from './Issue.svelte';
	import Config from './Config.svelte';
	import { Tabs, Tab, TabContent } from 'carbon-components-svelte';

	export let issueCategories;
</script>

<Tabs autoWidth type="container">
	<Tab label="Issue" />
	<Tab label="Config" />
	<svelte:fragment slot="content">
		<TabContent><Issue {issueCategories} /></TabContent>
		<TabContent><Config /></TabContent>
	</svelte:fragment>
</Tabs>
