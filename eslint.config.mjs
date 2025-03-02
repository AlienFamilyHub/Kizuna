import antfu from "@antfu/eslint-config";
import tailwind from "eslint-plugin-tailwindcss";

export default antfu({
	vue: true,
	formatters: {
		css: true,
		html: true,
		prettierOptions: {
			plugins: ["prettier-plugin-tailwindcss"],
		},
	},
	stylistic: {
		indent: "tab",
		quotes: "double",
		semi: true,
	},
	rules: {
		"antfu/top-level-function": "off",
		"react-hooks/exhaustive-deps": "off",
	},
}, [
	...tailwind.configs["flat/recommended"],
	{
		settings: {
			tailwindcss: {
				callees: ["classnames", "clsx", "ctl"],
				config: "tailwind.config.js",
				cssFiles: [
					"**/*.css",
					"!**/node_modules",
					"!**/.*",
					"!**/dist",
					"!**/build",
				],
				cssFilesRefreshRate: 5_000,
				removeDuplicates: true,
				skipClassAttribute: false,
				whitelist: [],
				tags: [],
				classRegex: "^class(Name)?$",
			},
		},
	},
]);
