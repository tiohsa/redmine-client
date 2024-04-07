<script>
	import { TextInput, Form, Button, ToastNotification } from 'carbon-components-svelte';
	import Save from 'carbon-icons-svelte/lib/Save.svelte';
	import Config from './models/config';
	import { getContext } from 'svelte';

	const config = getContext('config');

	let result = undefined;

	async function save() {
		result = undefined;
		result = await new Config().save($config);
	}
</script>

<Form
	on:submit={(e) => {
		e.preventDefault();
		save();
	}}
>
	<Button type="submit" icon={Save}>Save</Button>
	<TextInput
		bind:value={$config.url}
		labelText="URL"
		placeholder="http://localhost:8080"
		required
	/>
	<TextInput bind:value={$config.token} labelText="Token" placeholder="Token" required />
	<TextInput
		bind:value={$config.project_id}
		labelText="Project Id"
		placeholder="Project Id"
		required
	/>
</Form>

{#if result}
	<ToastNotification
		lowContrast
		title={result.isRight ? 'Success' : 'Error'}
		fullWidth={true}
		kind={result.isRight ? 'success' : 'error'}
		subtitle={result.isRight ? result.right : result.left}
		caption={new Date().toLocaleString()}
	/>
{/if}
