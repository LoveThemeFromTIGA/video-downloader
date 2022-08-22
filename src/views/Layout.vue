<script setup lang="ts">
</script>

<template>
    <div class="layout">
    <el-container>
      <el-header align="center"></el-header>
      <el-container>
        <el-aside width="auto">
            <Menu></Menu>
        </el-aside>
        <el-container>
          <el-main height="80%">
                <router-view v-slot="{ Component }">
                    <keep-alive>
                        <component :is="Component" :key="$route.name" v-if="$route.meta.keepAlive" />
                    </keep-alive>
                    <component :is="Component" :key="$route.name" v-if="!$route.meta.keepAlive" />
                </router-view>
          </el-main>
          <el-footer>
          </el-footer>
        </el-container>
      </el-container>
    </el-container>
  </div>
</template>


