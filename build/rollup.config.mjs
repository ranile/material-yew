// rollup.config.mjs
import nodeResolve from "@rollup/plugin-node-resolve";

/** @type {import('rollup').RollupOptions} */
const config = {
    input: [
        "./rollup_inputs/button.js",
        "./rollup_inputs/fab.js",
        "./rollup_inputs/chip.js",
        "./rollup_inputs/icon-button.js",
        "@material/web/checkbox/checkbox.js",
        "@material/web/radio/radio.js",
        "@material/web/menu/menu.js",
        "@material/web/menu/menu-item.js",
        "@material/web/menu/sub-menu.js",
    ],
    plugins: [nodeResolve()],
    output: {
        dir: `../md-web`,
        chunkFileNames: "[name].js",
        banner: (chunk) => {
            // if (chunk.name !== "core") {
            //
            // } else { return "" }
            return 'export const __dummy_loader = () => {};\n'
        },
        manualChunks: (id, { getModuleInfo }) => {
            const info = getModuleInfo(id);
            if (info.importers.length <= 1) {
                // This will be inlined anyway
                return;
            }

            return "core";
        },
    }
}

// noinspection JSUnusedGlobalSymbols
export default config;