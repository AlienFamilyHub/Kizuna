<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useEventStore } from "../stores/eventStore";

const eventStore = useEventStore();
const eventData = computed(() => eventStore.eventData);
const isLoading = ref(true);

onMounted(async () => {
	// 检查eventStore中是否已有数据，如果有则立即显示
	if (Object.keys(eventStore.eventData).length > 0) {
		isLoading.value = false;
	}

	const unlisten = await listen("home-event", (event) => {
		eventStore.setEventData(event.payload as ReturnData);
		isLoading.value = false;
	});

	onUnmounted(() => {
		if (unlisten) {
			unlisten();
		}
	});
});

function formatTimestamp(timestamp: number) {
	const date = new Date(timestamp * 1000);
	return date.toLocaleString();
}
</script>

<template>
	<div class="min-w-screen flex min-h-screen items-center justify-center bg-gray-100 p-6 dark:bg-gray-900">
		<div class="w-full max-w-2xl overflow-hidden rounded-lg bg-white shadow-lg dark:bg-gray-800">
			<div v-if="isLoading" class="space-y-6 p-8">
				<div class="h-8 w-3/4 animate-pulse rounded-md bg-gray-200 dark:bg-gray-700" />
				<div class="h-6 w-1/2 animate-pulse rounded-md bg-gray-200 dark:bg-gray-700" />
				<div class="h-32 w-full animate-pulse rounded-md bg-gray-200 dark:bg-gray-700" />
			</div>
			<div v-else class="divide-y divide-gray-200 dark:divide-gray-700">
				<!-- Program Info -->
				<div class="space-y-4 p-8">
					<div class="flex items-center space-x-6">
						<img v-if="eventData.icon" :src="`data:image/png;base64,${eventData.icon}`" alt="Program Icon" class="size-16 rounded-md">
						<div>
							<h1 class="mb-1 text-3xl font-light text-gray-900 dark:text-gray-100">
								{{ eventData.data?.window_name }}
							</h1>
							<p class="text-lg font-light text-gray-600 dark:text-gray-400">
								{{ eventData.data?.process }}
							</p>
						</div>
					</div>
				</div>

				<!-- Media Info -->
				<div v-if="eventData.data?.media" class="space-y-6 p-8">
					<div class="flex items-start space-x-6">
						<img
							v-if="eventData.AlbumThumbnail" :src="eventData.AlbumThumbnail"
							alt="Album Thumbnail" class="size-32 rounded-md object-cover"
						>
						<div class="flex-1 space-y-3">
							<h2 class="text-2xl font-medium text-gray-900 dark:text-gray-100">
								{{ eventData.data.media.title }}
							</h2>
							<p v-if="eventData.data.media.artist" class="text-xl font-light text-gray-700 dark:text-gray-300">
								{{ eventData.data.media.artist }}
							</p>
							<p v-if="eventData.data.media.AlbumTitle" class="text-lg font-light text-gray-600 dark:text-gray-400">
								{{ eventData.data.media.AlbumTitle }}
							</p>
							<p v-if="eventData.data.media.AlbumArtist" class="text-md text-gray-500 dark:text-gray-500">
								{{ eventData.data.media.AlbumArtist }}
							</p>
						</div>
					</div>
					<p v-if="eventData.data.media.SourceAppName" class="text-sm uppercase tracking-wide text-gray-400 dark:text-gray-500">
						{{ eventData.data.media.SourceAppName }}
					</p>
				</div>

				<!-- Timestamp -->
				<div class="bg-gray-50 p-4 dark:bg-gray-900">
					<p class="text-sm text-gray-400 dark:text-gray-500">
						更新时间: {{ formatTimestamp(eventData.data?.timestamp) }}
					</p>
				</div>
			</div>
		</div>
	</div>
</template>
