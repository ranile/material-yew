# Material Yew

*Material Yew* is a components library for [Yew framework](https://yew.rs/) which is a wrapper around [Material Web Components](https://github.com/material-components/material-web) exposing Yew components. All modern browsers are supported.

### Example

```rust
use material_yew::Button;
use yew::html;

html! {
    <Button label="Click me!" />
};
```

## Getting started

### Installation

`Cargo.toml`:
```toml
[dependencies]
material-yew = "*"
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

## Documentation

Full API documentation can be found [here](https://yew-material.web.app/docs/material_yew). 

## Contributing

1. Fork it (<https://github.com/hamza1311/material-yew>)
2. Create your feature branch (`git checkout -b your-feature-branch`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin your-feature-branch`)
5. Create a new Pull Request
