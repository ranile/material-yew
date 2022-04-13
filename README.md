# Material Yew

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

*Material Yew* is a components library for [Yew framework](https://yew.rs/) which is a wrapper around [Material Web Components](https://github.com/material-components/material-components-web-components) exposing Yew components. All modern browsers are supported. There is no support for polyfills required by Internet Explorer 11.

### Example

```rust
use material_yew::MatButton;
use yew::html;

html! {
    <MatButton label="Click me!" />
};
```

## Getting started

### Installation

`Cargo.toml`:
```toml
[dependencies]
material-yew = "0.1"
```

Material icons and a Material font can also be imported for full functionality.  
`index.html`:
```html
<link href="https://fonts.googleapis.com/css?family=Roboto:300,400,500" rel="stylesheet">
<link href="https://fonts.googleapis.com/css?family=Material+Icons&display=block" rel="stylesheet">
```

It's also important to note that you need `viewport` `meta` tag for the Material Components to be responsive.
```html
<meta name="viewport" content="width=device-width, initial-scale=1.0">
```

### Feature flags

Following are all the cargo features available (each feature corresponds to its respective component):

* `button`
* `circular-progress`
* `checkbox`
* `circular-progress-four-color`
* `drawer`
* `top-app-bar`
* `icon-button`
* `fab`
* `formfield`
* `linear-progress`
* `icon`
* `radio`
* `switch`
* `top-app-bar-fixed`
* `dialog`
* `list`
* `icon-button-toggle`
* `slider`
* `tabs`
* `snackbar`
* `textfield`
* `textarea`
* `select`
* `menu`

`full` feature enables all the components

## Theming

These components respect the theming applied to Material Web Components using stylesheets. [Learn about how to theme Material Web Components.](https://github.com/material-components/material-components-web-components/blob/master/docs/theming.md)

## Documentation

Full API documentation can be found [here](https://yew-material.web.app/docs/material_yew). Demos of components can be found [here](https://yew-material.web.app/components).

## Meta

Distributed under the Apache License 2.0 license. See `LICENSE` for more information.

[https://github.com/hamza1311/](https://github.com/hamza1311/)

## Contributing

Your contributions are welcome.
