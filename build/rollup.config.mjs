// rollup.config.mjs
import nodeResolve from "@rollup/plugin-node-resolve";

/** @type {import('rollup').RollupOptions} */
const config = {
    input: [
        "./rollup_inputs/button.js",
        "@material/web/radio/radio.js",
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