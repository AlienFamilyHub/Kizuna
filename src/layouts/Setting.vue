<script setup>
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

const toast = useToast();
const config = ref({
	server_config: {
		endpoint: "",
		token: "",
		report_time: 0,
	},
	rules: [],
});
onMounted(async () => {
	const response = await invoke("get_config");
	config.value = JSON.parse(response);
	// 初始化所有规则为折叠状态
	config.value.rules.forEach(rule => rule.collapsed = true);
});
const addRule = () => {
	config.value.rules.push({
		match_application: "",
		replace: {
			application: "",
			description: "",
		},
		collapsed: true,
	});
};
const removeRule = (index) => {
	config.value.rules.splice(index, 1);
};
const toggleRule = (index) => {
	config.value.rules[index].collapsed = !config.value.rules[index].collapsed;
};

const saveConfig = async () => {
	try {
		await invoke("save_config", { config: JSON.stringify(config.value) });
		toast.add({
			id: "save_config",
			title: "保存成功",
			description: "配置已成功保存",
			icon: "i-mingcute-check-line",
			timeout: 3000,
		});
	} catch (error) {
		console.error("保存配置时出错:", error);
		toast.add({
			id: "save_config_error",
			title: "保存失败",
			description: error.toString(),
			icon: "i-mingcute-close-line",
			timeout: 3000,
		});
	}
};
</script>

<template>
	<div class="flex min-h-screen items-center justify-center bg-base-200 lg:ml-60">
		<div class="card mx-auto w-full max-w-lg bg-base-100 p-6 shadow-2xl">
			<div class="card-body">
				<h1 class="card-title mb-4 text-2xl font-bold">
					设置
				</h1>
				<form @submit.prevent="saveConfig">
					<div class="form-control mb-4">
						<label class="label" for="endpoint">
							<span class="label-text">服务器端点</span>
						</label>
						<input
							id="endpoint" v-model="config.server_config.endpoint" class="input input-bordered w-full"
							type="text" placeholder="服务器端点"
						>
					</div>
					<div class="form-control mb-4">
						<label class="label" for="token">
							<span class="label-text">令牌</span>
						</label>
						<input
							id="token" v-model="config.server_config.token" class="input input-bordered w-full"
							type="text" placeholder="令牌"
						>
					</div>
					<div class="form-control mb-4">
						<label class="label" for="report_time">
							<span class="label-text">报告时间（秒）</span>
						</label>
						<input
							id="report_time" v-model.number="config.server_config.report_time"
							class="input input-bordered w-full" type="number" placeholder="报告时间"
						>
					</div>
					<div v-for="(rule, index) in config.rules" :key="index" class="form-control mb-4">
						<div class="flex items-center justify-between">
							<span>规则 {{ rule.match_application }}</span>
							<button type="button" class="btn btn-sm" @click="toggleRule(index)">
								{{ rule.collapsed ? '展开' : '折叠' }}
							</button>
						</div>
						<div v-if="!rule.collapsed">
							<label class="label" :for="`match_application_${index}`">
								<span class="label-text">匹配应用程序</span>
							</label>
							<input
								:id="`match_application_${index}`" v-model="rule.match_application"
								class="input input-bordered w-full" type="text" placeholder="匹配应用程序"
							>
							<label class="label" :for="`application_${index}`">
								<span class="label-text">替换应用程序</span>
							</label>
							<input
								:id="`application_${index}`" v-model="rule.replace.application"
								class="input input-bordered w-full" type="text" placeholder="替换应用程序"
							>
							<label class="label" :for="`description_${index}`">
								<span class="label-text">描述</span>
							</label>
							<input
								:id="`description_${index}`" v-model="rule.replace.description"
								class="input input-bordered w-full" type="text" placeholder="描述"
							>
							<div class="flex justify-center">
								<button type="button" class="btn btn-error mt-2 w-full" @click="removeRule(index)">
									删除规则
								</button>
							</div>
						</div>
					</div>
					<div class="form-control mb-4">
						<button type="button" class="btn btn-secondary" @click="addRule">
							添加规则
						</button>
					</div>
					<div class="form-control mt-6">
						<button class="btn btn-primary w-full" type="submit">
							保存
						</button>
					</div>
				</form>
			</div>
		</div>
	</div>
</template>
