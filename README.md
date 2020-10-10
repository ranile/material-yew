# Yew material components

Yew Material components is a components library for [Yew framework](https://github.com/yewstack/yew/) which is a wrapper around [Material Web Components](https://github.com/material-components/material-components-web-components) exposing Yew components. All modern browsers are supported. There is no support for polyfills required by Internet Explorer 11.

### Example

```rust
use yew_material_components::MatButton;
use yew::html;

html! {
    <MatButton label="Click me!" />
}
```

## Getting started
### Installation

Currently, this library can only be imported via git. In the future, it'll be available from crates.io.
`Cargo.toml`:
```toml
[dependencies]
material-yew-components = { git = "https://github.com/hamza1311/material-yew-components/", branch = "master" }
```
Material icons and a Material font can also be imported for full functionality.
`index.html`:
```html
<link href="https://fonts.googleapis.com/css?family=Roboto:300,400,500" rel="stylesheet">
<link href="https://fonts.googleapis.com/css?family=Material+Icons&display=block" rel="stylesheet">
```

## Theming

These components respect the theming applied to Material Web Components using stylesheets.Learn about how to theme Material Web Components

## Documentation

Full API documentation can be found here. Demos of components can be found here.

## Contributing

Your contributions are welcome.
