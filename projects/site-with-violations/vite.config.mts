import { defineConfig } from "vite";

export default defineConfig({
	build: {
		sourcemap: true,
		modulePreload: false,
		minify: false,
		cssMinify: false,
	},
	server: {
		headers: {
			"content-security-policy": "img-src 'self'; report-to default",
			"Reporting-Endpoints": 'default="http://localhost:4174/reports"',
		},
	},
});
