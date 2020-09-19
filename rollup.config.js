import { nodeResolve } from '@rollup/plugin-node-resolve';
import { terser } from "rollup-plugin-terser";

const generateInput = (component, minify = false) => {
    const plugins = [nodeResolve()]
    if (minify) {
        plugins.push(terser())
    }

    return {
        input: `@material/mwc-${component}`,
        plugins,
        output: {
            format: "es",
            file: `build/${component}.js`,
            inlineDynamicImports: true,
        },
    }
}


export default ['button', 'circular-progress'].map((it) => generateInput(it))
