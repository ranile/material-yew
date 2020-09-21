import { nodeResolve } from '@rollup/plugin-node-resolve';
import { terser } from "rollup-plugin-terser";
/*
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


export default [
    'button',
    'circular-progress',
    'checkbox',
    'circular-progress-four-color',
    'drawer',
    'top-app-bar',
    'icon-button',
    'fab'
].map((it) => generateInput(it))*/

export default {
    input: `main.js`,
    plugins: [nodeResolve(), terser()],
    output: {
        format: "es",
        file: `build/built-js.js`,
        inlineDynamicImports: true,
    },
}
