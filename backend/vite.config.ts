/// <reference types="vitest" />
import { defineConfig } from 'vite';
import { resolve } from 'path';
import dts from 'vite-plugin-dts';
import checker from 'vite-plugin-checker'

module.exports = defineConfig({
    plugins: [
        dts({
            insertTypesEntry: true,
        }),
        // @ts-expect-error
        checker({
            typescript: true,
        }),
    ],
    resolve: {
        alias: {
            "@": resolve(__dirname, "index.ts"),
        }
    },
    build: {
        target: "esnext",
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
        ],
        testTimeout: 20000,
    }
});