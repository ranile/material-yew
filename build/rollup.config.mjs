// rollup.config.mjs
import nodeResolve from "@rollup/plugin-node-resolve";

/** @type {import('rollup').RollupOptions} */
const config = {
    input: [
        "@material/web/button/elevated-button.js",
        "@material/web/button/filled-button.js",
        "@material/web/button/filled-tonal-button.js",
        "@material/web/button/outlined-button.js",
        "@material/web/button/text-button.js",
    ],
    plugins: [nodeResolve()],
    output: {
        dir: `./dist`,
        chunkFileNames: "[name].js",
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