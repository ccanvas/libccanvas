`#[cfg(feature = "layout")]`

# ccanvas layout

Bindings to [`ccanvas-layout`](https://github.com/ccanvas/ccanvas-layout) - layout manager for ccanvas.

## Quickstart

To enable layout support for a component, simply enable feature `layout` in Cargo.toml for `libccanvas`. No code needs to be modifed.

When loaded in as a normal component, the component will have assess to the full screen. It is only when it is loaded in as a layout component it will be restricted to its own window.

For more info, check [`ccanvas-layout`](https://github.com/ccanvas/ccanvas-layout).

## How it works

If it is loaded in as a layout component, environment variable `USE_LAYOUT=1` should be set. Otherwise it is loaded in as a normal component.

### Behaviour

As mentioned, when spawned in as a normal component, the component will have access to the entire terminal, meaning it can set characters at any location.

If the component is spawned in as a layout component, then it will only have access to only the area within its own window area, meaning it cannot set characters outside its own window. And character (0,0) will be the top left character of the window. Note that this change is purely component-side, components that does not support layout will still have access to the entire terminal.

`client.term_size()` will return the size of the window, and `Subscription::ScreenResizes` will **not** give you the real terminal size, instead it will give you the window size.

### Implementation

When layout receives a screen resize event, it will recalculate the dimension and positions of each window. And set the variable of `!layout-allocated-rect` at each component to the new size.

To ensure that all components have finished rendering before another component with a lower priority renders, it will hold the event until all components within the layout have finished rendering. This is done by waiting for the `!layout-render-confirm` variable to be updated in each variable. The event will get passed to the next subscribed component only when all the components in the layout have confirmed rendering finished.
