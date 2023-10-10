// rollup.config.mjs
import nodeResolve from "@rollup/plugin-node-resolve";

/** @type {import('rollup').RollupOptions} */
const config = {
    input: [
        "./rollup_inputs/button.js"
    ],
    plugins: [nodeResolve()],
    output: {
        dir: `./dist`,
        chunkFileNames: "[name].js",
        banner: 'export const __dummy_loader = () => {};\n',
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