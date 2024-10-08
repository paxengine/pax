# Sams Lab Journal

### 23 Dec, 2023: "ss/towards-dirty-dag" branch state and slot reintroduction.

The towards-dirty-dag branch works well for arbitrary nesting of for, if and
many other components/primitives, but is missing slots, some Z-index stuff, and scroller/
clipping. Overall I'm positive that this partial re-expansion approach is a
good one and should pave the way for complete dirty dag.

Some general changes and design choices:

 - ExpandedNode has received a lot more responsibilities. Most logic for
   modifying the tree, handling updates and rendering has been moved here.
	 The thought is that operations like "set_children" naturally should set either
	 themselves or their containing component as their new containing component, and
	 their parent to themselves.
 - Nodes are expanded immediately on creation, ie ExpandedNode::new creates an entire tree
 - Only the "recurse_update" method is expected to be called on the root ExpandedNode each
   frame. This both updates properties and re-expands subtrees iff the properties listened
	 to on a node-by-node basis needs it! (only if and for, so far, soon slot).
 - ExpandedNode has been moved to a separate module and the ExpandedNode
   children field has intentionally been set to private. The thought is
   that all modifications to be encapsulated though methods on the ExpandedNode struct.

If you'd like to understand it better I recommend to first look at the methods
in ExpandedNode, and then on for example in conditional.rs "recompute_children
and update methods.

Below are some thoughts around how to reintroduce slot when the Expanded node
tree isn't re-expanded each frame, but sub-trees are instead surgically
updated when a property that requires an update triggers.

Suppose this is the state of the ExpandedNode tree: (v chain denoting that the
stacker owns this separate "shadow" tree disconnected from the main one).

 root:  ExpandedNode (ComponentInstance)
 │
 │
 └───── stacker: ExpandedNode( ComponentInstance)
        │
        │
        └─── ExpandedNode (Repeat) ...
        v    │
        v    │
        v    └────────── ExpandedNode (SlotInstance) INDEX: 3
        v
        v
        stacker: shadowtree (from which slot_children are derived)
        │ 
        └──── ExpandedNode (RectangleInstance)
The           │        
Shadow tree   │
Is updated    ExpandedNode (RepeatInstance) for 0..4
by the stac-  │  
er each frame │   
(ie reacts to └───── ExpandedNode (Rectangle)
changes in
properties and possibly re-expands children.
This tree needs to NOT fire mount or dismount
or native patch events.

However when a part of this shadow tree is mounted in a slot, mount needs to be
fired and updates triggered from the stacker shadow tree needs to send native
patches. Mount ALSO needs to be fired when the stacker updates the slot_children tree.

Current thought: introduce an "attached" property to ExpandedNode that is set
to true recursively when children are attached and it's parent is attached.
This allows for the shadow tree to update without firing events (only fire
if attached), without effecting the expansion of sub-trees that are active
(attached).

Something to think about: What is the expected behaviour if two slots reference
the same ExpandedNode? (ie slot(0) exists twice in a component). Long term it might
be nice to allow a component to for example be rendered twice with the same underlying
ExpandedNode. What would this require? Or maybe just disallow this.


### Slot children and shadow trees,

When parent stacker is updating slot_children, the bounds are being
calculated wrongly. need to update from the reference of the slot
itself. how do do this but still update the layout of the tree?
maybe layout calc should be another pass than the "should recompute
children" pass, done directly before rendering on the tree itself.

- the idea of "attached" needs to stay around, to mange expansions of sub-trees
  that need to fire on-mount/dismount events
- merge update and recompute children (one is simply always calling the other if needed)
- everything else than "update_children" is as separate passes. (compute_tab is atm what is causing a problem).

For slot this results in:
- update_children of stacker both updates the shadow tree and the main component
  tree (but attached limits events from shadow)

### How should canvas layer indexing work?

If the entire expanded node tree has no repeat/conditionals, the canvas index of
each node can be computed once and never changes. With repeat and conditionals
however, indices will need to be shifted around to allow for arbitrary
insertions of native/canvas layers between existing ones.

Some options:
- destroy elements and recreate them whenever it's canvas index changes. Keep
  track of this engine side. This probably is pretty expensive if a half of all
  rendered things need to be shifted in the same frame.
- do some fancy stuff to allow direct "shifting" of layers by some native
  message by the conditional/repeat element (this would require revamping how
  native elements keep track of their target layer - possibly by some ID passed
  from the chassi?)
- is it possible to figure out the maximum number of layers that will be needed
  at compile time? since the source for repeat indices can be arbitrarily
  changed to any length at runtime, I don't think it is.

Go for the first one initially?

Sending of create/destroy is done in mount/dismount. This means that a node
changing layer would need to dismount and mount again with a different ID
the way things are structured at the moment. Could a new layer ID instead be
introduced as part of an update patch somehow? That might be simpler (and allow
for movement of native elems instead of destruction and creation?).

### Ordering problems when rendering native and canvas elements causes flickering

Currently, native and canvas elements are rendered using two APIs:

In the canvas case, individual chassis provide an implementation of the RenderContext
trait from the piet library, exposing methods such as draw_path(path, color,
fill, ..). This is expected to be re-run every frame with no state.

Native elements on the other hand are created, destroyed or updated
by individual native messages only when they change.

Now to the problem: In the engine tick method I would like to do something like
this:

1. calculate updates to ExpandedNode graph
2. re-calculate occlusion indices for all nodes, and append OcclusionUpdate
   messages to the message queue for native elements.
3. draw non-native elements to canvas.
4. send messages back to chassi for processing of native elements

But this doesn't really work. The problem is that the updated occlusion index
for some of the nodes that are rendered to a canvas might exceed the currently
available number of canvas layers (since the number of layers are first updated
when while the native messages are processed chassi side). I think this requires
doing something like this (reordering of last list):


1. calculate updates to ExpandedNode graph
3. draw non-native elements to canvas (before occlusion indices are updated).
2. re-calculate occlusion indices for all nodes, and append OcclusionUpdate
   messages to the message queue for native elements.
4. send messages back to chassi for processing of native elements

Which makes the text below canvas elements flicker for a frame, before the
canvas layer is "fixed" next frame.

Some ideas for a solution:

- Buffer native messages one frame, to "wait" for  updates on the canvas (this
  doesn't feel good as it introduces a one-frame delay).
- Let the introduction of new occlusion layers happen at canvas draw time by
  replacing the HashMap<RenderContext> in draws with a struct that creates new occlusion
  layers (and render-contexts) on the fly whenever a too-large of an index is
  used.
- Create a NativeMessage trait, that is implemented by a chassi and exposes
  direct calls for adding, updating and deleting native elements (even if run
  as javascript this should be possible, since the RenderContext piet trait is
  doing something similar for canvas drawing)

I think I like the third one more if it's not too hard to do. Mostly because
it makes native rendering more similar to the way canvas rendering is handled.
