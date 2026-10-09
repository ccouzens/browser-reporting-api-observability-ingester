import { defineConfig } from "vite";

export default defineConfig({
	build: {
		sourcemap: true,
		modulePreload: false,
		minify: false,
		cssMinify: false,
	},
	preview: {
		allowedHosts: true,
	},
	server: {
		headers: {
			"content-security-policy": "img-src 'self'; report-to default",
			"Reporting-Endpoints": 'default="/reports"',
		},
		proxy: {
			"/reports": "http://localhost:4174",
		},
	},
});
