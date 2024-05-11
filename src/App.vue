<template>
  <div class="common-layout">
    <el-container class="layout-container-demo">
      <el-aside width="100px" height="100%">
        <el-menu default-active="2" @open="handleOpen" @close="handleClose">
          <el-menu-item index="single" @click="openPage">
            <span>单图抠图</span>
          </el-menu-item>
          <el-menu-item index="batch" @click="openPage">
            <span>批量抠图</span>
          </el-menu-item>
          <el-menu-item index="setting" @click="openPage">
            <span>系统设置</span>
          </el-menu-item>
        </el-menu>
      </el-aside>
      <el-container>
        <!-- <el-header>
          <div style="background-color: aqua;height: 30px;width: 100%;"></div>
        </el-header> -->
        <el-main>
          <div>
            <Single v-show="page == 'single'"></Single>
            <Batch v-show="page == 'batch'"></Batch>
            <Setting v-show="page == 'setting'"></Setting>
          </div>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script setup>

import Single from './components/single.vue'
import Batch from './components/batch.vue'
import Setting from './components/setting.vue'
import { ref } from 'vue'

const page = ref("single")

const handleOpen = (key, keyPath) => {
  console.log(key, keyPath)
}
const handleClose = (key, keyPath) => {
  console.log(key, keyPath)
}

const openPage = (event) => {
  console.log(event)
  page.value = event.index;
  console.log(page)
}

</script>

<style>
* {
  margin: 0px;
  padding: 0px;
}

.layout-container-demo .el-aside {
  height: 100vh;
  background-color: #FFFFFF;
  border-right: #b9b9b9 1px solid;
}


.layout-container-demo .el-header {
  height: 30px;
  padding: 0px;
}

.layout-container-demo .el-main {
  padding: 0px;
}

.layout-container-demo .el-main {
  padding: 0px;
}

.layout-container-demo .el-footer {
  padding: 0px;
}
</style>
