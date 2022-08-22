<script setup lang="ts">
import { GithubFilled } from '@ant-design/icons-vue';
import { shell } from '@tauri-apps/api'

const onClick = () => {
    shell.open('https://github.com/ClassmateLin/video-downloader')
}
</script>

<template>
    <div class="content">
        <el-col :span="12">
            <el-row>
               <el-button @click="onClick"><el-icon class="ali-icon-shipinxiazai"></el-icon>Video downloader</el-button>
            </el-row>
            <el-row>
                <p>基于rust+ts+vue开发的一款视频下载器, 支持多个第三方平台视频下载。</p>
            </el-row>
        </el-col>
    </div>
</template>

<style scoped>
.content {
    height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
}


</style>
