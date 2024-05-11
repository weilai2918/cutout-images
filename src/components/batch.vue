<template>
    <div style="padding:10px">
        <div style="display: flex;justify-content: space-between;">
            <div>
                <el-button type="primary" @click="selectImages">选择多张图片</el-button>
                <el-button type="success" @click="matting">开始批量抠图</el-button>
            </div>

            <el-button type="danger" @click="clearList">清空列表</el-button>
        </div>
        <div style="margin-top: 10px;">
            <el-space wrap>
                <el-card v-loading="img.loading" padding="0px" margin="0px" v-for="img in imageList" class="box-card"
                    style="width: 135px;height: 135px;">
                    <el-image style="width: 95px;height: 95px;" :src="img.image_base64" :fit="mode" />
                </el-card>
            </el-space>
        </div>
    </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

const mode = "contain";

const imageList = ref([]);

const selectImages = () => {
    invoke('open_images').then(res => {
        console.log(res)
        let list = imageList.value;
        for (let index in res) {
            res[index].loading = false;
            list.push(res[index]);
        }
        imageList.value = list;
    })
}


const matting = () => {
    let listLoad = imageList.value;
    for(let index in listLoad){
        listLoad[index].loading = true;
    }
    imageList.value = listLoad;

    let list = imageList.value;
    for (let index in list) {
        invoke('matting_image', { filePath: list[index].image_path }).then(res => {
            console.log(res);
            list[index].image_base64 = res.image_base64
            list[index].loading = false;
            imageList.value = list;
        }, err => {
            list[index].loading = false;
            imageList.value = list;
        })
    }

}

const clearList = () => {
    imageList.value = [];
}

</script>

<style>
.box-card {
    padding: 0px;
    margin: 0px;
}
</style>