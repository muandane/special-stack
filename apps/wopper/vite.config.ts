import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	preview: {
		port: 5173,
		host: true,
		strictPort: true
	}
	// server: {
	// 	port: 8080,
	// 	origin: "http://0.0.0.0:8080",
	// 	strictPort: true,
	// },
});
