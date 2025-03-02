// https://nuxt.com/docs/api/configuration/nuxt-config
import { defineNuxtConfig } from "nuxt/config";

export default defineNuxtConfig({
	compatibilityDate: "2024-11-01",
	devtools: { enabled: true },
	typescript: {
		typeCheck: true,
	},
	srcDir: "src/",
	css: ["~/assets/css/main.css"],
	vite: {
		clearScreen: false,
		envPrefix: ["VITE_", "TAURI_"],
		server: {
			strictPort: true,
		},
	},
	ui: {
		// @ts-expect-error - 忽略类型检查
		notifications: {
			position: "top-0 bottom-[unset]",
		},
	},
	ssr: false,
	postcss: {
		plugins: {
			tailwindcss: {},
			autoprefixer: {},
		},
	},
	modules: ["@pinia/nuxt", "@nuxt/icon", "@nuxt/ui"],
});
