<script>
	import { TextInput, Form, Button, ToastNotification } from 'carbon-components-svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	let error = '';
	let url = '';
	let token = '';
	let save_result = '';
	let save_status = '';
	let settings = { url: '', token: '' };

	invoke('read')
		.then((json) => {
			settings = JSON.parse(json);
			console.log(settings);
			url = settings.url;
			token = settings.token;
		})
		.catch((e) => (error = e));

	async function save() {
		save_status = '';
		let data = {
			url: url,
			token: token
		};
		console.log(data);
		try {
			save_result = await invoke('save', { json: JSON.stringify(data) });
			save_status = 'success';
		} catch (error) {
			save_result = error;
			save_status = 'error';
			console.error(error);
		}
	}
</script>

<Form
	on:submit={(e) => {
		e.preventDefault();
		save();
	}}
>
	<Button type="submit">Save</Button>
	<TextInput bind:value={url} labelText="URL" placeholder="http://localhost:8080" required />
	<TextInput bind:value={token} labelText="Token" placeholder="Token" required />
</Form>

{#if save_status}
	<ToastNotification
		lowContrast
		title="Result"
		fullWidth={true}
		kind={save_status}
		subtitle={save_result}
		caption={new Date().toLocaleString()}
	/>
{/if}
