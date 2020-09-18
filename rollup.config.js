import { nodeResolve } from '@rollup/plugin-node-resolve';
import { terser } from "rollup-plugin-terser";

export default {
    input: "@material/mwc-button",
    plugins: [nodeResolve(), /*terser()*/],
    output: {
        format: "es",
        file: "build/button.js",
        inlineDynamicImports: true,
    },
}
