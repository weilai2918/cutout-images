<script>
import { invoke } from '@tauri-apps/api/tauri'

export default {
  data() {
    return {
      image_url:"",
      image_fit:"contain"
    }
  },
  methods: {
    openImage() {
      let that = this;
      invoke('open_image').then(base => {
        that.image_url = base;
      })
    }
  }
}

</script>

<template>
  <div>
    <div style="width: 100%;">
      <el-button @click="openImage" type="primary">打开文件</el-button>
    </div>
    <div>
      <el-image :fit="image_fit" style="width: 400px; height: 400px" :src="image_url">
        <template #error>
          <div class="image-slot">
            <div style="width: 400px;height: 400px;align-items: center;line-height: 400px;text-align: center;">请选择图片</div>
          </div>
        </template>
      </el-image>
    </div>
    
  </div>
</template>

<style scoped></style>
