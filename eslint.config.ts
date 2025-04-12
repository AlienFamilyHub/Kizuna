import antfu from "@antfu/eslint-config";
import tailwind from "eslint-plugin-tailwindcss";

const atf = antfu({
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
		"eslinttailwindcss/no-custom-classname": "off",
		"no-console": ["warn", { allow: ["warn", "error"] }],
		"brace-style": ["error", "1tbs", { allowSingleLine: true }],
	},
});

export default [
	...await atf,
	...tailwind.configs["flat/recommended"],
	{
		rules: {
			"tailwindcss/no-custom-classname": "off",
		},
		settings: {
			tailwindcss: {
				callees: ["classnames", "clsx", "ctl"],
				config: "tailwind.config.ts",
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
];
