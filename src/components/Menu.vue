<template>
    <el-menu
        default-active="1"
        class="menu"
    >
     <el-sub-menu index="1">
            <template #title>
            <el-icon><img src="../assets/douyin.svg" width="16" height="16"></el-icon>
            <span>抖音</span>
            </template>
            <el-menu-item index="1-1">单个视频下载</el-menu-item>
            <el-menu-item index="1-2">用户所有视频</el-menu-item>      
    </el-sub-menu>

    <el-sub-menu index="2">
            <template #title>
            <el-icon><img src="../assets/bilibili.svg" width="16" height="16"></el-icon>
            <span>B站</span>
            </template>
    </el-sub-menu>

  </el-menu>

</template>

<script lang="ts" setup>
import { ref } from 'vue'
import {
  Document,
  Menu as IconMenu,
  Download,
  Setting,
} from '@element-plus/icons-vue'

const isCollapse = ref(true)
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
</script>

<style>
.el-menu-vertical-demo:not(.el-menu--collapse) {
  width: 200px;
  min-height: 400px;
}
</style>
