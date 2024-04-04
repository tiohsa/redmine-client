import { sveltekit } from '@sveltejs/kit/vite';
import { optimizeCss } from "carbon-preprocess-svelte";
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [sveltekit(), optimizeCss()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
