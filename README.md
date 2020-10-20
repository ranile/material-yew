# Yew Material

Yew Material is a components library for [Yew framework](https://github.com/yewstack/yew/) which is a wrapper around [Material Web Components](https://github.com/material-components/material-components-web-components) exposing Yew components. All modern browsers are supported. There is no support for polyfills required by Internet Explorer 11.

### Example

```rust
use yew_material::MatButton;
use yew::html;

html! {
    <MatButton label="Click me!" />
}
```

## Getting started
### Installation

Currently, this library can only be imported via git. In the future, it'll be available from [crates.io](https://crates.io/). Cargo features are used to pick the components. See [features](#features)   
`Cargo.toml`:
```toml
[dependencies]
material-yew-components = { git = "https://github.com/hamza1311/material-yew-components/", branch = "master", features = ["full"] }
```
Material icons and a Material font can also be imported for full functionality.  
`index.html`:
```html
<link href="https://fonts.googleapis.com/css?family=Roboto:300,400,500" rel="stylesheet">
<link href="https://fonts.googleapis.com/css?family=Material+Icons&display=block" rel="stylesheet">
```

### Features

Following are all the cargo features available:

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

These components respect the theming applied to Material Web Components using stylesheets.Learn about how to theme Material Web Components

## Documentation

Full API documentation can be found [here](). Demos of components can be found [here](https://yew-material.web.app/).

## Contributing

Your contributions are welcome.
