<template>
    <div style="padding: 10px;">
        <div style="display: flex;justify-content: space-between;">
            <el-input disabled style="margin-right: 10px;" v-model="input" placeholder="请选择抠图保存位置" />
            <el-button type="primary">选择位置</el-button>
        </div>
        <div style="display: flex;justify-content: space-between;margin-top: 20px;">
            <el-input style="margin-right: 10px;" v-model="input" placeholder="请输入抠图秘钥" />
            <el-button type="primary">保存秘钥</el-button>
        </div>
    </div>

</template>

<script setup>

</script>


<style>

</style>
