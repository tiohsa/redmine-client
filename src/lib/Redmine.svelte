<script>
	import Issue from './Issue.svelte';
	import Config from './Config.svelte';
	import { Tabs, Tab, TabContent } from 'carbon-components-svelte';
</script>

<Tabs autoWidth type="container">
	<Tab label="Reisger" />
	<Tab label="Config" />
	<svelte:fragment slot="content">
		<TabContent><Issue /></TabContent>
		<TabContent><Config /></TabContent>
	</svelte:fragment>
</Tabs>
