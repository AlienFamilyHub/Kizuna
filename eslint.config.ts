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
		settings: {
			tailwindcss: {
				// These are the default values but feel free to customize
				callees: ["classnames", "clsx", "ctl"],
				config: "tailwind.config.ts", // returned from `loadConfig()` utility if not provided
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
				tags: [], // can be set to e.g. ['tw'] for use in tw`bg-blue`
				classRegex: "^class(Name)?$", // can be modified to support custom attributes. E.g. "^tw$" for `twin.macro`
			},
		},
	},
];
