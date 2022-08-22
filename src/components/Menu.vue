<template>
    <el-menu
        router
        default-active="1"
        class="menu"
    >
    <el-menu-item index="/"><el-icon class="ali-icon-shipinxiazai"></el-icon>Video downloader</el-menu-item>
     <el-sub-menu index="1">
            <template #title>
            <el-icon class="ali-icon-douyin"></el-icon>
            <span>抖音</span>
            </template>
            <el-menu-item index="/douyin/single">分享链接下载</el-menu-item>
            <el-menu-item index="/douyin/muplite">用户主页下载</el-menu-item>      
    </el-sub-menu>

    <el-sub-menu index="2">
            <template #title>
            <el-icon class="ali-icon-bilibili"></el-icon>
            <span>B站</span>
            </template>
    </el-sub-menu>

  </el-menu>

</template>

<script lang="ts" setup>
import { DownCircleOutlined } from '@ant-design/icons-vue';
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
