<template>
    <div>
        <div style="padding: 10px;">
            <el-button @click="openImage" type="primary">打开文件</el-button>
        </div>
        <div style="display: flex;justify-items: center;justify-content: center;">
            <el-image :fit="image_fit" style="width: 300px; height: 300px;margin: 40px;" :src="image_base64">
                <template #error>
                    <div class="image-slot">
                        <div
                            style="width: 300px;height: 300px;align-items: center;line-height: 300px;text-align: center;">
                            暂无图片
                        </div>
                    </div>
                </template>
            </el-image>
            <el-image v-loading="loading" :fit="image_fit" style="width: 300px; height: 300px;margin: 40px;"
                :src="matting_base64">
                <template #error>
                    <div class="image-slot">
                        <div
                            style="width: 300px;height: 300px;align-items: center;line-height: 300px;text-align: center;">
                            暂无图片
                        </div>
                    </div>
                </template>
            </el-image>
        </div>
        <div style="width: 100%;text-align: center;">
            <el-button :loading="loading" style="width: 120px;" @click="matting" type="error">抠图</el-button>
        </div>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri'
import { ElMessage } from 'element-plus'
export default {
    data() {
        return {
            loading: false,
            image_base64: "",
            image_path: "",
            matting_base64: "",
            matting_path: "",
            image_fit: "contain"
        }
    },
    methods: {
        openImage() {
            let that = this;
            invoke('open_image').then(res => {
                console.log(res)
                that.image_base64 = res.image_base64;
                that.image_path = res.image_path;
            })
        },
        matting() {
            let that = this;
            that.loading = true;
            console.log(this.image_path);
            invoke('matting_image', { filePath: this.image_path }).then(res => {
                console.log(res);
                ElMessage({
                    type: 'success',
                    showClose: true,
                    message: "处理完成",
                    center: true,
                })
                that.matting_base64 = res.image_base64;
                that.matting_path = res.image_path;
                that.loading = false;
            }, err => {
                ElMessage.error(err)
                that.loading = false;
            })
        }
    }
}

</script>

<style scoped></style>