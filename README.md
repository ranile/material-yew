# Material Yew

[![Crates.io](https://img.shields.io/crates/v/material-yew)](https://crates.io/crates/material-yew)
[![API Docs](https://img.shields.io/docsrs/material-yew)](https://docs.rs/material-yew/)
[![Apache-2.0 licensed](https://img.shields.io/github/license/hamza1311/material-yew)](https://github.com/hamza1311/material-yew/blob/master/LICENSE.md)
[![Website](https://img.shields.io/website?url=https%3A%2F%2Fmaterial-yew.rm.rs%2F)](https://material-yew.rm.rs/)

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
material-yew = { version = "0.2", features = ["full"] }
```
or
```shell
cargo add material-yew --features full
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

These components respect the theming applied to Material Web Components using stylesheets. [Learn about how to theme Material Web Components.](https://github.com/material-components/material-web/blob/mwc/docs/theming.md)

## Documentation

Full API documentation can be found [here](https://yew-material.web.app/docs/material_yew). Demos of components can be found [here](https://yew-material.web.app/components).

## Contributing

1. Fork it (<https://github.com/hamza1311/material-yew>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
