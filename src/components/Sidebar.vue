<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { onMounted, ref } from "vue";
import Avatar from "./Avatar.vue";

const curYear = new Date().getFullYear();
const backend_version = ref("");

onMounted(async () => {
	try {
		const version = await invoke("get_version");
		backend_version.value = version as string;
	} catch (error) {
		console.error("Failed to get version:", error);
	}
});

const sections = [
	{
		name: "主页",
		icon: "mingcute:home-2-line",
		path: "/",
	},
	{
		name: "日志",
		icon: "mingcute:align-left-line",
		path: "/log/",
	},
	{
		name: "设置",
		icon: "mingcute:settings-2-line",
		path: "/setting/",
	},
];

const openInBrowser = async (url: string) => {
	try {
		await openUrl(url);
	} catch (error) {
		console.error("Failed to open URL:", error);
	}
};
</script>

<template>
	<div class="drawer lg:drawer-open">
		<input id="my-drawer" type="checkbox" class="drawer-toggle">
		<div class="drawer-content">
			<label for="my-drawer" class="drawer-button lg:hidden">
				<Icon name="solar:sidebar-code-bold" class="m-4 size-8" />
			</label>
			<slot />
		</div>
		<div class="drawer-side">
			<label for="my-drawer" aria-label="close sidebar" class="drawer-overlay" />
			<aside
				class="fixed left-0 top-0 z-50 flex h-screen w-60 max-w-60 flex-col justify-between border-r border-gray-300 bg-gray-100 p-4 md:flex dark:border-gray-950 dark:bg-gray-800"
			>
				<div>
					<div class="flex flex-col items-center">
						<div class="mb-4 flex items-center text-base font-bold">
							<Avatar class="mr-2 size-10" />
							Kizuna
						</div>
						<div class="w-full max-w-56 lg:w-56">
							<ul class="menu w-full rounded-2xl bg-base-200 text-base">
								<li v-for="(item, index) in sections" :key="index" class="mb-2">
									<router-link
										:to="item.path" :class="{ active: $route.path === item.path }"
										class="mb-2 flex w-full items-center"
									>
										<Icon :name="item.icon" class="mr-2 size-6" /> {{ item.name }}
									</router-link>
								</li>
							</ul>
						</div>
					</div>
				</div>
				<div class="mt-4 text-center text-sm lg:text-base">
					<div class="divider mb-2" />
					© {{ curYear }} <a
						class="text-blue-500 hover:text-blue-700 dark:text-blue-300 dark:hover:text-blue-500"
						href="javascript:void(0)" @click="openInBrowser('https://github.com/TNXG/Kizuna')"
					>Kizuna</a>
					{{ backend_version }}
					<br>
					<p class="text-sm text-gray-700 dark:text-gray-300">
						Designed by <a
							href="javascript:void(0)"
							class="text-blue-500 hover:text-blue-700 dark:text-blue-300 dark:hover:text-blue-500"
							@click="openInBrowser('https://github.com/TNXG/tnxg-homepage')"
						>tnxg-homepage</a>
					</p>
				</div>
			</aside>
		</div>
	</div>
</template>
