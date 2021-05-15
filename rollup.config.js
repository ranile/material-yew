import { nodeResolve } from '@rollup/plugin-node-resolve';
import { terser } from "rollup-plugin-terser";

const COMPONENTS = [
    "button",
    "circular-progress",
    "checkbox",
    "circular-progress-four-color",
    "drawer",
    "top-app-bar",
    "icon-button",
    "fab",
    "formfield",
    "linear-progress",
    "icon",
    "radio",
    "switch",
    "top-app-bar-fixed",
    "dialog",
    "list",
    "list/mwc-list-item",
    "list/mwc-check-list-item",
    "list/mwc-radio-list-item",
    "icon-button-toggle",
    "slider",
    "tab",
    "tab-bar",
    "snackbar",
    "textfield",
    "textarea",
    "select",
    "menu",
];

export default {
    input: COMPONENTS.map((component) => `@material/mwc-${component}`),
    plugins: [nodeResolve(), terser({ format: { comments: false } })],
    output: {
        dir: `yew-material/build`,
        chunkFileNames: "[name].js",
        manualChunks: (id, { getModuleInfo }) => {
            const info = getModuleInfo(id);
            if (info.importers.length <= 1) {
                // This will be inlined anyway
                return;
            }

            return "core";
        },
    },
};
