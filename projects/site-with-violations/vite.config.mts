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
			NEL: JSON.stringify({
				report_to: "nel",
				max_age: 600,
				success_fraction: 1,
				failure_fraction: 1,
			}),
			"Permissions-Policy": "unload=<allowlist>; report-to=default",
			"Reporting-Endpoints": 'default="/reports"',
			"Report-To": JSON.stringify({
				group: "nel",
				max_age: 600,
				endpoints: [{ url: "/reports" }],
			}),
		},
		proxy: {
			"/reports": "http://localhost:4174",
		},
	},
});
