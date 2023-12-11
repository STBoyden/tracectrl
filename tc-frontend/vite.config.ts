import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [react()],
	server: {
		port: 8000,
		strictPort: true,
		hmr: {
			port: 8001,
		},
		proxy: {
			"/ws": {
				ws: true,
				target: "base",
				rewrite(path) {
					return path.replace(/^\/ws/, "");
				},
			},
		},
	},
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "./src"),
		},
	},
});
