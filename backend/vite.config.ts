/// <reference types="vitest" />
import { defineConfig } from 'vite';
import { resolve } from 'path';
import dts from 'vite-plugin-dts';

module.exports = defineConfig({
    plugins: [
        dts({
            insertTypesEntry: true,
        }),
    ],
    resolve: {
        alias: {
            "@": resolve(__dirname, "index.ts"),
        }
    },
    build: {
        minify: false,
        lib: {
            entry: resolve(__dirname, 'index.ts'),
            name: 'backend',
            fileName: (format) => `backend.${format}.js`
        }
    },
    test: {
        include: [
            resolve(__dirname, 'api_tests/test.ts'),
        ]
    }
});