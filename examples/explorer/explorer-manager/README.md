# Explorer manager

This component renders the frame/borders, loads in *only two* other components, as the last component (the file preview component) is loaded in by [explorer-dir](../explorer-dir) after it has done its init stuff.

## What this does

1. Watch for 'q' to exit when pressed.
2. Rerenders frame and updates the dimensions of where the other components should render when screen resizes.
3. Renders a "screen saver" when it gets too small, and captures all events so no other components gets it.
