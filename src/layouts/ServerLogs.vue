<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed, ref, watchEffect } from "vue";
import { useLogsStore } from "../stores/logsStore";

const toast = useToast();
const logsStore = useLogsStore();
const logs = computed(() => logsStore.logs);

const logContainer = ref<HTMLElement | null>(null);

const scrollToBottom = () => {
	if (logContainer.value) {
		// Type assertion: assert that `logContainer.value` is an HTMLElement
		(logContainer.value as HTMLElement).scrollTop = (logContainer.value as HTMLElement).scrollHeight;
	}
};

watchEffect(() => {
	scrollToBottom();
});

const openLogDirectory = () => {
	invoke("open_log_directory");
	toast.add({
		id: 'open_log_directory',
		title: '日志目录已打开',
		description: '已在文件管理器中打开日志目录',
		icon: 'i-mingcute-folder-open-line',
		timeout: 3000
	})
};
</script>

<template>
	<div class="p-4">
		<div class="mb-4 flex items-center justify-between">
			<h1 class="text-2xl font-bold dark:text-white">
				Logs
			</h1>
			<button class="btn btn-neutral dark:btn-primary dark:text-white" @click="openLogDirectory">
				日志目录
			</button>
		</div>
		<div ref="logContainer" class="h-80 overflow-y-auto rounded-lg bg-gray-100 p-4 dark:bg-gray-800">
			<p v-for="(log, index) in logs" :key="index" class="mb-1 text-sm text-gray-800 dark:text-gray-200">
				{{ log }}
			</p>
		</div>
	</div>
</template>
