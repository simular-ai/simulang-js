import { App, FocusPolicy, Visibility, AccessibilityTree, AriaRole, TraversalOrder } from '@simular-ai/simulang-js'

// Open simular.ai in the default browser and wait for it to load
App.defaultBrowser().open('https://simular.ai', FocusPolicy.Steal, Visibility.Show, true)

// Bind to the browser window and find the "About" link by role and name
const tree = AccessibilityTree.fromForeground()
const [link] = tree.find(TraversalOrder.BreadthFirst, AriaRole.Link, 'About', true, 1)

// Click it
if (link && link.refId != null) {
  tree.activate(link.refId)
} else {
  console.error('Could not find the "About" link')
}
