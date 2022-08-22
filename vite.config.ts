import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver, AntDesignVueResolver } from 'unplugin-vue-components/resolvers'
import ElementPlus from 'unplugin-element-plus/vite'


export default defineConfig({
  plugins: [
    vue(),
    AutoImport({
      imports: ['vue', 'vue-router'],

      resolvers: [
        ElementPlusResolver(),
        AntDesignVueResolver(),
      ],
    }),
    ElementPlus({
    }),
    Components({
      
      resolvers: [
        ElementPlusResolver(),
        AntDesignVueResolver({
          importStyle: true,
          resolveIcons: true,
        }),
      ],
      

    }),

  ],
})
