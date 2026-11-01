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
			"Connection-Allowlist-Report-Only":
				"(response-origin); report-to=default",
			"Content-Security-Policy":
				"img-src 'self' https://www.gstatic.com; report-to default",
			"Permissions-Policy": "unload=<allowlist>; report-to=default",
			"Reporting-Endpoints": 'default="/reports"',
		},
		proxy: {
			"/reports": "http://localhost:4174",
		},
	},
});
