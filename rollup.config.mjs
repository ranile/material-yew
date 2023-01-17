import { nodeResolve } from '@rollup/plugin-node-resolve';
import { terser } from "rollup-plugin-terser";

const COMPONENTS = [
    "button",
    "checkbox",
    "circular-progress",
    "circular-progress-four-color",
    "dialog",
    "drawer",
    "fab",
    "formfield",
    "icon-button-toggle",
    "icon-button",
    "icon",
    "linear-progress",
    "list",
    "list/mwc-list-item",
    "list/mwc-check-list-item",
    "list/mwc-radio-list-item",
    "menu",
    "radio",
    "select",
    "slider",
    "snackbar",
    "switch",
    "tab-bar",
    "tab",
    "textarea",
    "textfield",
    "top-app-bar-fixed",
    "top-app-bar",
];

export default {
    input: COMPONENTS.map((component) => `@material/mwc-${component}`),
    plugins: [nodeResolve(), terser({ format: { comments: false } })],
    output: {
        dir: `material-yew/build`,
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
