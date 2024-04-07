<script>
	import Redmine from '$lib/Redmine.svelte';
	import Config from '$lib/models/config';
	import { setContext } from 'svelte';
	import { writable } from 'svelte/store';

	/** @type {import('./$types').LayoutData} */
	export let data;

	let config = new Config();
	if (data.config.isRight) {
		config = data.config.right;
	}
	console.log(config);
	const config_store = writable();
	$: config_store.set(config);
	setContext('config', config_store);
</script>

<svelte:head>
	<title>Redmine Ticket Register</title>
	<meta name="description" content="Redmine" />
</svelte:head>

<section>
	<Redmine />
</section>
