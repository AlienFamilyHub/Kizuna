<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed, nextTick, ref, watch } from "vue";
import { useLogsStore } from "../stores/logsStore";

const toast = useToast();
const logsStore = useLogsStore();
const logs = computed(() => logsStore.logs);

const logContainer = ref<HTMLElement | null>(null);
const autoScroll = ref(true);

const handleScroll = (event: Event) => {
	const container = event.target as HTMLElement;
	const isAtBottom = container.scrollTop + container.clientHeight >= container.scrollHeight - 50;
	autoScroll.value = isAtBottom;
};

const scrollToBottom = () => {
	if (logContainer.value && autoScroll.value) {
		logContainer.value.scrollTop = logContainer.value.scrollHeight;
	}
};

const openLogDirectory = () => {
	invoke("open_log_directory");
	toast.add({
		id: "open_log_directory",
		title: "日志目录已打开",
		description: "已在文件管理器中打开日志目录",
		icon: "i-mingcute-folder-open-line",
		timeout: 3000,
	});
};

const clearLogs = () => {
	logsStore.clearLogs();
	toast.add({
		id: "clear_logs",
		title: "日志已清空",
		description: "所有日志已被清除",
		icon: "i-mingcute-delete-line",
		timeout: 3000,
	});
};

watch(logs, () => {
	nextTick(() => {
		scrollToBottom();
	});
}, { deep: true });
</script>

<template>
	<div class="p-4">
		<div class="mb-4 flex items-center justify-between">
			<h1 class="text-2xl font-bold dark:text-white">
				Logs
			</h1>
			<div class="flex gap-2">
				<button class="btn btn-error" @click="clearLogs">
					清空日志
				</button>
				<button class="btn btn-neutral dark:btn-primary dark:text-white" @click="openLogDirectory">
					日志目录
				</button>
			</div>
		</div>
		<div
			ref="logContainer"
			class="h-80 overflow-y-auto rounded-lg bg-gray-100 p-4 dark:bg-gray-800"
			@scroll="handleScroll"
		>
			<div
				v-for="(log, index) in logs"
				:key="index"
				class="mb-1 text-sm text-gray-800 dark:text-gray-200"
			>
				{{ log }}
			</div>
		</div>
	</div>
</template>
