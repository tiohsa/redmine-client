<script>
	import Issue from './Issue.svelte';
	import Settings from './Settings.svelte';
	import { Tabs, Tab, TabContent } from 'carbon-components-svelte';
</script>

<Tabs autoWidth type="container">
	<Tab label="Reisger" />
	<Tab label="Settings" />
	<svelte:fragment slot="content">
		<TabContent><Issue /></TabContent>
		<TabContent><Settings /></TabContent>
	</svelte:fragment>
</Tabs>
