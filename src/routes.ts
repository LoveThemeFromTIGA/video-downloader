import { createRouter, createWebHistory } from "vue-router";

const routes = [
    {
        path: "/",
        name: "Home",
        component: () => import("./views/Home.vue"),
        meta: {
            keepAlive: false,
        }
    },
    {
        path: "/douyin/single",
        name: "DouyinSignle",
        component: () => import("./views/DouyinSingle.vue"),
        meta: {
            keepAlive: false,
        }
    },
    {
        path: "/douyin/muplite",
        name: "DouyinMuplite",
        component: () => import("./views/DouyinMuplite.vue"),
        meta: {
            keepAlive: false,
        }
    },
];

export default createRouter({
    history: createWebHistory(),
    routes: routes,
});