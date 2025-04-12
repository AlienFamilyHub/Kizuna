<script setup>
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

const toast = useToast();
const config = ref({
	server_config: {
		endpoint: "",
		token: "",
		report_time: 0,
		report_smtc: true,
		skip_smtc_cover: false,
		upload_smtc_cover: false,
		log_base64: false,
		s3_config: {
			s3_enable: false,
			bucket_url: "",
			endpoint: "",
			region: "",
			bucket_name: "",
			access_key: "",
			secret_key: "",
			custom_url: "",
		},
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
					<div class="form-control mb-4">
						<label class="label cursor-pointer">
							<span class="label-text">上报SMTC信息</span>
							<input v-model="config.server_config.report_smtc" type="checkbox" class="checkbox">
						</label>
					</div>
					<div class="form-control mb-4">
						<label class="label cursor-pointer">
							<span class="label-text">是否跳过上报SMTC提供的音频封面</span>
							<input v-model="config.server_config.skip_smtc_cover" type="checkbox" class="checkbox">
						</label>
					</div>
					<div class="form-control mb-4">
						<label class="label cursor-pointer">
							<span class="label-text">是否将SMTC封面上传到第三方服务器</span>
							<input v-model="config.server_config.upload_smtc_cover" type="checkbox" class="checkbox">
						</label>
					</div>

					<div v-if="config.server_config.upload_smtc_cover" class="form-control mb-4">
						<label class="label cursor-pointer">
							<span class="label-text">启用S3功能</span>
							<input v-model="config.server_config.s3_config.s3_enable" type="checkbox" class="checkbox">
						</label>
					</div>
					<div v-if="config.server_config.upload_smtc_cover && config.server_config.s3_config.s3_enable" class="form-control mb-4">
						<label class="label" for="s3_bucket_url">
							<span class="label-text">S3桶URL</span>
						</label>
						<input
							id="s3_bucket_url" v-model="config.server_config.s3_config.bucket_url"
							class="input input-bordered w-full" type="text" placeholder="https://example.com"
						>
						<p class="mt-1 text-xs text-gray-500">
							可设置自定义URL路径格式，支持变量：{year}年、{month}月、{day}日、{hash}哈希值等，例如：{S3桶URL}/images/{year}/{month}/{day}/{hash}.webp
						</p>
					</div>
					<div v-if="config.server_config.upload_smtc_cover && config.server_config.s3_config.s3_enable" class="form-control mb-4">
						<label class="label" for="s3_endpoint">
							<span class="label-text">S3端点（可选）</span>
						</label>
						<input
							id="s3_endpoint" v-model="config.server_config.s3_config.endpoint"
							class="input input-bordered w-full" type="text" placeholder="https://s3.amazonaws.com"
						>
						<p class="mt-1 text-xs text-gray-500">
							自定义S3端点，留空则使用标准AWS端点
						</p>
					</div>
					<div v-if="config.server_config.upload_smtc_cover && config.server_config.s3_config.s3_enable" class="form-control mb-4">
						<label class="label" for="s3_region">
							<span class="label-text">S3区域（可选）</span>
						</label>
						<input
							id="s3_region" v-model="config.server_config.s3_config.region"
							class="input input-bordered w-full" type="text" placeholder="us-east-1"
						>
						<p class="mt-1 text-xs text-gray-500">
							S3区域，如us-east-1、ap-northeast-1等，留空则使用默认区域
						</p>
					</div>
					<div v-if="config.server_config.upload_smtc_cover && config.server_config.s3_config.s3_enable" class="form-control mb-4">
						<label class="label" for="s3_bucket_name">
							<span class="label-text">S3桶名称（可选）</span>
						</label>
						<input
							id="s3_bucket_name" v-model="config.server_config.s3_config.bucket_name"
							class="input input-bordered w-full" type="text" placeholder="my-bucket"
						>
						<p class="mt-1 text-xs text-gray-500">
							S3桶名称，留空则尝试从桶URL中提取
						</p>
					</div>
					<div v-if="config.server_config.upload_smtc_cover && config.server_config.s3_config.s3_enable" class="form-control mb-4">
						<label class="label" for="s3_access_key">
							<span class="label-text">S3访问密钥（可选）</span>
						</label>
						<input
							id="s3_access_key" v-model="config.server_config.s3_config.access_key"
							class="input input-bordered w-full" type="text" placeholder="S3 Access Key"
						>
					</div>
					<div
						v-if="config.server_config.upload_smtc_cover && config.server_config.s3_config.s3_enable && config.server_config.s3_config.access_key"
						class="form-control mb-4"
					>
						<label class="label" for="s3_secret_key">
							<span class="label-text">S3密钥（可选）</span>
						</label>
						<input
							id="s3_secret_key" v-model="config.server_config.s3_config.secret_key"
							class="input input-bordered w-full" type="password" placeholder="S3 Secret Key"
						>
					</div>
					<div v-if="config.server_config.upload_smtc_cover && config.server_config.s3_config.s3_enable" class="form-control mb-4">
						<label class="label" for="s3_custom_url">
							<span class="label-text">自定义URL（可选）</span>
						</label>
						<input
							id="s3_custom_url" v-model="config.server_config.s3_config.custom_url"
							class="input input-bordered w-full" type="text" placeholder="https://custom.example.com"
						>
						<p class="mt-1 text-xs text-gray-500">
							自定义访问URL，留空则使用标准S3 URL
						</p>
					</div>
					<div class="form-control mb-4">
						<label class="label cursor-pointer">
							<span class="label-text">base64信息写入日志</span>
							<input v-model="config.server_config.log_base64" type="checkbox" class="checkbox">
						</label>
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
