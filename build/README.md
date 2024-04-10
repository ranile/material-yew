# Component generator

Generates Rust wrapper for a component from material-web.

## Updating material web submodule

1. Update the submodule to the latest version: `git submodule update --recursive --remote build/material-web`
2. Checkout the new tag: `cd build/material-web && git checkout v1.4.0`
3. Update the submodule reference in the main repository: `git add build/material-web && git commit -m "pin build/material web to v1.4.0"`
4. Update build/package.json with the tag used in step 2
5. Run `npm install` in the build directory
6. Run `npm run build` in the build directory

> **Note**: The submodule is currently pinned at tag v1.4.0.