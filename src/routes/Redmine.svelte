<script>
	import Register from './Register.svelte';
	import Settings from './Settings.svelte';
	import { Tabs, Tab, TabContent } from 'carbon-components-svelte';
</script>

<Tabs autoWidth type="container">
	<Tab label="Reisger" />
	<Tab label="Settings" />
	<svelte:fragment slot="content">
		<TabContent><Register /></TabContent>
		<TabContent><Settings /></TabContent>
	</svelte:fragment>
</Tabs>
