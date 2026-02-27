<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, LogicalSize, Window } from '@tauri-apps/api/window';
import { Webview } from '@tauri-apps/api/webview';
import { open } from '@tauri-apps/plugin-dialog';
import injectedScript from './injected/injected.js?raw';

const aclMessage = ref('Waiting for restricted command...');
const aclColor = ref('black');
const scriptMessage = ref('Waiting for script load...');
const scriptColor = ref('black');
const dialogMessage = ref('Waiting for file selection...');

// 1. ACL Test
const runAclTest = () => {
    invoke('plugin:restricted-plugin|restricted_command')
        .then((msg: any) => {
            console.log("ACL Success:", msg);
            aclMessage.value = msg;
            aclColor.value = 'green';
        })
        .catch((err: any) => {
            console.error("ACL Error:", err);
            aclMessage.value = 'Error: ' + err;
            aclColor.value = 'red';
        });
};

runAclTest();

// 2. Script Test
const loadScript = () => {
    const script = document.createElement('script');
    script.src = 'https://cdnjs.cloudflare.com/ajax/libs/jquery/3.7.1/jquery.min.js';
    script.onload = () => {
        // @ts-ignore
        scriptMessage.value = 'jQuery loaded successfully! Version: ' + (window.jQuery ? window.jQuery.fn.jquery : 'unknown');
        scriptColor.value = 'green';
    };
    script.onerror = () => {
        scriptMessage.value = 'Failed to load script (CSP might be blocking it).';
        scriptColor.value = 'red';
    };
    document.head.appendChild(script);
};

// 3. Window Test
const openWindow = () => {
    const appWindow = new Window('tauri-app-window');
    appWindow.once('tauri://created', function (payload) {
        console.log("Window created:", payload);
        const webview = new Webview(appWindow, 'tauri-app-webview', {
            url: 'https://v2.tauri.app/',
            x: 0,
            y: 0,
            width: 800,
            height: 600
        });

        webview.once('tauri://created', function (args) {
            console.log("Webview created:", args);
            appWindow.onResized(({ payload: size }) => {
                console.log('Window resized', size);
                webview.setSize(new LogicalSize(size.width, size.height))
            })
            // Test dynamic JS injection
            setInterval(() => {
                console.log(Date.now())
            }, 3000)
            setTimeout(() => {
                console.log("Injecting JS...");
                webview.eval(injectedScript)
                    .then(() => console.log("Injection success"))
                    .catch(e => console.error("Injection failed:", e));
            }, 10000);
        });

        webview.once('tauri://error', function (e) {
            console.error("Webview creation error:", e);
        });
    });

    appWindow.once('tauri://error', function (e) {
        console.error("Window creation error:", e);
    });
};

// 4. Dialog Test
const openDialog = () => {
    open({
        multiple: false,
        directory: false,
    }).then(res => {
        if (res) {
            dialogMessage.value = 'Selected: ' + res;
        } else {
            dialogMessage.value = 'User cancelled selection';
        }
    }).catch(err => {
        dialogMessage.value = 'Error: ' + err;
    });
};
</script>

<template>
    <div class="container">
        <h1>Full-Blooded Tauri Test (Vue 3)</h1>

        <h2>1. ACL Bypass Test</h2>
        <p :style="{ color: aclColor }">{{ aclMessage }}</p>

        <h2>2. External Script Test (CSP Bypass)</h2>
        <button @click="loadScript">Load External Script (jQuery)</button>
        <p :style="{ color: scriptColor }">{{ scriptMessage }}</p>

        <h2>3. Navigation Test (Scope Bypass)</h2>
        <p>Click below to open a new window with an external site.</p>
        <button @click="openWindow">Open tauri.app</button>

        <h2>4. Dialog Plugin Test</h2>
        <button @click="openDialog">Open File Dialog</button>
        <p>{{ dialogMessage }}</p>
    </div>
</template>

<style scoped>
.container {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    text-align: center;
    color: #2c3e50;
    margin-top: 60px;
}

button {
    margin: 10px;
    padding: 8px 16px;
    cursor: pointer;
}
</style>
