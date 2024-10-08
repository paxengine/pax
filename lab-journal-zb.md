# Lab Journal, zb

### Early design exploration, ca. Nov 2021
_Transferred to lab-journal-zb from pax-example/design-exploration.md in Oct 2023_

<Main>
    <SomeUnknownThing>
        ^ This tag-name will be lower-cased
        when parsed by browser
    </SomeUnknownThing>
    <AnotherThing>
        Should this just be XML?

        - Pro: parsing is easier. it's probably just a better tool.
        - Con: XML's brand sucks; html is universal
               and a better fit for mass appeal

        We totally lose the "fork the web" angle
        if we go with xml.
    </AnotherThing>
</Main>


<html>
    <stacker>
        <repeat data-list={{
            || -> Vec<Rc<PropertiesCoproduct />
            //... no.
                }}>
            <rectangle fill={{}}>
        </repeat>
    </stacker>
</html>

//is there a better, shorter name for `component`?
//it's overloaded with a different concept from game engines
//(the C in ECS) AND it would be ergonomically
//beneficial to have a shorter name for use in expressions,
//instead of `component.some_property`
//   Module, widget, entity, movie clip,
//   component,
//   ...


Alphabet of injectables:

`datum`: used by `repeat` to expose iterated data to expressions
        - How to reach across nested repeat frames? for i; for j

        -
`this`: (or `self`, `me`, `component`, etc.) used as a reference to
        the containing component and its properties / children
`




#template!
<stacker>
    <repeat data-list={{this.panels}}>
        |datum| {
            <rectangle fill={{Color.hsla(datum.bg_fill)}} />
        }
    </repeat>
</stacker>

#template-src!("./path/to/src.html")

#template-behind! //automatically load same_file_name.html


#template!
<stacker>
    @foreach (panel in panels) {
        <rectangle fill={{Color.hsla(panel.bg_fill)}} />
    }
</stacker>





#template!(
<stacker id="outer-stacker">
    @foreach (panel in panels) {
        <rectangle fill={{Color.hsla(panel.bg_fill)}} />
    }
</stacker>
)





// Properties:  can be inlined or declared alongside
#properties!(
    #outer-stacker {
        size: (Size::Percent(100.0),Size::Percent(100.0))
        transform: || {}
        direction:
        cells:
        gutter:
    }
)
// We really want language server aid here...

// What about in rust, outside of macro:

properties: [ //Vec<PropertiesCoproduct>
    #join!(#outer-stacker) { //automatically determine that this is a StackerProperties

    }
]


**If we commit to full custom parsers**, we could do a CSS-like
    syntax, handling literals as needed w/ parser —
and another parser for expressions.

That is, three total parsers:
    - CSS-like parser for property-value binding
    - Expression parser, including stream injection
    - Template parser, Blazor-like


// Properties:  can be inlined or declared alongside
```

#template!(
    <Stacker id="outer-stacker">
        <Rectangle id="rect-0" />
        <Rectangle id="rect-1" />
        <Rectangle id="rect-2" />
    </Stacker>
)


#properties!(
    #outer-stacker {
        size: 50px, (dash) => {
            dash.height
        }px, //expression syntax is JS-lambda-like,
        transform: {
            translate: 20px, x, //x is "don't care" i.e. "default"
            scale: 100%, 90%,
            anchor: 50%, 50%,
        }
        orientation: vertical, //enums are tricky.  single global keywords are most ergonomic
                               //but have obvious namespace collision risks.
                               //To start we could give judicious global real estate to 
                               //orientation.{vertical|horizontal}, toward.{top,right,bottom,left}
        cells: 10,
        gutter: 10px,
    }
    
    #rect-0 {
        
    }
)








#properties!(
    #outer-stacker {
        size: (50px, (dash) => {
            dash.height
        }px), //expression syntax is JS-lambda-like,
        transform: {
            translate: (20px, x); //x is "don't care" i.e. "default"
            scale: (100%, 90%);
            anchor: (50%, 50%),
        }
        orientation: vertical, //enums are tricky.  single global keywords are most ergonomic
                               //but have obvious namespace collision risks.
                               //To start we could give judicious global real estate to 
                               //orientation.{vertical|horizontal}, toward.{top,right,bottom,left}
        cells: 10,
        gutter: 10px,
    }
    
    #rect-0 {
        
    }
)




```

It's important to have auto-complete here.  We'll want to parse
(or duplicate/declare, in a manifest) the source typedefs




```rust
#template!(
    <Stacker id="outer-stacker">
        <Rectangle id="rect-0" width=@{num_clicks * 20} />
        <Rectangle id="rect-1" />
        <Rectangle id="rect-2" />
    </Stacker>
)

#properties!(
    #outer-stacker {
        
    }
)

```




#### Expressions — should be CEL-like

Snippet from: https://github.com/google/cel-spec/blob/7972b9076513e6a4bbd184f9d073db949ea53c65/README.md
```
// Condition
account.balance >= transaction.withdrawal
    || (account.overdraftProtection
    && account.overdraftLimit >= transaction.withdrawal  - account.balance)

// Object construction
common.GeoPoint{ latitude: 10.0, longitude: -5.5 }
```




### On polymorphic data + stack frames
_Date of authoring unknown; moved from pax-runtime/src/repeat.rs on Oct 31 2023_
---------------

To support polymorphic data <T> inside stack frames,
we need a `dyn SomeTrait` contract that stack frame data
can adhere to (instead of arbitrary `T`)

ex. `repeat` element stackframe data:
{
    index: usize,
    datum: T
}

We could have any stack frame abide by this contract:

StackFrameData<T> {
    get_index() -> usize;
    get_datum() -> Box<dyn T>;
}
...but how does the consumer know it's dealing with `T`?  Where does `T` come from?

Ultimately, it's userland.  E.g. consider the user-provided data:
cats = [{fur_color: Colors.WHITE, eye_color: Colors.BLUE}, {fur_color: Colors.BROWN, eye_color: Colors.GREEN}]
describes a schema and thus `T` of {fur_color: Color, eye_color: Color}

Perhaps this gets easier if we introduce our `scope` object here, and deal with a string:value (dynamic) lookup?

This turns our StackFrameData approach into:

StackFrame {
    get_scope() -> Scope;
}

along with

Scope {
    get_type(key: &str) -> PolymorphicType // used for unsafe unboxing of value
    get_value(key: &str) -> PolymorphicValue
}

When working with a Scope inside a `repeat`, the user looks up values & types by (string) key.

Seems like a suitable solution.

*/

//Can we operate on a guarantee that for `n` elements in a repeat, the consumer (expression)
//will be invoked exactly `n` times?  If so, we could push a stackframe for each datum (in reverse)
//so that each invocation consumes a new stack frame, in order.  The tricky piece of this is
//a need to introduce stack frame `pop`s somewhere before the did_eval_properties_in_place lifecycle
//method, in a way that's unique to `repeat`.

//An alternative approach to this problem, which operates with the grain of "one stack frame
//per component instance," is to add an iterator to a new RepeatPropertiesContainer, which
//yields the next `RepeatProperties` on each invocation.  This may require simply modifying
//the inject_and_evaluate logic.  Perhaps we can introduce a `.next` method on Evaluator, with
//a default implementation that's a no-op, but which Repeat can override to step through
//an iterator.

// rtc.runtime.borrow_mut().push_stack_frame(
//     Rc::clone(&self.children),
//       Box::new(Scope {
//           properties: Rc::clone(&self.properties) as Rc<dyn Any>
//       })
// );


### untangling Definitions, Values, and Patches
2022/01/27

We need "patches" to support stacking of sparse
definitions, e.g.  {foo: "bar"} + {baz: "qux"} => {foo: "bar", baz: "qux"}

How do these patches come to bear between .pax and the runtime?
Where do Expressions (and the exptable) fit in?
`Patch`ing also requires Default fallbacks — where do _those_ slot in?

Perhaps Patches can be combined (a la overriding `+`), and can
be distilled into values.  Perhaps a component, e.g., `Root` supports
`apply_patch(&mut self, patch: RootPatch)`

Where does this logic live (which feature/lifecycle stage?)
Seems like `designtime` is the key.  Not needed for runtime
(action value setting can be a different concern with a lighter footprint)

Two flavors of instantiating Definitions:
- transpiling into RIL (hard-coded N/PIT; once & done) (where "N/PIT" means the "Node/Property Instance Tree")
- dynamic traversal into N/PIT for designtime
    - accept changes in definitions
        - special handling for Expressions/exptable
    - separate definitions from values (e.g. maybe `patch`es for each?)
      Note: e.g. `Root` vs. `RootPatch` already does this; Root is a "value" container (can rep. patch)
      Perhaps start with use-case:  we need to `Patch`-stack in order to assemble sparse property
      DEFINITIONS from pax, e.g.
    - Do we really need to worry about values at all?  Those are already handled well by the runtime.
      when a user changes

One wrinkle re: managing the patch-stacking logic in the designtime:
this would require dynamic evaluation in order to generate RIL.
Which shouldn't strictly be necessary.  RIL SHOULD be
generatable directly from a PaxManifest. (Is this true?
is this some sort of purity-for-purity's-sake situation?)

Perhaps it isn't so bad for the compiler to load the cartridge + designtime in order
to traverse the manifest =>

1. normalize PaxManifest (into a single traversable tree with inline property values as frames of the bare structs, ready for RIL
    1. This requires collapsing stacked values, probably in a way that's distinct from the way the designtime does it (designtime deals in stacks of patches, vs RIL transpiler dealing with a normalized tree)
    2. This also requires transpiling + "blank"ing in an Expression table
    3. This also requires knowing property schema in the Manifest!  Thus far this hasn't been a thing.
        1. Need a way to universally qualify types, a la module_path!() [this might be tricky!]
        2. Alternatively, could do another bin-conversion trick a la parser, and rely on macros to make sense of property types on-the-fly
            1. (Note: it will be important to know property schema eventually, not least to expose to design tool)

Conclusion: further dynamic evaluation is unideal; requires more compilation loops

### expressions
2022-01-28

transpile @ {x + 5} into |&properties| -> { &properties.x + 5 } along with glue/injection logic,
dirty-handling logic, and vtable-like logic for storing & retrieving function (references)

handle type resolution as much as necessary; let rustc complain about type mismatches/etc.

Expressions need to be dealt with in a few ways:
- parsed from 1. a template attribute, 2. a settings value (recursively within settings tree)
- looked up by ID, in RIL and in the DefinitionToInstance traverser
- hooked into with dirty-watching system, along with dependency DAG (choose which expressions to re-eval, when)
- future: hot-updated, via server message + recompiled binary/state-pass, when running the cartridge (compiler run mode, design tool)


- originally was thinking of a central vtable a la propertiescoproduct
    - this would make hot reloading easier (just replace the expressions sub-cartridge) — but it makes referencing difficult
        - maybe referencing isn't difficult with implicit return types!!
- am now thinking that each file generates its own expanded macros (via #[pax])
    - during compiler codegen phase, expressions are transpiled, surrounded by glue, and tagged/ID'd
    - for RIL, weave a code-genned pointer between a property instance and function (known from manifest)
        - e.g. `background_color: PropertyExpression(&expression_table.ae25534efc)`
    - for dynamic resolution, e.g. in designtime -
        - First of all, what does dynamic resolution mean?
            - It starts with a compiled cartridge + `designtime` feature, (already including RIL binary?) — which must already have an expression table compiled! (or capable of having it async loaded, FFI/etc.)
            - Then, a user changes the value of a property from a literal to an expression, or changes the definition of a current expression
            - Now, we must: 1. transpile the expression values to RIL,
- if it yet becomes the case that we need to deal with explicitly stated return types on the expression lambdas:
    - expose fully qualified types in `pax_exports`, then track fully qualified import paths (e.g. pax_std::exports::foo) in manifest
    - expose naked functions at the root of codegenned `expression_table`, like
```
pub fn x(input: isize) -> isize {
    input + 6
}
```
- (cont.)
    - where the return type of the codegenned function is fully qualified via the nested-mod re-exports trick
    - (and where primitive types are enumerated & special-cased)
    - This likely also requires registering `DeeperStruct`s, e.g. via `#[pax struct]`


### helpers, injectables
2022-01-28

e.g. Engine.frames_elapsed, or a userland helper function hue_shift()

API thought: can continue the `#[pax ...]` convention, decorating a function declaration like so:
```
#[pax helper]
pub fn hue_shift() {
    //gather an entropic hue value from the world
}
```


### on RIL generation, consumption
2022-01-31

How is the generated RIL consumed?
- Chassis reaches into cartridge and calls a method to get root/main component instance(s)
- Chassis then passes instance to the Engine to start rendering


Is this the right time to rethink instance management? Could fix the
mess of Rc<RefCell<>> going on in core.

Broadly, instances could be stored centrally in some Vec<dyn InstanceNode> (or hash)
This instance pool allows for safe passing of &mut throughout core

Finally, RIL can instantiate via this instance pool

...

Update, after spending a day on revamping instance management (progress on branch zack/valiant-rc-refactor-jan-2022), it's not currently tenable.
The lifetime tangle through core/runtime/stackframes/properties/RenderNode is beyond my current skill to fix.
Rc<RefCell<>> is not a bottleneck problem, aside from aesthetics and a minor runtime penalty (noting that
an equivalent lifetime-based solution would still effectively reinvent RefCell via the InstancePool)


SO: for RIL, proceed with on-the-fly Rc<RefCell<>> instantiations, just like PoC renderer code


### on circular dependencies, PropertiesCoproduct
2022-02-02

Despite moderately careful planning, we've ended up with a circular dependency between:
- userland cartridge
- core
- properties-coproduct
- userland cartridge (to wrap properties types)

Ideas for mitigation:
- codegen RootProperties _into_ pax-properties-coproduct instead of inline to the source file
    - Main drawback: this requires "ghosting" every type, annotating each subtype (or globally unique type name manifest)
    - Note also: the codegenned logic will depend on `runtime`, via `timeline` (at least) (`timeline` -> `Property` -> `RenderTreeContext` -> `Runtime` -> `Stack` -> `StackFrame` -> `PropertiesCoproduct`)
        - Could separate Timeline from Property, maybe — or revisit `compute_in_place` to see if something other than `RenderTreeContext` could be injected
- Slight variation: generate a separate cartridge/blank project `pax-cartridge`
- Split common dependencies from `core` & `cartridge` into `runtime` — it can be a leaf-node dependency of both, allowing common data structures/exchange
    - Some """driver""" logic may even need to jump over to cartridge, e.g. `Scope` and its dependants
- If all of `runtime.rs` logic is moved into PropertiesCoproduct — this might be fixed!
    - (plus Timeline, plus Property... plus RenderTreeContext?)
    - (plus ComponentInstance...)

As a broader strategy, could step back and look at the architecture of Engine,
more carefully drawing boundaries between Runtime, Property, Timeline, Core, and PropertiesCoproduct

### on properties
2022-02-06

In userland, e.g. an Action, properties:
- can read properties programmatically with .get()
- can set properties programmatically with .set()
    - not v0, but would be nice to have a path to someday setting values other than literal values, e.g.
      to create Expressions and Timelines at runtime
      In engine, properties:
- need to `compute_in_place` (whether literal, timeline, or expression)
- need to represent whether a property is literal, timeline, or expression
- need to `read` the current (cached) value
- need to instantiate with a starting value
- need to support runtime mutations via userland `.set()`, plus accurate up-to-the-frame value retrieval via `.get()`
- have dependencies on engine, via InjectionContext and `compute_in_place` parameters

Further: PropertiesCoproduct frames need to encapsulate Properties *instances* (the engine variant, if there are two variants)
which suggests a dependency on Engine wherever {}Properties are generated

Are these Properties data structures the same or different?  The rubber meets the road in RIL —
are the macro-generated RootProperties/RectangleProperties and PropertiesCoproduct entities the same?

Note it's easier to generate RectangleProperties alongside Rectangle in cartridge-userland, but
with an engine dependency they seem to need to exist in fully code-genned cartridge-runtime...

One possible tool to share the core Property definition is to split Property, PropertyLiteral,
PropertyExpression, and PropertyTimeline into pax_runtime::api (importable by both Engine & userland) —
then to write traits/impls that allow engine to `compute_in_place` and `read`

*^ proceeding with this strategy*

#### Re: Transform as a Property —
- Transform has a special method in the rendering lifecycle, `compute_matrix_in_place`.
- This is called in a subtly different context than `compute_in_place` for computableproperties — namely, it's called with the context of calculated hierarchical bounds (container size, etc.)
- Further, every RenderNode is expected to manage Transform, via get_transform
- Ergonomically, it would be ideal to treat any of the sub-properties of Transform as a plain Property,
- e.g. so that rotation.z can be an expression while scale[x,y] are literal values
- (Further, there seems to be no reason this can't continue recursively, with the `operations` API)

Question: given the above, should `transform` be expected as a member of `{}Properties`, or should we hoist it to be a top-level property of `{}Instance`?
- In the world where it's hoisted to be an `Instance` property:
    - We can still `compute_in_place` by special-casing `.transform` whenever we handle `compute_in_place` for properties —
    - that is, `.properties.compute_in_place()` and `.transform.compute_in_place()`.  To spell out further: `transform` is treated as a `ComputableProperty` in the engine
    - in every way except for being part of the PropertiesCoproduct.
- This also suggests an opaque-to-user special-handling of Transform during compilation.  Namely,
    - the user addresses Transform just as they would any Property, e.g. through Settings and through
    - runtime .get/.set APIs.  However — in RIL and engine (.transform.set), Transform is special cased


### Transform API
2022-02-07

What's a reasonable & ergonomic API for transforms, which:
- is terse & expressive in PAXEL
- is terse & expressive in the action API
- is thorough and enables specifying arbitrary transform order
-
Some ideas —

#### Array for operation sequence, enums for declaring operations
```
<Rectangle id="meow">

@settings {
    #meow {
        transforms: [Transform::Rotate(Angle::Degrees(27.8))]
    }
}
```
Pros: highly expressive
Cons: verbose (esp. enums)

#### More CSS-like?
```
<Rectangle id="meow">

@settings {
    #meow {
        transform: rotate(32) scale(1.2)
    }
}
```
pros: expressive & terse
cons: new DSL


#### Recursive?
```
<Rectangle id="meow">

@settings {
    #meow {
        transform: {
            operations: [
                Transform {
                    scale: [1.2, 2.2]
                },
                Transform {
                    translate: [400.0px, 300.0px]
                }
            ]
        }
    }
}
```
Pros: expressive and aligned with RIL
Cons: verbose, esp. nesting `operations` and reincantation of `Transform`


#### fusion of operation sequence + recursive?
either accept polymorphic Transform values (array or Transform) —
or surface monomorphic top-level properties (`transform-sequence : []Transform` and `transform: Transform`)
```
<Rectangle id="meow">

@settings {
    #meow {
        transform-sequence:[
            {
                scale: [1.2, 2.2]
            },
            {
                translate: [400.0px, 300.0px]
            }
        ]
    }
}
```



#### Require transform sequences to be handled with expressions?
con: runtime penalty (maybe?  maybe it's equivalent given expression caching!)


```
<Rectangle id="meow">

@settings {
    #meow {
        transform: @{
            Transform::scale(1.4, 2.2) * Transform::rotate(120deg) * Transform::translate(200px, 100px)
        }
    }
}
```

The above is quite nice.  The single-transform case is easily handled as a literal, as is the "manually expand matrix" case,
and the "combine transform" case is easily & elegantly handled with an expression.

Can also expose a matrix2d method on Transform for manual computation:
```
Transform::matrix2d(a,b,c,d,e,f)

representing:
| a c e |
| b d f |
| 0 0 1 |
```

Another note: Kurbo's `Affine` (used for pax's 2d backends) currently handles all of this with similar
ergonomics.  Would it make sense to (selectively) expose these APIs directly (e.g. impl'ing local
traits as necessary to inject behavior) — or should there be a stand-alone glue layer between
the user-facing Transform API and the rendering Transform API?
^ decision: yes expose new middle object

Finally:  it's not so crazy to introduce a special "transform list" syntax and
supporting it with the parser, e.g.:
`transform: scale(1.5, 1.5) * translate(100.0, 100.0)`
instead of wrapping in an expression.
But it's a tiny readability difference, ultimately `@{}` vs not.

Decision:
`Size` lives alongside `Properties` and `Children` as a top-level member of an `instance`.
Design GUI can special-case these built-ins
In the (unusual) case where a size is explicitly not desired (e.g. Group), then
it must be handled as a primitive (i.e. one that defines `fn get_size() { None }`)

one more decision:
Add `position` as a property? (essentially `.x` and `.y` — but consistent with ergonomics of giving `transform` and `size` their own buckets)
This would act as an affine translation, after anchor and align calculation but before remaining transform calculation
Currently it's not necessary because `translate` is effectively equivalent.

If `position` were added, given that it's purely ergonomic (approachability), consider
whether to add aliases like `.x`, `.y`,



### On `compute_in_place` for generated userland components
2022-02-08

Using `Stacker` as a reference, `compute_in_place` manually iterates over
properties and calls `compute_in_place`.

We don't want users to worry about this; we want to autogenerate the `compute_in_place` code for
properties.  Problematically, we can only call `compute_in_place` in the cartridge runtime context
(due to dependency on engine,) but we don't currently have metadata knowledge of properties
in that context.

One possibility:  expose an iterator that returns a sequence of Box<dyn Property> (`Property<WHAT>` though)...

Another possibility: separate the `rtc` across a trait boundary, allowing a similar maneuver
as `dyn ComputableProperty` in Engine

Note also: `Stacker` created its own RenderNode as its subtree root, with a single child `Component`
Should this be the general approach?  Is there a benefit to doing this?
(beyond the necessary ability to write `compute_in_place` logic for arbitrary properties,
though note that this could be generalized by exposing an iterator over

two options:
- expose RenderTreeContext via pax_runtime::api, untangle as needed, e.g. through traits or closure-passing
- codegen `compute_properties_fn` closures in RIL, cartridge-runtime; add properties intelligence to parser

For the former, conceptually it's a tough split.  the RenderTreeContext is squarely conceptually attached to the runtime.
The reason for the attachment is fundamentally to access StackFrame & Scope, which are used for runtime calculations (e.g. of Expressions)
Thus, and given that property schemas will need to be understood by the parser eventually:
*Decision: codegen `compute_properties_fn` closures in RIL, cartridge-runtime; add properties intelligence to parser*

We need a fn:
```
compute_properties_fn: |mut properties: PropertiesCoproduct, rtc: &RenderTreeContext|{
    if let PropertiesCoproduct::HelloWorld(mut properties_cast) = properties {
        //Note: this is code-genned based on parsed knowledge of the properties
        //      of `Root`
        properties_cast.deeper_struct.compute_in_place(rtc);
        properties_cast.current_rotation.compute_in_place(rtc);
        properties_cast.num_clicks.compute_in_place(rtc);
    }
},
```

This requires only knowing property names, not even types/import paths (extra easy)

Update: achieved apparently functional `compute_in_place`
Next steps: pencil in second rectangle
— then bite of expressions with manually written harness code, because there's a potential design dead end
here if we hit a wall with wiring up Expressions, Properties, Scopes, etc.

### on expressions
expressions will be transpiled to Rust — so some semantics will likely
carry over, e.g. symbol names, :: for namespace access or associated functions, etc.

dependency graph: via expressions, properties may depend on other properties
Expressions may also depend on globals/constants like @frames_elapsed
future/theoretical: expressions may depend on descendents' values, via selector (`@('#abc').num_clicks
Expressions may depend on other 'helper' expressions, or perhaps vanilla
functions that handle their own dirty-notifications
Expressions may have no dependencies, e.g. @{ 1+1 }

Numeric literals need special handling re: float & ints
Should cast implicitly between int/float when necessary
Perhaps study JS/AS as model

#### Symbol resolution
Symbols in scope may be:
1. properties on `self`
2. "helpers" (methods, or special macro-decorated expressions) on `self`
3. imported symbols, in scope in the context of `whatever.rs` — alternatively, maybe can capture in closure??
4. maybe "special" imported symbols, essentially a pre-imported expression `std`



Some example transpilations:

```
@{1 + 1}
1 + 1
```

```
@{
Color::rgba(
    Math::abs(
        Math::sin(num_clicks / 10.0)
    ),
    1.0,
    0.0,
    1.0
)}

Color::rgba(
    Math::abs(
        Math::sin(num_clicks / 10.0)
    ),
    1.0,
    0.0,
    1.0
)}

```

### Journey of an expression
2022-02-09

1. authored in .pax
2. parsed by parser; lives as string in Definition object, passed from parser binary to compiler
3. transpiled by compiler —> String (of Rust code, ready for cartridge runtime)
4. codegenned into cartridge runtime, as closure in ExpressionTable

In RIl (cartridge runtime),


### More dependency graph untangling
2022-02-10

*Property can't depend on Engine*, due to
the inverted dependency relationship between
cartridge and engine.  This is not news, but is worth pointing out
as the crux of this issue.

Previously we tried "sideloading" behavior
via a trait, which didn't work (is there yet a way
to make this work? one possibility is to declare
`Properties` objects (like `RectangleProperties`) in
a scope that has access to the necessary bits of `rtc`

Probably solid approach A:
- remove `compute_in_place`
- give every Property a UUID — register
  uuid -> &property (maybe ComputedProperty!) in a global hashmap
  instead of compute_in_place, look up
  a given property in each of
  `expression` global map and `timeline`
  global map.  if present, evaluate.
  What does evaluate mean here?  It means
  storing a cached computed value
- instead of `compute_in_place`...
  (this might run into the same problem with dep. graph, trait side-loading)


**Probably solid approach B:**

- keep `compute_in_place`, but pass it a `dyn`
  object, e.g. of a simple `Receiver` object (probably `impl Receiver for CarbonEngine`)

- pass the property's string ID to that receiver object
  when evaluating compute_in_place

- `Receiver` (probably Engine) pops from this stack
  (or removes from singular register) of string ID, uses that string ID to route `rtc`
  to the right table & Fn for evaluation (Expression, Timeline)



Re: storing the ExpressionTable — there's a wrinkle in that each return type for `||{}` makes for a unique
type signature.  Either we can give a PropertiesCoproduct treatment to return types — or MAYBE we can give a PropertiesCoproduct treatment to the `Fn`s themselves.

static HashMap<String, ExpressionLambdaCoproduct> {
"aef132": ExpressionLambdaCoproduct::
}

```



get_expression_evaluator_by_id(id: &str) {
    
}
```



### on API for align

when combining transformations, align should be thought of a bit differently vs other properties.
1. it's 'global' for the scope of a sequence of transformations.  In other words, there's at most one global value of alignment per RenderNode per frame.
2. it should be applied only once, at the end of a coalesced Transform sequence.
3. the latest align in a sequence (if more than one is specified) takes precedence, replacing previous settings


compute_transform_matrix can return two values: an Affine for Align, and an Affine for "everything else."
remove multiplication of align @ compute_transform_matrix
add multiplication of align at the caller of compute_transform_matrix()


### on nestable Properties, TypesCoproduct, Types in general
2022-02-12

1. TypesCoproduct vs. PropertiesProduct — is the distinction worthwhile?

generally speaking, PropertiesCoproduct is for larger aggregate types (the set of all properties for a given component)
while the TypesCoproduct, at advent, is intended for more atomic types, specifically anything
that can be returned by an Expression.  At least, there is an expected perf benefit
(at least memory footprint, possibly also wrapping/unwrapping overhead) of breaking out
a separate coproduct for "return types"

That said — do they converge, in theory, on the same idea?  There's no provision or need for
"all properties of a component" to be expressed as a single Expression, but certainly each
individual property can be.

Further, for compound property types like Stroke, there's a need to
express the "aggregate type" as an individual expressible Property in addition (as a mutaully exclusive option)
to addressing its subtypes as individual Properties.  In practice that probably looks like
```
PropertyLiteral > 
    PropertyLiteral
    PropertyExpression
```
for
```
stroke: {
    color: Color::rgba(255, 255, 255, 255);
    width: @{ $frames_elapsed * 0.001}
}
```

So in short, the PropertiesCoproduct and the TypesCoproduct are categorically the same thing,
and _could_ be shimmed into the same object if necessary.  That said, there's a likely
performance benefit to keeping them separate (allows TypesCoproduct operations to be
smaller in memory footprint + bandwidth, possibly also in computational wrapping/upwrapping, weight
in CPU cache)

As an important use-case, consider the Path (i.e. arbitrary bezier/segment/gap sequence)
— it can be represented either as: a series of nodes and segment-type specifiers, or a series of "drawing commands" (a la SVG)
In design-tool land, it would be nice to be able to "direct select" a path point and express any of its properties individually — additionally,
it would be nice to support "shape tweening" between two arbitrary `Path` values

```
<Path id="my-path" />
@settings {
    @my-path {
        points: [
            {x: ,y: ,handles: [{x:y:},{x:y:}]},
            {x: ,y: ,handles: [{x:y:},{x:y:}]},
            {x: ,y: ,handles: [{x:y:},{x:y:}]},
            {x: ,y: ,handles: [{x:y:},{x:y:}]},
        ]
    }
}
```

This verbose (if as minimal as possible) description of points feels ergonomically similar to Timeline.  Consider this similarity when
locking in API design...


### on Actions and event handling
2022-02-12

need to nail down:
- syntax in pax
- userland API, ease of getting/setting values (rename Property::cache_value to `set`?)

built-in lifecycle events: onclick, ontap, ontick

are there user-extensible lifecycle events? perhaps a component can trigger events,
allowing binding of events (classic BEST-style) -- e.g.:

```
<MyElement onclick=@handle_click />
```


Types: function will be expected to accept the correct types, e.g. ClickArgs for onclick
This isn't an Expression and won't be evaluated through the lense of PAXEL
The @ syntax is both conceptually correctly _different_ than `@{}` — and is a nod to the "magic splice"
nature of the decorated symbol (i.e. that @handle_click will magically (via codegen) be spliced with its
args into a complete statement in RIL)

What about binding to methods on descendents?  Perhaps `@('#some-id').some_fn` (with a hat-tip to jQuery) allows
for nested access.  Note that this extends to referring to desc. properties as well!


#### Journey of an Action

1. defined as a `pub fn` in code-behind
2. bound to an element a la a property, e.g.: `<Group onclick=@handle_click>` or `@settings{#some-group : { onclick=@handle_click }}`
3. parsed, pulled into manifest: templatenodedefinition `x` binds event `y` to function (handler) `z`
4. generated into RIL to handle this runtime story:
    1. user clicks on screen
    2. hits either:
        1. native element (e.g. in Slots or mixed mode content)
        2. or virtual element (canvas)
    3. for virtual elements, ray-cast location of click, find top element
    4. dispatch click event (add to message queue for relevant element(s))
        1. DECIDE: capture/bubble or something better? Might be able to avoid needing a `capture` phase if parents can add (essentially) `pointer-events: none` to children, deferring within-hitbox events to themselves (parents)
        2. What other events may be dispatched?  Tick, tap, mouseover, etc. — but what about custom events?
        3. This probably is a responsibility of RenderNode, and might offer a robust default impl. for common events.
    5. each tick, process message queue for each element.  Take a look at the lifecycle/event order here, e.g. which of `tick` vs. `click` etc. happen first (intuitively, a click's handler should fire BEFORE the next click)


Generated RIL needs to accept a polymorphic arg, e.g. EventClick or EventTick (args coproduct?), unwrap it,
and bind its values to a method call on the instance itself (`pub fn handle_click(&mut self, args: ArgsClick)`)

Could keep `n` different queues per node, for `n` different types of args
(requires writing queue management logic when adding new event types)

Or could keep one queue with genned glue logic for unwrapping coproduct
into args

(cont. 2022-02-14)

Probably `impl Handler<EventClick> for RenderNode`, as well as
all other built-in event handlers.  `EventCustom` might be centrally
implementable in the same way, allowing userland to deal with named
`EventCustom`s


Engine has coordinates & a RenderNode — must fire userland declared
method with correctly injected args

The method itself exists on the instance (`Rc<RefCell<dyn InstanceNode>>`)

The execution of the method can be done with a closure (which can be
code-genned, and which can also be attached at runtime!)




Pass `fn` pointer — note that even for methods, the `fn` is global.
Must be resolved by global import, and probably (almost certainly)
must be passed through parser+codegen.

This fn pointer, then, can be evaluated by calling:

```

fn dispatch_event_click(args) {
    //args are passed by engine
    //probably unwrap args coproduct here
    let some_fn_pointer: fn(&mut self: RootProperties, evt: EventClick) = pax_example::handlers::some_onclick
    let mut instance = (*some_rc).borrow_mut();
    some_fn_pointer(&mut instance, args);
}
```



Each tick, the message queue will be exhausted, even if there are no
handlers bound to relevant events (i.e. `EventClick`s will propagate
in message queue but will be unhandled.)

Should handlers support being attached at runtime?  probably not, at least
while Rust is only supported language. (how to add without recompilation?)

**Click -> Chassis -> Engine queue (with known element id)
Every tick, process queue — if the ASSOCIATED ELEMENT (via id from engine queue)
has a REGISTERED HANDLER (via .pax, or in the future perhaps added at runtime)
then TRIGGER the registered handler with chassis-generated args**

Chassis: set up native listener, instantiate relevant engine struct with data, enqueue in engine
Engine: each tick, before rendering, process event queue; dispatch events on RenderNodes
RenderNode: default impl dispatch: unwrap (match) arg/event type,
check for any local registered handlers (&fn), fire if present (in order registered)

What about tick handlers?  tick is a little "special" because of how voluminous the events are
(60-120/s * n elements) — there's likely a bandwidth/computation overhead to processing so many
events spuriously.

Perhaps `tick` can be special-handled, checking for handlers on each element during rendering (or properties comp.) recursion
and then dispatching

Handlers: attach to instances or to definitions?
To instances.  Ex. if there are two Rectangles, each should have a separate handler for Click
Thus it follows that we want to associate handlers with INSTANCE IDs rather than component/def. IDs



#### design and code together; ship UIs to every device.



### on instance IDs, handlers, and control flow

1. inline, compiler-generated literal ids will add to cartridge footprint
2. handlers need to be able to look up element by ID (node_registry)
3. either: a.) IDs are inlined during compilation (e.g. by the mechanism used to join expressions + properties), or b.) generated at runtime
    1. Expression IDs have to be managed at compile-time, to solve vtable functionality within Rust constraints
    2. node_registry (instance) IDs should probably be managed at runtime, because:
        1. literal inlining takes toll on footprint
        2. dynamic primitives like if/repeat, which may dynamically instantiate children with methods/handlers, _must_ do this at runtime

HandlerRegistry<T> must be associated with e.g. RectangleProperties (<T>),
because that type T will need to be injected as `&mut self` when calling
a method/handler.

Either: bundle all HandlerRegistry<T> Ts into PropertiesCoproduct,
or store a distinct HandlerRegistry<RectangleProperties> (e.g.) per dyn InstanceNode

Engine has an intent to dispatch an event,
an element ID,
and event-specific args (ArgsTick, for example.)

Look up element by id, get `dyn InstanceNode`

could expose `dispatch_event()` on `RenderNode` —
challenge is passing the right `&mut self` into the registered method call.

Maybe we don't want to resolve RenderNodes with the node_registry at all?
Can we resolve to the instance of `RootProperties`?
The answer is probably yes, because properties are stored in an Rc<RefCell<>>, which we can clone into the node_registry

So: Engine has an id and an event, looks up id in node_registry,
gets an Rc<RefCell<dyn Any>>

That PropertiesCoproduct needs to be unwrapped so the right
type of &self can be passed to our stored fn.




*Important distinction:* event handlers (methods) do NOT
accept a `&self` for the element attached to the method —they accept a `&self` for
the component (or `repeat`/etc.) owning the relevant stack frame.

One possibility: generate a `HandlersCoproduct`, akin to `PropertiesCoproduct`, that generates
all of the necessary method/handler signatures (`fn(&mut RootProperties, ArgsClick)`, etc.)

Ideally, though, these pointers don't need to be stored centrally.

During code-gen, we know the context (component) to which any instance
belongs.  So it's straight-forward enough to inline a `fn` declaration, type, or container
with the right type during that inlining process.

We only care about handlers in the context of Components!



### plan for event dispatch
2022-02-18

From RIL, we can:
wrap each dispatch definition (the raw userland method call) with a closure
that takes generic (coproduct) parameters and uses
codegen to map/unwrap those parameters into the invocation.

codegen a reference to the StackFrame owner's Properties
(namely Component/RepeatItem properties)

Then:
- de-genericize HandlerRegistry; populate with the wrapped/codegenned closures described above
- add `get_handler_registry() -> &HandlerRegistry` to `trait RenderNode`
- now, engine can ask a RenderNode for its HandlerRegistry, check if a given event type is registered, and dispatch the handlers if so

Re: tick, the same check can be made during properties comp (pre-`compute_in_place`, perhaps, so that any side-effects will be accounted for in the immediate next render)
in other words, `tick` is really `pre`-`tick`, esp. notable if `pre`/`post` tick handlers are later introduced.




### on scoping, @foreach
2022-02-22
consider the template fragment:

```
<Group>
    <Rectangle ontick=@handle_tick id="rect-a" />
    @for rect in rects {
        <Rectangle fill=@rect.fill />
    }
</Group>
```



when creating new scopes for @for, we have two major options:
- Angular-style: replace scope.  would require kludgy re-passing of data into repeatable structs, far from ideal
- more robust: additive (shadowed) scopes (req. dupe management and dynamic resolution up stack frame)
specifically, `for` adds to scope, vs. component definitions resetting scope (cannot access ancestors' scopes, certainly not implicitly)
duplicate symbol management: check at compile-time whether symbol names are available in shadowing scopes


logic to resolve symbol (e.g. `rect.fill`) to backing value:
- determine whether `rect` is in current scope:
- resolve value via `rtc` with statically baked lookup logic (code-genned)
-


For enumeration:
```
@for (rect, i) in rects {
    <Rectangle fill=@rect.fill />
}
```

Note that this still requires the creation of `PropertiesCoproduct` entries for shadowing scopes,
which contain just 1 or two members: `rect` and `i` in the case above. (corresponding to RepeatItem's `datum` and `i`)
-- in fact, maybe not!  Perhaps just RepeatItem needs to be added, which is recursive and handles everything we need



#### What about dynamic IDs?
```
@foreach (rect, i) in rects {
    <Rectangle id=@{"rect-" + i} />
}
```
This would require dynamic application of SettingsLiteralBlock, which is currently only done at compile-time
Can this use-case be handled otherwise? Introduction of `class` would run into the same problem

Could destructure and pass a reference to a `RectangleProperties`. (`<Rectangle ...rect>` where `rect : RectangleProperties`)  Gets a bit unergonomic around optional/null

Winner:
*Can natively handle indexing at the `@settings` level, e.g. with pseudo-class like syntax:*

```
#rect:nth(0) {

}
```

This comes with a decision: *all declared IDs must be known at compile-time.*

Quick sidebar: this brings us to commit to `id` as non-unique (or to supporting `class`)

Is runtime application of SettingsLiteralBlock going to be important regardless? It's solved for the `Repeat` case.  Will we someday want
to support imperative node addition, with the expectation that new nodes matching existing selectors will be applied the
relevant settings block?  This requires adding selector-matching logic in the runtime, as well as storing SettingsLiteralBlocks
separately in runtime (as opposed to current flattening/inlining)

Since all known use-cases are currently handled by pre-authoring (declarative) tree permutations, this question and
refactor can be revisited at a future time where we contemplate an imperative userland scene graph mutation API


One other sidebar — since IDs are guaranteed to be static, they may as well be plain ol' `snake_identifiers` rather than `"string values"`, right?



`@foreach i in (0..10) `


Support literal ranges, both exclusive and inclusive `(0..=10)`

Could technically enumerate also, `@foreach (val, i) in (0..10)` but only useful (maybe useful?) in cases where the range is not (0..n)




### On instantiation in RIL

1. currently using `::instantiate` associated functions, as a way to inject side effects (registering instance in instance map.)

An alternative, robust solution is to instantiate more 'imperatively' -- instead of in one big recursive statement (RIL tree),
instantiation can occur using vanilla object constructors and manual invocation of side effects (e.g. registration in node_registry)

Roughly, this requires starting from the bottom of the render tree and moving upwards.  For a given leaf node, instantiate its bottom-most sibling, then each successive sibling until all children of the shared parent are instantiated.  
Recurse upwards until root is instantiated.

Disadvantage:  RIL becomes more cumbersome to read, write.  Advantage: cleaner instantiation logic.

Another option: add `instantiate` to RenderNode, thereby firming the contract of what needs to go into instantiation
(e.g. the node_registry, the handler_registry, and properties)

*Decision: add `instantiate` to RenderNode*



### children vs template vs slot_children
2022-02-28

Refer to the following comment copied from the definition of `RenderNode`:

```
/// Return the list of nodes that are children of this node at render-time.
/// Note that "children" is somewhat overloaded, hence "rendering_children" here.
/// "Children" may indicate a.) a template root, b.) slot_children, c.) primitive children
/// Each RenderNode is responsible for determining at render-time which of these concepts
/// to pass to the engine for rendering, and that distinction occurs inside `get_instance_children`
```

Perhaps it is worth revisiting these concepts in order to compact
the conceptual surface area.

*Template:* conceptually sound with
- Repeat (repeating a template, a la a stamp)
- Component (instantiates an instance based on a template, a la a stamp)

*slot_children:* conceptually sound with Stacker, via slots
- fits in the same struct `InstanceNodePtrList`
- instead of an "average case tree," slot_children are an "expected case list"
- Sequence of siblings is relevant (beyond z-indexing); used to pluck slot_children away into whatever context

*Children:* a.k.a. "primitive children," the intuitive
"XML children" of certain elements that deal with children,
such as Group, Frame, and more.
Note that slot_children are a special form of children — Stacker's
`children` (per the `Component` `template` definition that declares that Stacker)
are dealt with by Stacker as slot_children.

Tangential observation: the `Component` has no way of knowing whether
the children it's passing will be dealt with as `slot_children` or `primitive children`.

**So: can these concepts be compacted?**

Let's imagine establishing a rule: that a `RenderNode` may deal with its
`children` however it sees fit.  For example: it may or may not deal with
`children` as order-sensitive `slot_children`.  The management of `children` is thus
_encapsulated._

A key distinction between `template` and `children` is the authoring.  `children`
are passed as "intuitive XML children", e.g. to Repeat & friends.

`template` is a definition, currently used only by Components.  A Component may
have both a `template` (its definition) and `children` (slot_children) [but is there
ever a case where a Component instance has non-slot_child children? perhaps not!]

Given the duality above, perhaps it's worth making `slot_children` explicit?
It reinforces the notion that `children` are injected from the outside,
rather than "birthed" internally (a la a template)

Is there a case where `slot_children` doesn't make sense to describe children?
For `Group`, for example, it's awkward.  Group also doesn't use slots.
So:
- `children` are for primitives, e.g. for a `Group` and its contents
- `slot_children` are specific to `Component` instances (because `StackFrame` is req'd.) that use `Slot`s
- `template` is specific to `Component` instances




### on properties for core primitives

e.g. `source_expression` for `Repeat` and `index` (and maybe someday: `selector`) for `Slot`

Currently these are being shimmed into the top-level `InstantiationArgs`, e.g. `source_expression`.
This works but is a bit inelegant.  Options:

- introduce a sub-struct, essentially just a cleanliness 'module' for InstantiationArgs.
  add the relevant built-in-specific data there, like `slot_index` and `repeat_source_expression`.
- generate the built-in `{}Properties` structs inside pax-cartridge,
  where they can be imported to PropertiesCoproduct (see RectangleProperties)



### on stacker, "primitive components"

after tangling with porting Stacker as a primitive, decided
to stop for now:

It's not a great use-case to encourage (primitive + component, lots of ugliness incl. manual expression unrolling)

Instead, this is a great use-case for bundling stacker.pax and stacker.rs into an importable package, via pax-std

Can "manually unroll" the code for importing in pax-std in order to "derive the codegen proof"


### on demo story

Performative coding or compelling example?
Former requires automated compiler
Latter requires "PoodleSurf" effort, plus support for images, text styling, full layouts, animations, interactions, more.

Both would be ideal for a broad audience!

What about for a more understanding niche audience?

What about showcasing designability?

### on timeline API

2022-03-03


Here's the current def. of TimelineSegment

```
pub struct TimelineSegment {
    pub curve_in: Box<dyn Easible>,
    pub ending_value: Box<dyn Property<f64>>,
    pub ending_frame_inclusive: usize,
}
```

1. needs default value (e.g. for keyframe-0 if no keyframe value is defined)
2. needs sequence of TimelineSegment
3. can be element-inline or in settings block

```
# some-rect {
    transform: @timeline {
        starting_value: Transform::default(),
        segments: [
            (15, Transform::rotate(12.0), InOutBack)
        ] 
    }
}


//more spatial, ASCII-art-like.  Mildly awkward to type/maintain, but not so bad; big ergonomic benefits, and
//UX can be improved through tooling, e.g. like the markdown table plugin in jetbrains IDEs
# some-rect {
    fill: @timeline {
        (0           ,             10,           100)
        (Palette::Red, Palette::Green, Palette::Blue)
    }
}
//how about a vertical version?
# some-rect {
    fill: @timeline {
        0: Palette::Red,
        10: Palette::Green,
        100: Color::rgba(
            100%, 100%, 0%, 100%
        ),
    }
}
//not only does the above feel more familiar a la JSON, it elegantly handles multi-line constructions like the `Color::rgba...` above.


//or horizontal with paging?


//Maybe there's ALSO a use for the horizontal syntax, for simple cases and basic constants.  It just feels good when well aligned -- more like looking at a visual timeline

# some-rect {
    fill: @timeline {
        |0            |10             |100           |
        |None         |Linear         |OutBack       |
        |Palette::Red |Palette::Green |Palette::Blue |
        
        |120          |180            |220           |
        |Elastic      |Out            |In            |
        |Palette::Red |Palette::Green |Palette::Blue |
        
        |300          |500            |800           |
        |Bounce       |
        |Palette::Red |Palette::Green |Palette::Blue |
    }
}

//That said, the apparent inability for the horizontal syntax to elegantly handle multi-line statements makes it a less
//practical choice than the vertical JSON-like syntax 



```

Note that a segment's value can be a literal (as described here with `Transform::rotate(12.0)`) or it can be an expression (e.g. `@{Transform::rotate(12.0) * Transform::translate(100.0, 200.0)}`)


### on align
2022-03-03

Currently align is a float.
Ideally it should be a percent.
Decision: port to Size, panic if px value is passed



### on dirty-checking, userland API
2022-03-04

Stacker needs to update its cached computed layout as a function of its ~six properties:

pub computed_layout_spec: Vec<Rc<StackerCell>>,
pub direction:  Box<dyn pax_engine::api::Property<StackerDirection>>,
pub cells: Box<dyn pax_engine::api::Property<usize>>,
pub gutter: Box<dyn pax_engine::api::Property<pax_engine::api::Size>>,
pub overrides_cell_size: Option(Vec<(usize, pax_engine::api::Size)>),
pub overrides_gutter_size: Option(Vec<(usize, pax_engine::api::Size)>),


As a single expression? (probably requires functional list operators in PAXEL, like `map`, as well
as lambdas)
```
(0..cells).map(|i|{
    
})
```

By surfacing an imperative API for dirty-checking?

Add to `dyn Property`
`is_fresh() -> bool`

and `_mark_not_fresh()` — but when is this called?

As a consumer, Stacker wants to know whether a value is "fresh" in order
to determine whether to proceed with a calculation.

Specifically, `fresh` here means "does it have a new value since last tick," presumably
within the scope of an every-tick handler like "prerender"

Note that the Expression dirty-handling system should be able to bolt onto the same
API.  Perhaps this should be deferred to the point that system is built,
so that the major use-case (Expression dirty-checking) is handled best


Any time a value is set (and implicitly marked fresh), we need to ensure
that freshness is maintained until the next complete batch of expression calculations
is complete.  That means *at the end of the following frame* (not the current frame, which is partially evaluated)
This also means that a `postrender` handler should see `fresh` twice for a given
`set`, but `prerender` should see it only once.  Thus, by this approach,
there should be room for userland patching into `pre-render` to do manual
caching based on dirty-checking.



### On untangling Render Instances, API instances, Definition objects, Properties objects
2022-03-04

*Render nodes == instances*
Any instance will impl `dyn InstanceNode`


*API instances* are `Property`-free structs used e.g. for imperative instantiation
without needing to wrap with `Box` and `PropertyLiteral`.  `Stroke` vs `StrokeProperties` is a great example.
Note that `Stroke` is not a RenderNode, and likely never would be (a RenderNode is unlikely to be managed imperatively in userlkand

*Definition objects* are used by the parser, stored in manifests

*Properties objects* are containers for node-specific, arbitrary values, conventionally
wrapped in `Box<dyn Property>` (though technically not necessarily so, e.g. perhaps for a cached or private value)

---

*A use-case,* perhaps not perfectly served by the existing ontology, is an
object that can be:
1. easily declared with low conceptual overhead
2. imported and used in templates in other files.  
E.g. just `Rectangle`, not `RectangleProperties` or `RectangleInstance`.

This feels adjacent to `Stroke` vs. `StrokeProperties`, but it's re: a RenderNode
rather than a PropertiesObject.  Should there thus be an "API Instance", but
for RenderNodes?

Thus, to define a custom RenderNode, one must provide:

1. the API object, or at least symbol name (Rectangle)
2. the Properties object
3. `Instance`: if primitive, the `impl RenderNode for ...`, else `ComponentInstance`

---

*Another use-case:*

a userland component def (e.g. Stacker) may want to access not only the current node's
`properties`, but also it's built-ins like `size` or maybe even `transform`.

What does that API look like?

Consider the following excerpt:
```
pub fn handle_prerender(&mut self, args: ArgsRender) {
        
        let bounds = args.bounds;

        let active_bound = match *self.direction.get() {
            StackerDirection::Horizontal => bounds.0,
            StackerDirection::Vertical => bounds.1
        };

        let gutter_calc = match *self.gutter.get() {
            Size::Pixel(px) => px,
            Size::Percent(pct) => active_bound * (pct / 100.0),
        };

        // snip
        
    }
```

The key question: What is `self` here?
Is it the API object?  It would be the easiest to author (`impl Stacker { ... }`)
Let's say it's the API object.  Can we also have the Properties available on that API object?
(This would suggest that the PropertiesObject and the API object are the same thing.
This would further suggest that RectangleProperties -> Rectangle, and that the user is responsible
for wrapping properties in Box<pax_engine::api::Property<>> — or that the `pax` macro help in this regard
(perhaps suppressible with an arg to the macro))

So, we can reduce our surface area to:
1. Instance object (RenderNode), e.g. `RectangleInstance`
2. Properties object + API object, e.g. `Rectangle`.  Properties must be wrapped in Box<dyn Property>, even if by macro.
3. `DeeperStruct`-style nested literal objects.  These must be wrapped in `Box<dyn Property>`.



(cont. 2022-03-07)

SO: when declaring a component instance, say `Root` or `Stacker` —
1. we're declaring the `Properties + API` object (`Root` and `Stacker`, with a series of `Box<dyn pax_engine::api::Property<some_type>>` properties
2. there will be auto-generated Instance impl (or Factory, or boilerplate instantiation code)
    1. On this point — which is best?  Probably generation of Factory/Instance, both for consistency (easing codegen reqs) and for footprint (presumably lower footprint)_
3.







### On return type for `@for (elem i) in self.computed_layout_spec`

The expression generated for this @for statement -- i.e. the expression bound to the `source_expression`
property of the `Repeat` instance -- needs to return a `Vec` of _something_.

The `source_expression` property for Repeat is of type `pub source_expression: Box<dyn Property<Vec<Rc<PropertiesCoproduct>>>>`

So that _something_ should probably be a `Vec<Rc<PropertiesCoproduct>>`.  This means the expression needs to
remap from `computed_layout_spec` — i.e. whatever is the type of each element of that array — into a PropertiesCoproduct.

More specifically, the compiler needs to know what variant of the PropertiesCoproduct enum to wrap each element
into.  In the Stacker case, it's `PropertiesCoproduct::StackerCellProperty(elem)`

How can we know what type `StackerCellProperty` should be?  One option is to make it static, i.e. for
`B` in `@for A in B` to be a statically knowable symbol, present in the current scope.

One version of this constraint would be to ensure the symbol is a `Property`.  Another could
include its ability to be a method call (we can know the return types of methods from the parser via macros)

Is it the case we will only want to support `static property` symbols and `static method calls`?
(more dynamic collections could be computed in event handlers...)  so, yes!

OK — so the parser is responsible for gathering the type attached to a symbol, incl. via shadowing,
whether the symbol is a `property` or a `method`.  We can expect that the type is wrapped in `Vec<>`
(or, perhaps, is otherwise `enumeratable`).  We should also be able to support `literal ranges` like `(0..10)` or even `(0..num_clicks)`

With this type in hand, the compiler can generate the correct Coproduct wrapper type,
e.g. `PropertiesCoproduct::SomeDiscoveredType(datum)`

Let's walk through how that gets unwrapped, as a sanity check.

Repeat will create a `Component` instance mounted with `PropertiesCoproduct::SomeDiscoveredType(datum)` as its `properties`.
Inside any expression with access to this scope, a member may be invoked, any symbol...

*The compiler is responsible once again for resolving that symbol (or throwing an error if it
cannot be resolved) and then generating the correct "invocation" code to bind that symbol*



### On ergonomics, working with sequential data and interactions

Use-case:  from three elements repeated on a screen, when any one is clicked,
increment _that_ element's rotation, independently of the other two.

```
@template {
    @for i in (0..3) {
        <Rectangle on_click=@handle_click />
    }
}
```


Possibilities:

#### embed scope in ArgsClick

Requires unwrapping scope type in method body, no bueno

but... really... we want the fully qualified, shadow-compatible scope

if we have `@for i in (0..3)`, it would be simplest and cleanest to have access to `i`,
as well as `j` in a nested `@for j in (0..5) {`.

This is theoretically doable, with code-genned "shadowed frames" for each context
where scope may be embedded in a click arg.  In the simplest case, scope
just returns the Properties object.  If there's a `@for i in (0..3)`, any event
handler bound instead of that loop should have access to an {`i`} union {`...current_properties`}


#### embed metadata

probably by a special syntax around handler binding
(maybe just embed index? but what about nested `for`s?)

```
@template {
    @for i in (0..3) {
        <Rectangle on_click=@handle_click with i/>
    }
}
```

might be able to embed "metadata" in the ID and parse it later, but that's kludgy

Could also add a Vec<usize> to ArgsClick, which can keep indices (only; not data, which would really be nice to have)


#### embed args, codegen with compiler

something like:
```
<Rectangle id=r >
@for i in (0..10) {
   <Triangle on_click=@handler with (i, r)>
}
...
```
`on_click=@{self.call_method#(i)}`
`on_click=@{self.some_method | i }`
`on_click=@{def self.some_method(.., i) }`
`on_click=@some_method<i, s>`

`on_click=@some_method @(i, j, string)`
`on_click=@some_method of ..., i, j`
`on_click=@some_method with i`
^ this one is pretty nice, or:
`on_click=@{self.some_method < i,j,k }`
(pipe feels a bit more conventional...
this is adjacent to currying, but

this would codegen a closure that "summons" `i` via the
same codegen logic used elsewhere in compiler, then
passes not only the unwrapped ArgsClick (etc.) but also
`i` (and optionally `j` or `datum` or whatever.)

In other words:
`on_click=@{self.some_method | i }`
would call at runtime
`some::namespaced::method(&mut self_instance, args_click, resolved_i)`
and
`on_click=@{self.some_method | i, j }`
would call at runtime
`some::namespaced::method(&mut self_instance, args_click, resolved_i, resolved_j)`


A major advantage of this approach is that it allows rustc
to deal with enforcing types (if `i` or `datum` isn't the right type for
the method signature of `call_method`, `rustc` will complain)

`(fn a b) ** c -> (fn a b c)`


a distinction -- currying is unary — from a given (likely nested) function,

#### on `ref` use-cases

The `with` functionality may also support
React-style `ref` functionality, if we choose
to enable it.

For example:
`on_click=@some_handler with some_rect_id` where `some_rect_id` is `<Rectangle id=some_rect_id>`

This would (probably) resolve to the Instance+Properties object (the PropertiesCoproduct entry) for that element.

The major family of use-cases this would open is imperative manipulations of the render tree (if we want to support this?)
and otherwise imperative manipulation/access of properties


#### on returning chunks of template from methods
"stencil"

This is a common pattern in React, for example. It allows for cleanly breaking
complex templates into smaller pieces, without needing whole new Components

```
#[pax(
    <Rectangle>
    get_dynamic_thing()
)]
pub struct Hello {}

impl Hello {
    pub fn get_dynamic_thing() {
        pax_stencil!(
            <Group>
            
            </Group>
        )
    }   
}
```

Is this a special case of grouping?  
It's roughly like minting a component, without a stack frame
Could easily be managed in a design tool as such, while maintaining
compatibility with hand-coding

~complet? comp? precomp?~
Let's go with *stencil* for now. [edit: precomp might be better, c.f. After Effects]

What about when a stencil need to take parameters?  No problem —

```
#[pax(
    <Rectangle>
    @for i in (0..10) {
        get_dynamic_thing(i)
    }
)]
pub struct Hello {}

impl Hello {
    pub fn get_dynamic_thing(i: usize) {
        pax_stencil!(
            <Group>
                
            </Group>
        )
    }   
}
```

Note that `i` is bound from the template into the method call `get_dynamic_thing`

The compiler can weave this together in the same fashion that it handles `with`



### on slot_children
2022-03-17

Certain `should_flatten` elements, namely `if` (`Conditional`) and `for` (`Repeat`),
need to hoist their children as a sequence of slot_children, in lieu of themselves as singular nodes, e.g.

```
<Stacker>
    @for i in (0..10) {
        //shadowed scope (component) adds `i` here
        <Rectangle>
    }
    <Ellipse>
</Stacker>
```

Stacker should have 11 slot_children in this case

Possibly a wrinkle: the computation of `Repeat`'s children
(via `source_expression`) might come later in the lifecycle than assignment
of slot_children. (Update: this was indeed a wrinkle, fixed by adding a manual computation
of slot_children' properties when they are `should_flatten` (namely for `if`, `for`))

Also take note that `Repeat` wraps each element in its own
`Component`, which will take a stack frame and which currently

Stack frames are pushed/popped on each tick

Expose `pop_slot_child() -> Option<InstanceNodePtr>` on `StackFrame` (and maybe `nth_slot_child()`)
StackFrame greedily traverses upward seeking the next `slot_child` to pop.
`slot_children` become strictly an implementation detail, meaning the field can be eliminated
and `Component` can pass its `children` if specified to the StackFrame that it creates.  
Unpacking `should_flatten` nodes can happen at this stage, and this probably requires a linear traversal of top-level child nodes.


Stacker's template in the example above might be something like:

```

@for i in self.cells {
    <Frame transform=@{self.get_transform(i)}>
        <Slot index=@i />
    </Frame>
}

```



Stepping back briefly...

Conceptually, when we expose slots, we're opening a "slot".  We're allowing two nodes
to be connected in our graph, a `child` (to become `slot_child`) passed to a Component, and to a contained `Slot`, which mounts that `slot_child` as its own `child`.

Stacker introduces an additional `Component` into the mix, underneath `Repeat`.
There seem to be some cases where we want to traverse parent nodes
for slot_children, and other cases where we don't (e.g. a Stacker
with insufficient slot_children should render empty cells, not the surplus slot_children that were
passed somewhere higher in the render tree.)

We could pipe slot_children explicitly, e.g. Repeat hand-picks an
slot_child for each ComponentInstance, attaches it to that stack frame,
and we go on our merry way.

There's also still the problem of flattening.
Repeat and Conditional could be trained to push their children directly to
a mutable StackFrame's `slot_children` list... still there's a matter of
lifecycle management, though. (will those slot_children be pushed at the
right time?)

```
<Stacker>
    @for i in (0..10) {
        <Rectangle/>
    }
</Stacker>
```

Could the `pre_render` hook be useful here?  Properties have already been computed:
for **this node, but not for its children** (e.g. Repeat)
So no, `pre_render` probably won't be helpful as it sits.


Probably our best bet is for the lookup to be dynamic on StackFrame itself.

1. register children to StackFrame `slot_children` naively, no unpacking
2. expose `nth_slot_child` on StackFrame, which
    1. somehow knows when to stop traversing stack upwards to seek slot_child list (special flag on componentinstance => stackframe ? hard-coded in Repeat where it instantiates Component, for example), and
    2. expands `should_flatten` nodes to compute its index lookup. naively, this can start O(n)
3.
Where `nth_slot_child` checks all nodes for `should_flatten`, grabbing
a

Maybe there should be an explicit "slot_child delegation" operation? where a component
may delegate the responsibility of certain slot_children to a member of its template


Twofold problem:
1. slot_children need to have their properties computed, at least for top-level should_flatten

### on TypeScript support, syntax
2022-03-22

1. comment (@pax `<Rectangle ...>`)
2. JSX-style approach, extend JS/TS as a language.  Note: with a subtly different syntax, either:
    1. adjust Pax's template syntax to conform with JSX, or:
    2. fork JSX extensions (for build tooling, code editors) to support Pax

It's probably worth _embracing_ the distinction that Pax is a separate language (without closure access or side effects in `render` fn)

It's also probably worth embracing the advantage of strong typing (i.e. not worry about vanilla JS support; instead focus on TS support,) even if it diminishes the shorter-term reach of Pax.

There is almost certainly room in the world for more robustly built, strongly typed UIs.





### on parsing story, recap/rehash
2022-03-27

1. find UI root (for app cartridge).  find explicit export roots (for app/lib cartridges). for each of these roots:
    1. parse template, discover dependencies, gen parser bin logic (`parse_to_manifest` and friends)
    2. Execute parser bin:
        1. call each root's `parse_to_manifest`, load *template*, *settings*, and *expressions* into manifest, phone home to compiler server
        2. for each dep, recurse and call that dep's parse_to_manifest
        3. finish when compiler server has all manifest info
3. transpile expressions into vtable-ready function bodies (as String), parts:
    1. invocation:
        1. importing symbols and binding values, crawling up stack as necessary
        2. casting as necessary, for types with known conversion paths (rely on Rust's `Into`?)
    2. execution:
        1. transpile PAXEL logic to Rust logic
5. Generate pax-cartridge for app cartridges only:
    1. Generate expression vtable
    2. Generate timeline vtable
    3. Generate expression dependency graph ("ROM")
    4. Generate component instance factories
    5. Generate root component instance tree




### on timeline vtable

are timelines just a special form of expression, where `t` is a parameter?  this fits nicely with other moving pieces,
e.g. gets dirty watching for free (lazy evaluation of timeline-bound properties, not re-evaluating when `t` doesn't change -- extensible to support Expression keyframes, too)

timeline-bound functions can live in the same vtable as expressions, then



### back to children/slot_children
2022-04-01



Consider the following Pax program:

```
Root's template: [stack frame ⍺ for Root]
A <Rectangle />
B <Stacker>
C     <Ellipse />
D     <Path />
- </Stacker>
  
  Stacker's template: [stack frame β for Stacker]
E <Repeat> // [stack frames γ0-γn for n RepeatItems]
F     <Frame>
G        <Placeholder>
-     </Frame>
- </Repeat>
```

In what order and in the context of which stack frame should properties be computed?
```
stack frame ⍺:
  A, B, C, D (visit each child, and their non_rendering_children recursively)
stack frame β:
  E
stack frames γ0-γn:
  F, G
  
```

Note that get_instance_children for Stacker will return E, so we first need to
first visit B's non-rendering children, C and D

To pull this off[1], we will need to perform two separate passes of the render tree.

The first will be to perform properties computation, and it will recurse via `get_adopted_children`
and `get_template_children`.

The second pass will be a rendering pass, which will recurse by `get_instance_children`



[1] (namely, without running into issues of double-computation of properties by computing
them during "slot_child" traversal, then again during "render_children" traversal, especially with the
tangles that introduces to pulling values out of the runtime stack)



//maybe:  introduce distinction between get_instance_children and
//        ... get_(what exactly?)  get_instance_children_that_aren't_slot_children
//        maybe this can be solved with lifecycle?  traverse node/property tree before
//        slot_children are linked as rendering_children?
//Another possibility: link a reference to stack frame to node — then it doesn't matter when it's
//                     computed; instead of peeking from global stack (in expression eval) it can start evaluation
//                     from the linked stackframe
//C:  keep track of whether a node has already been visited for a given frame;
//    track that either in `rtc` (with a unique ID per node) or on each
//    node (with a `last_frame_computed` register.)

        //Perhaps perform multiple traversals of the graph:
        // - compute properties
        //    - special-cases slot_children (calcs first) recurses via get_natural_children
        // - render
        //    - recurse via get_instance_children


#### Cont. 2022-04-02

store a `stack_frame_offset` with component instances.  that offset should be known statically, thus can be embedded with the compiler.
traverse `stack_frame_offset` frames up the stack (backed by `unreachable!`) before evaluating expressions.
can instantiate this STACK_FRAME_OFFSET constant inside expr. instance declarations (`|ec: ExpressionContext| -> TypesCoproduct {`...)

revert changes to rendering lifecycle; traverse / compute properties / render as before.


New problem: interpolatable properties
Not all properties will be interpolatable (though a reasonable default can be trivially derived)
We will not know easily at runtime NOR compile-time (without digging into some rustc API) whether a property is interpolatable
a `default` impl (see `rust specialization rfc`) would solve this with a blanket implementation, e.g.:

```
impl<T: Clone> Interpolatable for T {
    default fn interpolate(&self, other: &Self, t: f64) -> Self {
        self.clone()
    }
}
```

BUT, that `default` is unstable, and without it rustc won't let the blanket impl live alongside concrete defs, such as:
```
impl Interpolatable for f64 {
    fn interpolate(&self, other: f64, t: f64) -> f64 {
        self + (other - self) * t
    }
}
```

Is there some way to hook into `RenderTreeContext::get_computed_value` here?
Probably so, as long as Interpolatable is defined for all built-in / common types
So that's probably the solution -- impl `Interpolatable` for all expected property types,
with a path to implementing for 3rd party types as well



### re: Message structs for native rendering

Should they be centralized or should they be decentralized (authored as part of reusable components)

Decision: because adding native rendering commands is so _centralized_ -- namely due to the need to update several native runtimes with each change in functionality -- it was thus decided to centralize
the definitions of the drawing message structs.


### Text

1. templating live values — naively, something like `<Text>{"Index: " + i}</Text>`.  Problem: doesn't seem like it'll extend well to support styling -- this approach is "all static literal" or "all expression", whereas we probably want a bit more nuance for text.
    1. Alternatively: `{self.inline} templating, where the contents of {self.inline + "hello"} get interpolated into this string`
2. inline styling -- at least three potential approaches:
    1. class/id/settings, a la HTML (support sub-elements for e.g. `<span id=some_span>`)
    2. markdown or markdown subset: `**this is bold** and *this is italic {"and dynamic" + "!"}* and [this is a link](https://www.duckduckgo.com/)`
    3. built-in DSL/primitives for styling: `<b>Hello there!</b> Good to <i>see</i> you!`

Must be able to mix & match, too.

A priori, markdown-esque feels compelling -- in particular, with support for templating


### Click events

Ray-casting: where the user clicks, 0 or more elements will be hit by a ray running orthogonally from the point of click through the bottom-most plane of the cartridge.
The _top-most_ element that is _clickable_ receives the event

An element _higher in the hierarchy_ (ancestor) should be able to suppress descendent events — this is an _override_ setting, and the highest-in-hierarchy (most ancestral) takes precedence

Use-case: when a button is clicked, handle the click with that button, but DON'T also fire the click handler for the container
Who is responsible for specifying that?  DOM approach is to call `stopPropagation` from the descendant
But we're really describing behavior of the _ancestor_ -- in a well encapsulated scenario,
**the _ancestor_ should be responsible for discerning whether the click was "direct" or "hierarchical" and deciding whether to respond**
(rather than the desc. saying "just kidding, no event!")

An element _lower_ in the hierarchy may stop propagation imperatively with `stopPropagation()` a la DOM



### Mount events + native elements + repeat

Currently, Repeat naively re-renders the _same instances_ of elements `n` times
This is problematic specicially for native elements -- each instance that's cloned by Repeat will have the same instance_id,
which is how native counterparts are keyed & looked up

Seems that Repeat will need to do a less naive clone of elements -- perhaps RenderNode can implement `Clone` or offer `duplicate`, which copies everything but creates a new instance with new ID/etc?
Each desc. must also be cloned recursively, producing an entirely new subtree

Then, each `RepeatItem` puppeteer gets a fresh subtree, rather than pointers to the same nodes


### On tracking & identifying Repeated elements
May 6 2022


1. when an element is instantiated, there's only _one_ instance ID assigned, even if it's repeated 100 times via ancestral `Repeat`s
2. this is problematic because e.g. there will be only one `Create` event, but 100 `Update` events, all keyed to the same element ID

One hacky possibility: traverse the runtime stack, reading any `RepeatItem` frames and attaching a 'path' to each virtual instance id
Note that since we're using `int` ids, this will need to be passed as a Vec internally, and either as an int + slice or as a string across native bridge (`"15_0_0_1"`, for example)

can solve "not firing subsequent mounts" for Repeated elements by
including reduced list of `RepeatItem` indices retrieved by traversing runtime stack
Does that tuple act as a suitable, drop-in unique id?  
The major concern would be "stability" -- i.e., could the relationship between "virtual instance" and `(element_id, [list_of_RepeatItem_indices])`
change in between `mount` and `unmount`?  Namely, if the data source changes, do we expect an un/remount?  Perhaps this can be revisited with the introduction of an explicit `key`?



TO DECIDE:
- worth continuing to chew through FFI?
- Or keep a simple bridge, pass serialized bytestream for MQ?

Even if we go with JSON, it's still being passed synchronously
through shared memory -- the only costs are:
- encoding & parsing compute (time, framerate) overhead
- encoding & parsing disk footprint (measure `serde`s footprint)

Note that if we standardize on JSON, we get a parser for free on Web, i.e. no additional disk footprint for WASM bundle.


(Update 5/9/22: decided to go with FlexBuffer for macOS/iOS bridge, probably will rely on wasm_bindgen for wasm)


### On native rendering, design
May 9 2022

Consider a `Text` element nested under two repeats, each covering `(0..3)`
There will be 9 `Text` elements rendered to screen.

Core rendering will include _only one_ `Text` instance, and on each render loop
its properties will be updated in-place before it's rendered to screen.

HOWEVER -- rendering of text across the native bridge requires an awareness of each
of the virtual elements associated with a given instance.  For example, `Text` is responsible
for tracking "last-sent update patches" for each of its virtual elements, tracked by
a `HashMap<Vec<u64>, TextPatch>`



### On "media queries"
May 10 2022

Sometimes content will need to be styled separately for different platforms,
for example text might need to be tweaked to be a little larger for
an iPhone version of an app.

Similarly to the CSS approach of media queries, we can enable per-platform
per-screen-size, or even per-`Expression` (boolean return values)

```
@settings {
    @macos, @ios: {
        #some_element: {
            ...
        }
        ...
    }
    
    @windows: {
        #some_element: {
            ...
        }
        ...
    } 
}
```

or

```
@settings {
    {screen.width < 500px}: {
        #some_element: {
            ...
        }
        ...
    }
    {self.condensed_mode}: {
        ...
    }
    {self.is_active}: {
        ...
    }
}
```

Perhaps this final syntax would best suit an explicit `if`?


```
@settings {
    if screen.width < 500px {
        #some_element: {
            ...
        }
        ...
    }
    if self.condensed_mode {
        ...
    }
    if self.is_active {
        ...
    }
}
```



### On native clipping, web

One approach:
- keep a hierarchy of `div` with overflow: hidden -- these are stacked within each other
  and allow adding child nodes that benefit from the clipping.  Note that the transformation of each
  nested element will need to compensate for the transform of its descendants (because each nesting sets a new origin, managed by browser)
  Thus, there's inherent inefficiency to this approach -- many unnecessary matrix calculations

- calculate the clipping bounds manually via aggregate intersection of clipping rects; use CSS `clip-path` with a `polygon` value
  This extends very well to non-rectilinear masks (unlike nested `overflow: hidden` divs)

- Use SVG: generate a representation of clipping masks hierarchically in "yet another layer"
  of SVG, superimposed (likely invisibly), point to individual nodes
  see: https://css-tricks.com/clipping-masking-css/#aa-using-clip-path-with-an-svg-defined-clippath

  Note, the above appears viable, though wrapper divs are required to contain each `clipPath`,
  and those wrapper divs will need to be managed by the chassis.  See PoC:

```
<div id="outer">
  <div id="inner">
    <img src="https://s3-us-west-2.amazonaws.com/s.cdpn.io/3/Harry-Potter-1-.jpg" alt="">
  </div>
</div>

<svg width="0" height="0">
  <defs>
    <clipPath id="myClip" >
      <circle cx="100" cy="100" r="40"/> 
    </clipPath>
    <clipPath id="myClip2">
      <circle cx="70" cy="70" r="40"/>
    </clipPath>       
  </defs>
</svg>

<style>
  img {
  width: 120px;
  margin: 20px;
}

#outer{
  clip-path: url(#myClip);
}

#inner {
  clip-path: url(#myClip2);
}

body, html {
  height: 100%;
}
</style>
```



### Cartridge properties
May 12 2022

Certain properties make sense to expose at the root of a given cartridge.

These should be reasonably straight-forward to support.  Probably they can be declared in some sort of
manifest file, e.g. a .paxrc/pax.json/pax.toml/pax.yaml.  Alternatively, these could be
managed within the pax language, with the added benefit of dynamic evaluation (e.g. for updating title content, bg color, maybe even framerate)

Some examples:
- Background color (or transparency)
- Title label for app cartridges (e.g. HTML title, macOS title bar)
- Target frame rate

### On scrolling
May 16 2022

Scrolling: a clipping frame, with a translatable view underneath
Also, native elements potentially nested (including other frames or even, maybe, other scrollviews (pending exploration of behavior for nested scrolling))

Is this a primitive?  Or a component?
On macos, the atomic instantiation is 1. scrollview, plus 2. Content view —
suggesting it isn't reasonable to separate the two.

Frame
- size, transform
Content
- transform (translate x/y)

In SwiftUI, this is a View with a specified size (plus any relevant, positioned native elements like Text)
On Web, this is a div with invisible background and fixed width/height, containing any relevant native elements like Text, inside another div with `overflow-x/y: auto/hidden;` for the frame

content_size
frame_size
scroll_x
scroll_y
transform
children

When instantiated, inject a `Group` above the passed-in `children`, and
store this modified tree.  Update the `Group`s transform (with an imperative `set`) when scroll changes are notified, by input events.  
The native layer will handle its own positioning of native elements within the scroller

On native side:
- attach a ScrollView and content view; attach native

It would be particularly nice to calculate the
`InnerPane`s size automatically.  This could be tracked
by engine during traversal, alongside how it currently tracks
`accumulated_bounds` for layout calculation.  That is:
a render



### On hit-testing, ray casting
May 18 2022


Should groups be bindable to events?
yes.  the logical behavior is "if any of this group's contents
pass hit-test, then the group passes hit-test"



Capture/Bubble with override control
OR: "top-down" control, where a parent may prevent a child from
handling certain events




Another take:


Find top-most element underneath ray -- this is the target.


Traverse ancestors to check if any `absorb(Click)`
- if so, the topmost-such element receives the `Click` event insteadx
Dispatch event to its
ancestors



### Survey of event mechanisms
2022-05-19

#### Web/DOM

`Capture` phase and `Bubble` phase -- every dispatched event first fires in `Capture` from root -> target, the fires again in `Bubble` from target -> root.
Any element may stopPropagation

See https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Examples#example_5_event_propagation

Pros: this allows fine-grained control of event cancellation; familiar to web devs
Cons: little clumsy; not well encapsulated'


#### Qt

See https://www.learnqt.guide/events/working-with-events/#event-propagation

> Qt will try to propagate the event up the parent child relationship chain until it finds a handler willing to deal with the event. If that handler is not found, then the event is discarded or fully ignored.

`accept()` and `ignore()` allow multiple handlers for a given event --
to `accept()` is to suppress further handlers, whereas to `ignore` allows
ancestors also to process the event.


#### Xamarin

Uses imperative `GestureRecognizers`, attached to specific instances
(Similar to SwiftUI!)

XAML example:
```
<Image Source="tapped.jpg">
    <Image.GestureRecognizers>
        <TapGestureRecognizer
                Tapped="OnTapGestureRecognizerTapped"
                NumberOfTapsRequired="2" />
  </Image.GestureRecognizers>
</Image>
```

Is there any sort of propagation?  Looks like manual piping only:
e.g. `Command="{Binding Source={x:Reference AlertsListView},
Path=BindingContext.YourCommandName}"`
See: https://stackoverflow.com/questions/68178176/nested-grid-tapgesturerecognizer-not-working
and: https://social.msdn.microsoft.com/Forums/en-US/e7dd1b34-0283-4a2b-8df5-dab879b57efd/listview-with-mvvm-tapgesturerecognizer-not-working?forum=xamarinforms

We can do better than this!!


#### SwiftUI


Event collisions may be:
`Simultaneous`, `Sequenced`, `Exclusive`

> While working with gesture recognizers, we might find ourselves having multiple gestures recognizers on the same view. And for such situations, we need exactly to know how those interact with each other. SwiftUI allows us to handle such cases in three-way: Simultaneous, Sequenced, Exclusive.

See: https://dreamcraft.io/posts/swiftui-mixing-gesture


Note that this requires explicit coordination between declarations,
which might not work well with encapsulated components

THAT SAID -- these three modes excellently describe the families
of use-cases for handling collisions.

Perhaps these can be
built upon in a more declarative, encapsulation-friendly way:
`Simultaneous`, `Sequenced`, `Exclusive`

(Note that in a single-threaded environment, `Simultaneous`
and `Sequenced` are... technically the same thing.  it seems that
SwiftUI offers the distinction as a specific provision for e.g. combining `Zooming` and `Panning` in an intuitive way.)

So for us, these boil down to "fire first," (à la Capture)




### Node cache & tab cache

Instead of `get_rendering_subtree_flattened`:

- during engine traversal, for each "virtual element" rendered, add to a global cache of
    - id_chain
    - InstanceNodePtr
    - parent InstanceNodePtr
    - (computed properties? (PropertiesCoproduct))


- during `interrupts`, consult this populated cache of "virtual elements", e.g. for hit-testing


Maybe this belongs in the `node_registry`?


(cont. May 21)

Nomenclature for "expanded" or "virtual" nodes,
vs. "base" or "instance" or "concrete" nodes

ex.
`VirtualNode {}`
`parent_expanded_node`
`expanded_node_cache: HashMap<Vec<u64>, Rc<VirtualNode<R>>>,`

Problematically, `virtual` could arguably be applied to both the `raw instance` and any `virtual` nodes

`aggregated`?  `expanded`?  `hydrated`? `filled`?
`unfolded`?

`hydrated node` vs `instance node` ?  trying it on




### Async methods

Use-case: load some data over HTTP; set a local Property; don't block rendering thread
Problem: without support for threading certain methods (or all methods...?), the only way to achieve something like 
         HTTP loading would require blocking the rendering thread

Reading:

 - https://internals.rust-lang.org/t/how-to-define-async-methods-without-capturing-lifetime-of-self/10708

    > There is no way to annotate an async fn to not capture a lifetime in it, and inferring that would go against Rust's current philosophy that function signature's should be able to be relied on independently of the body (other than auto-trait leakage in RPIT).
    >
    > You can instead make bar a normal fn returning an impl Future and use an async block to define it:
    > ... (see above link)
   
Approaches:

1. Channel for properties (or parallel version of `SelfChannel`)

```rust

#[pax(
    <AnimatedBanner>{banner_message}</AnimatedBanner>
)]
pub struct HelloWorld {
    pub banner_message : Property<String>,
}

impl HelloWorld {
    
    #[pax_on(PostMount)]
    pub async fn lifecycle_load_banner_message(self) {
        let data = do_http_things().await.unwrap();
        self.banner_message.set(data.new_message)
    }
    
}


// Alt: make `Property` itself alias a channel-brokering object, with same `.set` API, but without
// `&mut self` or even `&self` requirements

```
Work:
[ ] Refactor `Property<T>` definition to be a channel container rather than raw `Box<>`s
[ ] Figure out where to store the canonical, owned data (Perhaps an Rc<RefCell<>>?)
[ ] Refactor `example` etc. to deal with `self`, and prove ability to spawn "disposable selfs" in dispatch logic 
    (create a basket of channels, along with listening/data-gathering & cleanup logic, plus patch data)

Perhaps each `PropertyChannel` instance has two modes: 
1. "zergling" | "drone" where it's simply a broadcaster
(this is the "mode" that is baked into the cloned "channel basket" cloned instances)
2. "hatchery" | "hive" where it knows how to
   1. spawn baskets
   2. listen to its spawned baskets for updates
   3. store canonical data, and patch that data via updates from `zerglings`  


2. Callback?

```rust


pub async fn do_async_things() {
    fetch_http_data().await //T = DataType with member `server_message`
}

impl HelloWorld {
    
    #[pax_on(PreMount)]
    pub fn mount(&mut self) {
        pax_engine::async(do_async_things, self::http_callback);
    }
    
    pub fn http_callback(&mut self, args: ArgsCallback<DataType>) {
        self.message.set(args.data.server_message)
    }
    
}



```



## Archived lab journal from from pax-macro/.../lib.rs
Probably authored early 2022
//zb lab journal:
//   This file including trick does NOT force the compiler to visit every macro.
//   Instead, it (arguably more elegantly) forces Cargo to dirty-detect changes
//   in the linked pax file by inlining that pax file into the .rs file.
//   This is subtly different vs. our needs.

//   One way to handle this is with an idempotent file registry — the challenge is
//   how do entries get removed from the registry?

//   Another possibility is static analysis

//   Another possibility is a static registry — instead of phoning home over TCP,
//   each macro generates a snippet that registers the pax parsing task via code.
//   When the cartridge is run (with the `designtime` feature), that registry
//   is exposed to the compiler, which is then able to determine which files to parse.
//   This works in tandem with the dirty-watching trick borrowed from Pest —
//   the "static registry" assignment will exist IFF the macro is live.
//
//   Note: possibly problematically, this "dynamic evaluation" of the manifest requires
//         happening BEFORE the second cargo build, meaning the binary is run once (with blanks),
//         evaluated to pass its macro manifest, then patched+recompiled before ACTUALLY running
//   Perhaps this deserves a separate feature vs. `designtime`
//   Alternatively:  is there a way to fundamentally clean this up?

//   Another possibility: keep a manifest manually, e.g. in JSON or XML or YAML

// v0? ----
// Keep a .manifest.json alongside each pax file — the #[pax] macro can write to the companion file
// for each pax file that it finds, and it can encode the relevant information for that file (e.g. component name)
// The compiler can just naively look for all .pax.manifest files in the src/ dir
//
//Along with the "force dirty-watch" trick borrowed from Pest, this technique ensures that .manifest.json can
//stay current with any change.
//
//Sanity check that we can accomplish our goals here
// 1. generate PropertiesCoproduct for subsequent compilation,
// - codegen a PropertiesCoproduct lib.rs+crate that imports the target crate's exported members
// - codegen a "patched" Cargo.toml,
// 2. choose which .pax files to parse, and which ComponentDefinitions to associate a given .pax file with
// - refer to manifests for precise fs paths to .pax files
//
// Limitation: only one Pax component can be registered per file
// Refinement: can store a duplicate structure of .pax.manifest files inside the local .pax directory
//             that is, instead of alongside the source files in userland
// Finally: this could be evolved into an automated, "double pass" compilation, where `pax-compiler` orchestrates
//          fancy metaprogramming and message-passing (thinking: a special feature flag for the first pass, which
//          aggregates data and hands off to the second pass which operates under the `designtime` feature.)
//
// To recap: - during initial & standard compilation, generate .pax/manifests/{mimic structure of src/}file.manifest
//           - before designtime compilation: generate pax-properties-coproduct, Cargo.toml
//           - parse pax files and prepare data for dump
//              (Advantages of waiting until cartridge is running:
//                [will fail parsing more gracefully; will better transition to compiler-side RIL generation])
//           - perform second compilation; load resulting lib into demo chassis
//           - dump parsed data to demo chassis; start running
//             [Refinement: in the future when RIL is generated, this initial dump could be avoided]
//
// twist! might not be able to reliably get FS path from inside macros (see proc_macro_span unstable feature, since 2018)
//    Spitballing a possible approach using multi-stage compilation:
//     - macro generates functions behind a special feature "stage-0" that perform a TCP phone-home:
//          - call file!() at runtime, allowing reliable resolution
//          - pass file path for .pax file
//          - pass struct path (& module? TBD but probably std::module_path) for properties coproduct generation

// 1. compile cartridge with `parser` feature
//  - each #[pax] macro
//  ? how do files get into the tree?  Can we rely on the root file & its imports?
//    note that resolving our deps requires traversing Component templates — this probably means
//    we need to parse the templates *at this phase* so that each macro can 'phone home' for `parser`
//    i.e. unroll dependencies from Pax into Rust so that the compiler can visit all the necessary files
//  - THEN - either by evaling logic as a side-effect of importing (is this possible?) or by
//    conventionally importing a certain-named entity that abides by a certain impl (and calling a known method)
//    then each macro communicates its relevant data: file path, module path, name (and later: methods, properties)

// Then too... if we're going to be parsing the pax tree in order to determine which modules to resolve, maybe we don't need
// a separate manifest-generating step after all?  (Except we still need to generate the PropertiesCoproduct).


### Compiling expressions
2022 10 03

How do we represent NPIT in memory?
      — use actual instance data structures? (along with dep. on core)
        ^ probably not.  the core dep. gets too messy & complicated, especially with Rust _mise en place_.
      — create companion data structures for NPIT nodes, including:
         - instantiation args info (for ser. into RIL Instance declarations)
         - node / property types (as strings, for codegen — must be able to generate `compute_properties_fn`)
         - template, traversable

Decision:  introduce a `compile_expressions` method on `PaxManifest`, as well as an `Option<T>`-wrapped field 
           for storing computed expression specs.  In the future, expressions may be rebuilt individually, e.g. for hot reloading


### Helper / function-call (plus dirty watch) API

2022-10-19

Use-case: Stacker, without the `pax_on(WillRender)` hack.

Possible approaches:

#### 1. Bind a property with PAXEL in the struct declaration
 ```
pub struct Stacker {a

    //can `i` below be used implicitly, but only in a scope where `i` exists?
    //or should it be passed explicitly, as a "special" argument? (`translate(i)`)
    //neither feels ideal, but the former is more powerful
    //...another possibility is using a `let` statement or similar in template,
    //   which would require figuring out return type of expression (contrast with leveraging Rust's type system, via `Property<T>`)
    //   so this approach isn't ideal.
    #[paxel( self.cells * (i / n) )]
    pub helper_translate: Property<f64>,

}
````
 Pros:
 - Solves typing (`T` in `Property<T>`)
 - Feels conceptually sound à la spreadsheet
 Cons:
 - gets confusing with dynamic scopes, like `(elem, i)` with `Repeat`

1a:  One with-the-grain way to approach expression declarations is with `@defaults` —
```
#[pax(
    
    @defaults {
        foo: {
            Bar::random()
        }
    }
)]
pub struct Hello {
    pub foo: Property<Bar>,
}
```

Same pros & cons as above


#### 2. Call a function directly on impl
Probably the winner
```
impl Stacker {
    pub fn get_transform(index: usize, bounds: (Size2D, Size2D)) -> Transform2D {
        
    }
}
```
 Pros:
 - Feels intuitive
 - Gets to use arbitrary Rust, particularly useful with complex types
 - Provides nice escape hatch for imperative / complex logic
 Cons:
 - Requires managing argument types, maybe just with `.into()`
 - Probably requires manually annotating stream dependencies, like `#[pax_watch($bounds)]`
   - Alternatively: require that it's an assc fn, no `self`, and that each dep must be wired in as arg.  This means the host expression's dirty-watching & scoped resolution will _just work_, because the relevant symbols are made explicit when calling the function
 - Might encourage (erroneously) writing side-effectful code, and or expecting it to trigger at the wrong time
 - Requires tediously wiring up dependencies... (could this be automated with a macro on the function, which naively looks for all `self.` references?)
```
#[pax_helper]  //through naive static analysis, we can infer that `\self\..*\`, like `self.some_symbol`, is a "dependency" of this calculation
pub fn get_transform(&self) {
    let x = self.some_symbol + 6;
}
```
- What about stack-introduced symbols like `i` or `elem`?  These could be introduced manually/magically like:
```
pub fn get_transform(&self) {
    let i = pax_scoped!();
    let elem = pax_scoped!();
}
```
Or, more reasonably, if still not 100% ideally, these could be passed as explicit arguments like:
```
pub fn get_frame_transform(&self, i: usize, container: (Size2D, Size2D)) {
    
}
```
and called as such:

```
<Frame transform={get_frame_transform(i, $container)} ...
```

 

#### 3. Let statements / scoped temporaries

```
#[pax(
    for i in 0..self.cells {
        let cell_width = ($bounds.0 - ((cells + 1) * gutter)) / cells
        let cell_height = ($bounds.1 - ((cells + 1) * gutter)) / cells
        <Frame transform={get_transform(i)} size={(cell_width, cell_height)}>
            slot(i)
        </Frame>
    }
)]
```

Pros:
 - Makes use of PAXEL and spreadsheet model
 - familiar imperative-style logic
Cons:
 - Requires figuring out types
 - seems to promote complex and ternary-nested logic
 - Requires managing stack frames / etc.





### Binding symbols from Repeat

Example of casting an `elem` and `i` from a RepeatItem:
```
let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {

    (Rc::clone(datum), *i)
} else { unreachable!(1) };

let datum_cast = if let PropertiesCoproduct::StackerCell(d)= &*datum {d} else {unreachable!(1)};
```

Note: in other to invoke a cast datum OR index from a RepeatItem, the compiler
must be aware that a given symbol maps to RepeatItems.


### Reflecting on the T in `Vec<T>`

Consider `for elem in self.some_iterable`

We must know the type of `elem` (and it must be a member of PropertiesCoproduct, either as a primitive type or a `pax_type`-annotated type)
so that we may access that elem inside expressions — consider the vtable entry:

```
//Frame size y
vtable.insert(3, Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
    let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {

        (Rc::clone(datum), *i)
    } else { unreachable!(3) };

    let datum_cast = if let PropertiesCoproduct::StackerCell(d)= &*datum {d} else {unreachable!()};

    return TypesCoproduct::Size(
        Size::Pixels(datum_cast.height_px)
    )
}));
```

Some approaches:
- could naively statically analyze it, e.g. pull the contents of the outermost `<>`s.
- could require annotation by author, e.g. `for (elem: StackerCell, i) in self.properties`
- could punt on iterating over anything other than `usize` ranges for now — could at least hack a solution
where type annotations are explicit.  Alternatively, all of the `elem` iteration behavior
is available by array access with `i` — `some_collection[i]`
- could introduce PropertyVec<T>, which offers a Vec-like API and knows how to reflect and offer "T"
- might be able to impl a new parser method via traits, populating an Optional field representing `iter`'s `<T>` if present
  - something like `impl<T> IterableQualifiable for Iter<T> where T: Reflectable { fn get_iter_type() -> String {...} }`
- could hard-code support for `Vec`, special-handling pulling the `T` out of `Property<Vec<T>>` or `Property<std::vec::Vec<T>>`, and later extending that support to other built-ins.  




### When binding repeat...

Compile source as an expression; infer type based on "range => usize; Vec<T> => T" heuristic

resolve symbol:
 - consult scope_stack
 - provide an ExpressionSpecInvocation

When compiling an expression, we need to be able to "bind a symbol" — that is, 
from a string identifier like `elem`, we want to retrieve a stack offset that will allow that id to 
be dynamically passed into the runtime stack before performing a string symbol lookup 


## elem
 becomes an effective PropertyDefinition -- in particular, its type
 metadata is needed by `compile_paxel_to_ril` so that it can use that type data
 in generating vtable entries, like:

```

let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {

            (Rc::clone(datum), *i)
        } else { unreachable!(3) };

        let datum_cast = if let PropertiesCoproduct::StackerCell(d)= &*datum {d} else {unreachable!()};

        return TypesCoproduct::Size(
            Size::Pixels(datum_cast.height_px)
        )
        
```

That type data must be retrieved or inferred somewhere, likely in the `parsing` phase, which
is the only time that we have reflection available to us.

### Where to store this data

Should this type be added to the PropertiesCoproduct or the TypesCoproduct?  Arguably the distinction
is becoming less and less valuable — PropertiesCoproduct represents "inputs" so perhaps it's best to keep it there,
but PropertiesCoproduct also represents "aggregates" (component structs) while TypesCoproduct is a
collection of atomic types

TypesCoproduct also includes logic for case manipulation and storing complex types, like Vec<T<R>>, while PropertiesCoproduct
is focused on storing simple types (like `Stacker` or `Rectangle`).  This distinction alone seems to suggest
that we use the TypesCoproduct for storing the T<R> of Property<Vec<T<R>>


### `designtime` sketches

2022-12-15
pulled from `pax-std/.../rectangle.rs` during a cleanup pass

```
//
// #[cfg(feature="designtime")]
// lazy_static! {
//     static ref RECTANGLE_PROPERTIES_MANIFEST: Vec<(&'static str, &'static str)> = {
//         vec![
//             ("transform", "Transform"),
//             ("size", "Size2D"),
//             ("stroke", "Stroke"),
//             ("fill", "Color"),
//         ]
//     };
// }
//
// #[cfg(feature="designtime")]
// impl Manifestable for RectangleProperties {
//     fn get_type_identifier() -> &'static str {
//         &"Rectangle"
//     }
//     fn get_manifest() -> &'static Vec<(&'static str, &'static str)> {
//         RECTANGLE_PROPERTIES_MANIFEST.as_ref()
//     }
// }
//
// #[cfg(feature="designtime")]
// impl Patchable<RectanglePropertiesPatch> for RectangleProperties {
//     fn patch(&mut self, patch: RectanglePropertiesPatch) {
//         if let Some(p) = patch.transform {
//             self.transform = Rc::clone(&p);
//         }
//         if let Some(p) = patch.size {
//             self.size = Rc::clone(&p);
//         }
//         if let Some(p) = patch.stroke {
//             self.stroke = p;
//         }
//         if let Some(p) = patch.fill {
//             self.fill = p;
//         }
//     }
// }
//
// #[cfg(feature="designtime")]
// pub struct RectanglePropertiesPatch {
//     pub size: Option<Size2D>,
//     pub transform: Option<Rc<RefCell<Transform>>>,
//     pub stroke: Option<Stroke>,
//     pub fill: Option<Box<dyn Property<Color>>>,
// }
//
// #[cfg(feature="designtime")]
// impl Default for RectanglePropertiesPatch {
//     fn default() -> Self {
//         RectanglePropertiesPatch {
//             transform: None,
//             fill: None,
//             size: None,
//             stroke: None,
//         }
//     }
// }
//
// #[cfg(feature="designtime")]
// impl FromStr for RectanglePropertiesPatch {
//     type Err = ();
//
//     fn from_str(_: &str) -> Result<Self, Self::Err> {
//         unimplemented!()
//     }
// }
//
```



### Lab journal, Jan 8 2023

Problem: `root_template_node_id` is None for stacker, encountered 
in the context of compiling expressions

Separately, for future consideration, is it possible / desirable to merge the expression compilation logic
into the _same traversal_ as the original parse?  That is: populate
expression_specs inside the parser binary, and pass those serialized
expression_specs over the wire (instead of compiling them as a separate
step after loading a Manifest)


### Jan 13 2023

```
error[E0277]: the trait bound `R: piet::render_context::RenderContext` is not satisfied
  --> /Users/zack/code/pax/pax-example/.pax/cartridge/src/lib.rs:29:94
   |
29 | pub fn instantiate_expression_table<R: 'static + RenderContext>() -> HashMap<u64, Box<dyn Fn(ExpressionContext<R>) -> TypesCoproduct>> {
   |                                                                                              ^^^^^^^^^^^^^^^^^^^^ the trait `piet::render_context::RenderContext` is not implemented for `R`
   |
note: required by a bound in `ExpressionContext`
  --> /Users/zack/.cargo/registry/src/github.com-1ecc6299db9ec823/pax-runtime-0.0.1/src/expressions.rs:97:47
   |
97 | pub struct ExpressionContext<'a, R: 'static + RenderContext> {
   |                                               ^^^^^^^^^^^^^ required by this bound in `ExpressionContext`
help: consider further restricting this bound
   |
29 | pub fn instantiate_expression_table<R: 'static + RenderContext + piet::render_context::RenderContext>() -> HashMap<u64, Box<dyn Fn(ExpressionContext<R>) -> TypesCoproduct>> {
   |                                                                +++++++++++++++++++++++++++++++++++++
```
Problem: `pax#0.0.1` uses Piet 0.5.0, whereas `pax#master` uses Piet 0.6.0.  This results in the error above.

Options: 
 - For "dev mode," special-case introduce relative paths, so that `pax-example/.pax/` generated code refers to local fs, which would resolve to piet#0.6.0
 - Publish a 0.0.2 for each package, which will include updated deps to 0.6.0

cont. Jan 17:

 - special-case relative paths for patched entities (cartridge, coproduct) — they should _still_ be stripped of relative paths even when `is_lib_dev_mode`
 - figure out hierarchy / path-flatness of Chassis; requires an extra `../`

### Jan 18 2023

Problem:
```
Execution failed (exit code 101).
/Users/zack/.cargo/bin/cargo metadata --verbose --format-version 1 --all-features --filter-platform aarch64-apple-darwin
stdout :     Updating crates.io index
error: package collision in the lockfile: packages pax-properties-coproduct v0.0.1 (/Users/zack/code/pax/pax-example/.pax/properties-coproduct) and pax-properties-coproduct v0.0.1 (/Users/zack/code/pax/pax-properties-coproduct) are different, but only one can be written to lockfile unambiguously
```

When building `pax-example/.pax/chassis/MacOS`, we get a collision of `pax-properties-coproduct`.  Why?

1. we're patching pax-properties-coproduct 0.0.1 to refer to our relative, codegenned properties-coproduct at `.pax/properties-coproduct`.
2. Meanwhile, `pax-runtime` refers to a relative path for `pax-properties-coproduct`, `@/pax/pax-properties-coproduct`.  It appears that `patch` doesn't
   work alongside a relative path.  This can be validated by removing `path = ../pax-properties-coproduct` from pax/pax-runtime/Cargo.toml —however, then we can't build the core library by itself!

Possible options:
    Copy `pax-runtime` into the `.pax` codegen folder (along with everything else, probably!)
    Deal with a library that doesn't build standalone (blech)
    Point to `.pax/properties-coproduct` even for core lib deps!  e.g. pax-runtime::Cargo.toml can refer to path
    Revisit lib_dev_mode: punt for later, just rely on crates.io for pax-example/.pax projects

Conclusion, as of 1/18:
    Point to the generated `.pax` cartridge & PropCop as the relative dirs 
    Also pass `PAX_DIR` env into any core-lib cargo process
    (e.g. this entails configuring IDE, and has implications about ever removing `pax-example/.pax` from version control



### Lab journal, Feb 2 2023

Broke through with codegen to compiling into runtime lib —
specifically, pax-std-primitives is surfacing a couple of unfinished 
details around PropertiesCoproduct definition.

If a program uses a Pax-exporting crate like `pax-std`,
BUT that program does not use all of the members exported from
that crate (like `Text`, say) does not generate an entry
into the generated PropertiesCoproduct and/or TypesCoproduct

Thus, calls into that data structure that are made where 
they aren't valid.  

Options:
 a.) Require a round-up reexport at root of crate (similar to existing `pax_*` macros)
   - this allows the compiler to build a PropertiesCoproduct entry for each component type
     in a crate, without requiring each type to be used in the actual compiled project
   - this might look like:
```
#[pax_crate([
    frame::Frame,
    group::Group,
    rectangle::Rectangle,
    text::Text,
    scroller::Scroller, 
])]
```



 b.) Expose a cargo feature for each primitive in `pax-std-primitives`; 
     based on which primitives are used, encode those features into the generated cargo.toml for `pax-cartridge` (which is consuming `pax-std-primitives`)
     gate any `TypesCoproduct` or `PropertiesCoproduct` logic in `pax-std-primitives` with a feature-check for that primitive
     ^
     | This is probably the best way forward 
 
Final note: (b) has been penciled in as viable with `Text`; more work
remains to finish-line the feature-gating labor / wiring


 c.) unsafe cast?  perhaps we can unsafe-cast the PropertiesCoproduct 
     into the internal data structure, without using `match` and an explicit PropertiesCoproduct::type
     Though this introduces unsafety, it is presumably as bulletproof as the compiler logic that assembles it
     (i.e. ensuring that a PropertiesCoproduct::Text is associated with a node IFF it is a Text node)
     thus this unsafety could be mitigated with e.g. unit tests of the compiler.
     
     This approach might require using `#[repr(C)]` on the PropertiesCoproduct
     and then manually reaching into memory to pluck out the datum from the disc. union

### Lab journal zb, 2023-02-19

Consider the snippet:
```
for i in 0..4 {
        <Group transform={Transform2D::rotate(i * 0.07)} >
```
`i` is a `usize` (or perhaps `isize`, TBD)
`0.07` is an `f64`.  In the current state of codegen,
`i` is multiplied by `0.07`, yielding the rustc error:
```
81 | ...e::Percent(50.into())),))*Transform2D::rotate(((i *0.07.try_into().unwrap())),))
   |                                                      -     ^^^^^^^^
   |                                                      |
   |                                                      type must be known at this point
```

Possible approaches:

1. Numeric wrapper

Introduce a new type `Numeric` with `From` and `Into` built for each primitive types across Rust's `{floats, integers}`
Fill a truth table describing the cross-product of `set of all pax operators` ⨯ `operand 0 from domain {Numeric::Float, Numeric::Integer}` ⨯ `operand 1 from domain {Numeric::Float, Numeric::Integer}`
e.g., for multiplication:

```
MULTIPLICATION operator (`x`)
`Float` `x` `Float` => `Float`
`Integer` `x` `Float` => `Float`
`Float` `x` `Integer` => `Float`
`Integer` `x` `Integer` => `Integer`
```
(and so on, for remaining supported operators.  consider the unary operator `-`, also)

Further, implement `Mul<Numeric>`, etc., for each Rust operator that we ever
generate literally, e.g. incl. `%`


## Lab journal, Mar 13 2023

Pursued some improvements to macOS build today.  In particular,
focused on enabling multi-arch builds, partly to chase down the errors coming 
from xcodebuild:

```
Ld /Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/InstallationBuildProductsLocation/Applications/Pax\ macOS.app/Contents/MacOS/Pax\ macOS normal (in target 'Pax macOS' from project 'interface')
    cd /Users/zack/code/pax/pax-example/.pax/chassis/MacOS/interface
    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang -target x86_64-apple-macos12.0 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX13.1.sdk -L/Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/IntermediateBuildFilesPath/EagerLinkingTBDs -L/Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/BuildProductsPath/Debug -L/Users/zack/code/pax/pax-example/.pax/chassis/MacOS/interface/../target/debug -F/Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/IntermediateBuildFilesPath/EagerLinkingTBDs -F/Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/BuildProductsPath/Debug -filelist /Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/IntermediateBuildFilesPath/interface.build/Debug/Pax\ macOS.build/Objects-normal/x86_64/Pax\ macOS.LinkFileList -Xlinker -rpath -Xlinker @executable_path/../Frameworks -Xlinker -object_path_lto -Xlinker /Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/IntermediateBuildFilesPath/interface.build/Debug/Pax\ macOS.build/Objects-normal/x86_64/Pax\ macOS_lto.o -Xlinker -export_dynamic -Xlinker -no_deduplicate -Xlinker -final_output -Xlinker /Applications/Pax\ macOS.app/Contents/MacOS/Pax\ macOS -fobjc-link-runtime -L/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx -L/usr/lib/swift -Xlinker -add_ast_path -Xlinker /Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/IntermediateBuildFilesPath/interface.build/Debug/Pax\ macOS.build/Objects-normal/x86_64/Pax_macOS.swiftmodule -lpaxchassismacos -lpaxchassismacos -Xlinker -dependency_info -Xlinker /Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/IntermediateBuildFilesPath/interface.build/Debug/Pax\ macOS.build/Objects-normal/x86_64/Pax\ macOS_dependency_info.dat -o /Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/InstallationBuildProductsLocation/Applications/Pax\ macOS.app/Contents/MacOS/Pax\ macOS
ld: warning: ignoring file /Users/zack/code/pax/pax-example/.pax/chassis/MacOS/target/debug/libpaxchassismacos.dylib, building for macOS-x86_64 but attempting to link with file built for macOS-arm64
Undefined symbols for architecture x86_64:
  "_pax_dealloc_message_queue", referenced from:
      _$s9Pax_macOS0A10CanvasViewC4drawyySo6CGRectVF in PaxView.o
  "_pax_init", referenced from:
      _$s9Pax_macOS0A10CanvasViewC4drawyySo6CGRectVF in PaxView.o
  "_pax_interrupt", referenced from:
      _$s9Pax_macOS0A4ViewV4bodyQrvgy7SwiftUI11DragGestureV5ValueVcfU0_ySWXEfU_ySPySo15InterruptBufferVGXEfU_ in PaxView.o
  "_pax_tick", referenced from:
      _$s9Pax_macOS0A10CanvasViewC4drawyySo6CGRectVF in PaxView.o
ld: symbol(s) not found for architecture x86_64
clang: error: linker command failed with exit code 1 (use -v to see invocation)

** ARCHIVE FAILED **


The following build commands failed:
	Ld /Users/zack/Library/Developer/Xcode/DerivedData/interface-ceupzrmwryakhafqxlfbcvhpgytw/Build/Intermediates.noindex/ArchiveIntermediates/Pax\ macOS/InstallationBuildProductsLocation/Applications/Pax\ macOS.app/Contents/MacOS/Pax\ macOS normal (in target 'Pax macOS' from project 'interface')
(1 failure)
```

Things I did:

1.) explored path of using conditional LIBRARY_SEARCH_PATHs in the xcode project definition by manually editing file -- see README in cargo-lipo project for a pointer. Ultimately was unable to get xcode to resolve the dylib(s) appropriately when building. Might be surmountable with more config twiddling — note that this required editing the .pbxproj file by hand

2.) explored using cargo lipo to build a "fat binary" including all necessary arch builds information, then using that single embed (presumably, dylib file) for xcode. Problem: I was only able to get a `.a` file out of cargo lipo, rather than a `.dylib` (was able to get two `.dylibs`, across target/x86_64-apple-darwin and target/aarch64-apple-darwin, but didn't verify whether either of them was unexpectedly the "fat lib.". If that single `.a` file is enough, need to figure out how to get xcode project to build around that, which was not an immediately intuitive refactor.

3.) yet another option, unexplored: make mac x86_64 (intel) a completely different chassis or harness pair, vs. the aarch64 chassis / harness-pair.  Might even template or codegen out multiple variants of the macos chassis, swapping just architecture strings in the .pbxproj file.

In any case, when supporting cross-arch builds (building for other flavors of Mac from either Intel or ARM), steps like the following will be needed:
(previously added these to README while working on this, in hopes of shipping, but
ended up pulling back and dropping this lab journal entry instead.)

```
- run `rustup target add aarch64-apple-darwin`
- run `rustup target add x86_64-apple-darwin`
- install cargo-lipo with `cargo install cargo-lipo` (needed for cross-architecture builds, `x86_64` and `aarch64`)
```


## Lab journal, Mar 21 2023

On Range, Repeat, and Vec

Repeat Data Source is handled as an expression, e.g. `self.some_vec` or a range `0..5`/`0..self.max`

That expression currently returns a `Vec<PropertiesCoproduct>`, which is used by `Repeat` to iterate and attach (a copy of) that
PropertiesCoproduct to each stack frame, binding `elem` and `i` for use in expressions within the scope of that Repeat (child / desc. elements)

1. _Probably_, inside Repeat, we should deal with an `dyn Iterator` instead o `Vec`.  This will allow us to return a bare range from range expressions, instead of hacking + repackaging into a Vec

Problem:  
In practice, this looks like:
```
pub source_expression: Box<dyn PropertyInstance<Box<dyn Iterator<Item = Rc<PropertiesCoproduct>>>>>,
```

Because it's a `PropertyInstance<T>`, `T` must implement `Default`.  


Ex. of accepting
```
fn find_min<'a, I>(vals: I) -> Option<&'a u32>
where
    I: IntoIterator<Item = &'a u32>,
{
    vals.into_iter().min()
}

```

Update, Apr 12:  Decided to handle `range` and `vec` `PropertyInstance`s separately.
See `TemplateArgsCodegenCartridgeRenderNodeLiteral` and its fields
`repeat_source_expression_literal_vec` and `repeat_source_expression_literal_range`




## On nestable properties

Apr-May 2023

#### Use-case: 

Iterating over structs with `for struct_instance in some_vec_of_structs`, then using `elem.some_property` in PAXEL

We need this for `<Stacker />`, roughly: `for cell in self.computed_cells { /* frame & slot */ }`

#### Problem:

We don't yet support nested symbol invocations like `elem.some_property`.  Code-genning RIL for this
requires knowing the type of the data at hand, both to unwrap intermediate `PropertiesCoproduct` symbols
(consider `foo.bar.baz`) and to handle the resulting value (e.g. `Numeric`-wrapping)

#### Plan:
```
[x] Support chaining in generated RIL, e.g. `foo` and `foo_DOT_bar`:
    [x] In the Pratt parser, adjust how we encode `foo.bar` (string-sub `foo_DOT_bar`) 
    [x] Maybe: sort invocations alphabetically, thus guaranteeing `foo` will already exist for `foo_DOT_bar`, 
        so `foo` can be used inside the invocation RIL for `foo_DOT_bar`
        [-] Alternate: a symbol trie, to handle `foo` and `foo_bar`, etc.
[ ] Add necessary `parse_to_manifest` or `parse_to_manifest` generation logic to `pax_type`
    [x] Add property reflection logic in `pax_type` macro 
    [x] Add hooks into calling types' `parse_to_manifest` logic during parser binary phase
    [x] Populate type definitions into manifest (alongside component definitions), punch through to compiler & expressions
        [x] What if types are EXACTLY empty-template components?  fits with `#[derive(Pax)]` intuition.  Decide which direction is conceptually cleaner.
    [x] Figure out `Default` with `pax-std` types
        [x] Implement macro API++ (derive plus attribute flags)
        [x] `#[custom(...)]` escape hatches
    [ ] Update iterable_type population logic;
        codegen calls to `populate_type_definition` for each nested Vec<Vec<T...
        possibly: hook into Reflectable / Reflectable
[x] connect `ComponentDefinition` with `TypeDefinition`
    [x] refactor access / mutation of ComponentDefinition.property_definitions
    [x] assemble `type_table` with a TypeDefinition for each visited entity
        [x] pax_type
        [x] pax_primitive
        [x] pax_component 
    [x] conslidate pax_primitive API with pax derive API
    [x] refactor type_id generation logic, clean, consistent, & DRY
        [-] look at feasibility of making a dynamic method like parse_to_manifest, part of Reflectable or Reflectable
        [x] or consolidate with source_id?
[ ] Update `resolve_symbol` logic to handle nested symbols like `foo.bar.baz` and `elem.some.deeply.nested.thing`.
    [x] Split by `.`, recurse PropertyDefinition => TypeDefinition.property_definitions => PropertyDefinition => ...
    [ ] Add RIL generation logic to handle trailing `.foo.bar` — 
        What does this look like?  If `foo` is a Property<T>, then we opaquely call
        `.get()`.  Do we also need to unwrap the propertiescoproduct? 
        - yes: .get() each subsequent property.  gather terminal symbol's TypeDefinition.
        - do not need to unwrap properties coproduct after the root -- once unwrapped, it "owns"
          the subsequent chained symbols, in a strongly typed way. 
[x] Add `property_definitions` to `TypeDefinition`, allowing recursion through
    nested `TypeDefinition`s, 
    [x] Refactor: `PropertyDefinition` and `TypeDefinition`; clean-up and consolidate
    [x] Populate `property_definitions` / types into `type_table` — this may need to happen after the initial full recursion
        in the parser binary, right before we currently terminate and write to stdout — 
        recurse the entire tree one more time, populating `known_addressable_properties` with the benefit
        of the whole manifest in hand.  
```



## Notes from creating a website

### Jul 10 2023

* Should this start as an example in pax-example?  (easiest to get running, can peel out later)
    * Could also start in www.pax.dev, the submodule, as a pure userland example.  Might require setting up the CLI / generator
    * Decided to start as a component inside pax-example, with plan to split out into a separate project
* It would be nice to reorganize the pax_std entities — maybe most core items in the top level, breaking out into dirs for specific
* Stumbled a bit over the linearGradient API, inherits same challenges & tech debt as Color API, addressable with general color API refactor
* Writing nested literals, e.g. the Vec<Panel> in pax-example/lib.rs, in Rust is clumsy.  Maybe we could create a `paxel!()` macro, which handles e.g. Numeric and % => Size::Percent?
* Found myself wanting Groups to be sized for layout (ended up using Frames instead, despite knowledge of clipping penalty / overhead).  Revisit the constraint / decision for Groups to be None-sized


### Jul 11
Notes from Warfa:
1) The image issue was that path under-specified

Ok

2) I had to remove the option from style_link cause couldn't wrap nested struct in an option easily

Probably the same issue as "cannot yet use enum literals in PAXEL."  (so cannot use Some(...))

3) when I added stacker to lib, ran into the same import issue with built-ins for Numeric/Size and the re-exports of them by stacker. Solved this by removing Numeric & Size from the built-ins. But a real solution would be to add proper dedupe logic. Wasn't immediately obvious if we should just use type name to dedupe.

Probably due to different apparent import paths for the aliased pax_engine::... import vs. the pax_runtime::api::... import, both pointing to the same struct
Using the `import_path` discovered dynamically in our hard-coded list, instead of using `pax_runtime::api::Numeric` in the hard-coded list, for example, is probably a suitable approach

4) Hit a bug when using the stacker horizontally with text. Realized I had somehow lost absolute positioning on the native layer. Fixed it and finished navbar

Ok

5) Converted for loop to a stacker for the panels.

Ok

6) I hit a bug where we constantly sending text updates patches. Not sure what the issue is yet but you can see it in console.log

Two separate issues:
 - 1. in the absence of a proper DAG for dirty-watching, Repeat destroys and creates its child elements each frame (expected behavior, if unideal)
 - 2. apparently there's a bug hampering proper deletion / creation in some cases.

First approach, ultimately shelved: set up a proper equality check in Repeat before churning elements, as the existing Rc::ptr_eq was not working.  This required deriving `Eq` and `PartialEq` for every pax type, which was a rabbit hole.  Discarded this approach fairly quickly, hard-coding a "dirty = true" on each tick in Repeat until we have the dirty-DAG 

Observations: id_chains seem malformed, e.g. several `[0]`s and several cases where multiple levels of chaining are expected but not presented (e.g. just `[11]`, not `[1, 11]` or similar.)
    Seems plausible that this is some sort of off-by-one error when constructing the ID chain, e.g. missing a level of recursion, or otherwise faulty logic somewhere in the very old `node_registry` logic. 
Approach: get bug into a state reproducible on macos, so that a debugger can be attached
Hypothesis: this is a different manifestation of the same bug already observed on macOS:
    "0.5.0 bug: repeated native elements aren't destroyed"
Thus, will chase down "repeated native elements aren't destroyed" with a debugger and see if that fixes (or if I can otherwise thus gain insights into) the deletion bug observed on web

Observations:
 - NodeRegistry#register, #deregister, and #instance_map might all be dead code, as nothing ever reads them.
 - If the above is true, it seems the mechanism used by Repeat to unmount elements is a no-op.  
   This handily explains some (all?) of the broken behavior we are seeing: Repeat is incorrectly / partially destroying elements.
    Either we need to update the 'hydrated node expansion' logic in the engine core to be aware of the
    adjustments made by Repeat, or we need to update Repeat's (and Conditional's?) deletion logic to be more mindful
   - Note: Conditional doesn't use this mechanism at all; it uses a "double buffer" approach, with one empty children buffer and one many-children buffer.
 - Further investigation: node_registry#instance_map seems to be used as an Rc safety-net, 
    ensuring the count of the Rc will always be at least 1.  Removing from node_registry
    allows the Rc to be cleaned up, "garbage collected," so its use is valid in this context.

Perhaps: we need to _mark for removal_ at the _end of the frame_ (or even the beginning of next frame), instead
         of deleting on the spot. This may be necessary to allow full clean-up throughout the tick lifecycle!


Findings in progress:
 - Repeat + element deletion simply isn't implemented;
   - Repeat's element deletion system is similar to Conditional's, with the key
        difference that conditional keeps track of a static subtree of rendernodes for each
        branch true/false, simply routing rendering into the correct subtree based on the conditional expression
        Repeat, on the other hand, destroys and createes new elements to map to underlying data.
   - Specifically, the vestigial NodeRegistry logic surrounding mounting was not updated to be 
        aware of id_chains. 
 

### Jul 12 2023


Notes from Warfa:
1) Removed the print lines and made the text delete a no-op for undefined id chains. This rendered the repeat bug pretty much invisible for the site. Probably worth pushing that work until after we finish the site unless it's something that is close.
    Agree it's not on critical path for initial site.  Almost there, time-boxing to 90 minutes today. [Update: took longer than 90 minutes but took a step forward, and got text on Web to a stable, non-churning place.]
2) One behavior that wasn't super intuitive was that stacker didn't respect the width and height it was set with. I initially didn't plan on wrapping each stacker with another frame until I noticed this. lmk if this is as expected
   Very likely, you are expecting `<Group width=something />` to do something.  This is a reasonable expectation, but 
   in fact Group does not support sizing (will silently refuse to accept sizes.)  The reason for this:  Group sits at the intersection of 
    a design tool concept (Group, which has no size of its own; it simply has the super-bounding-box of its contained elements) and a sort of "Group as `div`" as 
   we would use in HTML.  
    The "correct" way to have sized containers for now is to use a Frame, which allows fixed sizing.  We might introduce a "clip : bool" property on Frame
    if we double-down on Frame being the blessed way to have a sized Group, so that we can use them freely without incurring clipping perf penalties.  
    We can also revisit the question of whether a Group should support sizing, but this would require some new thinking backwards from the design tool and possibly some refactoring around how we handle "None-sizing".
3) The sizes property on stacker seems to break when I set it. I couldn't figure out if that was because I was creating the vec in the wrong way or some broken functionality. That is a blocker to make the text look decent on the website so worth looking into.
   Very likely the same as above — if you are expecting a Stacker inside a Group to respect the size you set on that Group,
    your expectations will be broken.  Try changing the Group to a Frame and see if things work as you expect.
4) Writing the markdown list was pretty annoying using a string in the ide. It defaults to adding tab formatting. went on a bit of a wild goose chase for this until I realized it was these tabs that were creating the unintended markdown side effects.
One solution could be to support Rust-style `r#####"`... for string literals, but this is a bit ugly.
    
Another option is to lean into XML and to define the Text API a bit further.  An idea:

```jsx
<Text>
    <Markdown>
        //We special-handle tabs here, removing the same number of spaces from each line
        Hello — I don't need to use double-quotes around this, because `Markdown` can handle the raw character contents
        
        This feels a lot like writing a `<div>Hello ...</div>` in HTML
    </Markdown>
</Text>


<Text>
    Just plain' ol unicode text here; we can give it the same tab removal treatment
    ... and maybe even support literal new-lines by default? i.e. no need to `\n`
    
    We could introduce another wrapper like `Markdown`, maybe `Literal`, which requires e.g.
    explicit newline characters, more like we would expect a string literal to behave in a programming language
</Text>
```

Finally, with a light but slightly hacky touch, we could special-case multi-line strings as they exist today `<Text text="multiline string beginning here...">`, to remove matched leading space from all lines


#### 90-minute timebox into "proper repeat unmounting"

Each time Repeat creates a new ComponentInstance, it mints it a new instance_id.
Immediately at this time, it unmounts the previous instance & its subtree.  All of this is OK.

The bug!!  When we `unmount_recursive`, we do it all with the same `rtc`, which doesn't respect the 
per-node context required to generate id_chain, which is why we're passing mal-formed id_chains.

Solution:  mark_for_unmount as a method on NodeRegistry, handle that unmounting during render tree traversal, just like `mounting`.

Note that unmounting happens at the `Instance Node` level, passing a flag to all descendents that they should unmount

Update at end of 90 minutes: nearly there!  only problem now is the `___Create` and `___Delete` messages are sent on the same frame.
As a result, text never renders; it churns too fast on both macos and web.
1. proper dirty-watching DAG would fix this, or 2. some sort of hack to ensure created elements sustain for at least one frame.
OR 3. a hacked dirty-watching mechanism (which is the approach I ended up taking)

Note: also pursued #2 for Web, with a requestAnimationFrame hack for removing text nodes.  It's worth either:
 - pulling the same rAF logic into other native node cleanup, or
 - removing the rAF (and hacked macOS rAF) logic once we have dirty-DAG

Further note: hacked the "cardinality check" dirty checking into place for Repeat — now Repeat elements will 
only update if the length of its source vector changes.  
Trade-off:  `self.some_vec.set(new_vec_of_same_length_as_old)` will not update as expected!  This is an ugly bug that can go away with proper dirty-checking (dirty DAG).

#### Sign-off notes:

 - Responsiveness: did some tire-kicking; seems to port pretty well out of the box to in-Chrome responsive screen simulators.  Probably need to 
    figure out how to make the nav bar dynamically sized, though, pending a sanity check on a real phone.
 - Weird breaking on Web, opened in Safari on iPhone via iOS simulator.  Seems to be partially crashing, semi-consistently throwing the error that I've been seeing spuriously via the Webpack error overlay screen.
   - See https://docs.google.com/document/d/10JpV5qivbT8o2y2oPL2Z403WPKvgeAULhFFjE19YJeM/edit
   - Fortunately, the page seems to work as expected on macOS Safari
 - Content: needs another pass, already WIP over here.  Broadly, I'm feeling this messaging needs to lean
    a bit more "sell the dream" than "sell the utility" at this point.  This probably comes to bear as an alternative to, or a wordier addendum to, the "table stakes" section.


### Jul 13 2023

 
Viewport culling should majorly improve perf



### Aug 21 2023


TODO:
 [x] copy & expand all necessary deps, as code, a la cargo itself, into .pax (not just chassis & cartridge)
 [x] clean up the special cases around current chassis & cartridge codegen, incl. `../../..` patching & dir names / paths
 [x] adjust the final build logic — point to _in vivo_ chassis dirs instead of the special chassis folder; rely on overwritten codegen of cartridge & properties-coproduct instead of patches
     [x] web
     [x] macos
 [x] fix build times
      [-] Refactor:
        1. perform current code-gen & patching process into a tmp-pkg dir
        2. maintain a separate pkg dir, into which we move the "final state" `pax-*` directories, the ones we refer to from userland and the ones we build from inside the pax compiler
        3. after generating a snapshot of `tmp-pkg`, bytewise-check all existing files against the `pkg` dir, *only replacing the ones that are actually different* (this should solve build time issues)
        NOTE: see Aug 28 entry for resolution
 [x] Assess viability of pointing userland projects to .pax/pax-engine (for example)
 [-] verify that include_dir!-based builds work, in addition to libdev builds
     [-] abstract the `include_dir` vs. fs-copied folders, probably at the string-contents level (`read_possibly_virtual_file(str_path) -> String`)

//TODO: observation: can reproduce a minimal "cache-cleared slow build" by simply:
//      1. go to `pax-example` and build `cargo run --features=parser --bin=parser`.  Observe slow build
//      2. Run again — observe fast build
//      3. Change `.pax/pax-properties-coproduct/lib.rs` trivially, e.g. by adding a newline
//      4. Run again, — observe SLOW build.
//     Apparently a single change in a single lib file is enough to trigger a substantial rebuild.
//       Perhaps: because many things depend on properties-coproduct, a single change there is enough to require all of them to change
//     observation: when running cargo build @ command-line (as opposed to via Pax CLI), you can see that building `pax-compiler` takes a substantial portion of the time.  This checks out esp. re: the embedding of all the `include_dir`s into pax-compiler.
//Possibility: when code-genning & patching — do so into _yet another_ directory —e.g. `tmp` — so that the resulting files can be bytewise-checked against canonical files, before overwriting.  This should stabilize FS writes and cargo's tendency to clear caches when files change.
// 1. perform current code-gen & patching process into a tmp-pkg dir
// 2. maintain a separate pkg dir, into which we move the "final state" `pax-*` directories, the ones we refer to from userland and the ones we build from inside the pax compiler
// 3. after generating a snapshot of `tmp-pkg`, bytewise-check all existing files against the `pkg` dir, *only replacing the ones that are actually different* (this should solve build time issues)
// 4. along the way, abstract the `include_dir` vs. fs-copied folders, probably at the string-contents level (`read_possibly_virtual_file(str_path) -> String`)

//1. fetch pax version numbers from host codebase
//2. copy all deps — either from crates.io or from libdev ../
//3. codegen properties-coproduct & cartridge (incl. relative dep to host codebase in latter)
//4. include patch directive in appropriate chassis; build dylib, run dev-harness



### Aug 28 2023

Rethinking PropertiesCoproduct, patching, dependency management

Much build complexity arises from the code-genned PropertiesCoproduct.
    - If we didn't need to patch this — could we build userland projects entirely from crates.io? (it looks like the answer is "we must build from source")
    - The other code-genned dependency is `pax-cartridge` — does anything still rely on this besides the chassis?  (The answer is no — only the chassis)
    - What if we refactor `pax-properties-coproduct` to rely downstream on a dylib?  Thus, we can leave all dependencies on pax-properties-coproduct alone — no need to codegen & mangle dependencies — and can swap that dylib not only at static build time, but for live reloading as well

In the above world, we would only need to:
    1. codegen cartridge (clone code, patch lib.rs)
    2. codegen the internal properties-coproduct, build as dylib
    3. build the chassis, specifying path to cartridge (../), and hopefully just including the dylib in a special directory

This gets a bit hairy — 1. we still need to codegen (or clone) everything we're going to build, and 2. we still face the "conflicting dependency" problem between userland crate and codegenned / built crates.

One possible solution to the above: build the userland crate / project as a dynamic library!  Then, instead of a relative path to load it, the cartridge loads that dylib.
    - Would this actually resolve the conflicting versions problem?  Even if it makes the compiler happy, we are probably bundling different versions of deps......  This might be OK for e.g. pax-compiler, but gets particularly dicey around runtime deps

Stepping back to the latest pencilled approach:
 - clone EVERYTHING into .pax/pkg (plus the "double-buffering" optimization for cargo build times? Maybe not so important if we speed up compiler build time, by dropping `include_dir` )
 - depend on userland_crate via pax-cartridge
We have previously run into the issue where userland_crate relies on, say, pax-compiler#0.4.0, but cargo sees that as a conflict with the local FS version
We might be able to mitigate the above issue by:
   1. detecting all `pax-*` dependencies in the userland Cargo.toml — track which version(s) they specify (note that all versions must be lock-stepped; we can throw an error if any are mismatched)
        1a. consider what it would take to support a user relying on pax versions in downstream crates or in a cargo workspace
            - Either crawl all dependencies, resolving versions
            - Or look for the first dep, match it
            - Or, special-case where the pax lib version is specified (e.g. .paxrc)
            - Or, introspect the Cargo.lock and find a canonical list of resolved versions (?!)
        2a. Let's go with reading the Cargo.lock — we can build the project if Cargo.lock is missing, and we can *look for each whitelisted dependency in the Cargo.lock, checking versions and ensuring they match.*
   2. instead of bundling a particular version of the lib crates via `include_dir`, we can reach out to crates.io and clone/extract the published tarball.
   3. We can come back later for "offline builds," and we can handle libdev specially to "clone" from `../` instead of crates.io
      3a. For crates.io builds, we can assume idempotency for everything except codegenned crates — no need to clone if directory exists (but be sure to surface some sort of `clean` or `init` to clean corruptions)
      3b. For libdev builds, we can full-clone everything every time. Note that removing the dependency on `include_dir` should dramatically improve libdev build times
   4. Use `patch`, at `chassis-macos`, (root crate) to override each `pax-` dep used by userland_crate, to resolve to the local FS (cloned / extracted) version to the same `.pax/pkg` version used elsewhere in the build

So boiling down to an algorithm:

1. Read userland Cargo.lock, discover stated version of any `.pax` project with id in our whitelist
2. If host_crate_info.is_lib_dev mode
   3. Copy all directories in whitelist to `.pax/pkg`.  Each src directory can be constructed by roughly `pax_dir.join("../").join("../").join(whitelist_dir)`
4. Else
   5. Foreach directory in whitelist, detect if exists in `.pax/pkg`.  If so, do nothing; assume it is already cloned.  If the directory doesn't exist, clone tarball for version `pax_version` from crates.io to the appropriate dir, `.pax/pkg/{whitelist-pkg-id}`.  Print to stderr and panic if unable to find or clone any of these packages from crates.io.
6. Include patch directive in the appropriate `chassis`'s Cargo.toml (either `.pax/pkg/pax-chassis-web` or `.pax/pkg/pax-chassis-macos`, depending on `TARGET`) —
   7. Within this directive, patch all discovered dependencies from (1) to override concrete semver => local `.pax/pkg/{pkg-id}`
8. Within our `.pax/pkg/` chassis directory with the patched `Cargo.toml`, run `perform_build()` 

Update: the above seemed to work satisfactorily.

#### Aside: `pax` name collision with system binary `pax`

Looks like `cargo install` doesn't give us post-install hooks, e.g. to modify PATH or perform other related tasks.
We want this ability at least to alias `pax-cli` to `pax`

Note that `pax` is a system binary available on macOS at `/bin/pax` and likely in many *nix installs —
This appears to have been used for the macOS installer packages until about 15 years ago, deprecated with the release of Mac OSX Leopard (10.5)
See: https://en.wikipedia.org/wiki/Xar_(archiver), https://en.wikipedia.org/wiki/Mac_OS_X_Leopard, https://en.wikipedia.org/wiki/List_of_macOS_built-in_apps#Installer

It's very possible that `pax` is still in use on some machines, though for anyone who wants to use (our) pax, it feels
acceptable to usurp `pax` in PATH, especially with documentation & and path back out.

Cautiously, let's proceed with aliasing / overriding `pax` in the system PATH, with the recommendation that
users of the legacy `pax` fully qualify its usage `/bin/pax`, or otherwise alias / rename our pax binary to something like `pax-cli`

#### Next steps: "create-react-app"-style CLI

1. install pax-cli
   2. for development, can do this in a sibling-folder to `pax` monorepo, and refer to `../pax/pax-cli` (to cut out need to publish to crates.io)
3. run `pax create some-project` 
   4. clone template project
      5. Maintain template project inside monorepo — `new-project-template` ?
      6. either fs-copy this template project or pull tarball from crates.io (don't include_dir(../), as this breaks crates.io builds)

[x] Unpack template project
    [x] improve args & ergonomics - namely path & name
        Consider CRA's NUX — note that "path" and "name" are the same thing
        > npx create-react-app my-app
        > cd my-app
        > npm start
        Consider also cargo's NUX:
        > cargo new ../nested/path
        >    Created binary (application) `../nested/path` package
        Seems like we should just accept a single unnamed arg and treat it as both path and name.
        Handle cases of already-existing paths with an error:  `error: destination `/Users/zack/code/scrap/../nested/path` already exists | Use `cargo init` to initialize the directory`
        Could follow on with a pax init that patches the existing project, but this is a bit hairier and doesn't feel MVP
    [x] inject current pax lib versions into Cargo.toml
        - Decide for libdev mode: do we want to refer to monorepo packages or crates.io packages?
        (easiest way to start is crates.io packages)
        - Consider ability to specify explicit (past) versions?
    [x] ensure that crates.io stand-alone build of pax-cli / pax-compiler will have access to these files (no ../ or monorepo paths pointing out of crate, for example)
    [x] build out libdev vs. prod mode
        [x] libdev copies a live fs copy of template, to enable iterative refinement
        [x] prod mode uses include_dir!
    [x] iterate & refine the starter template — probably base it on our website
        [x] create a helper script in the monorepo (scripts/create-and-run-template.sh)
[x] Update checking
    [x] Async, during any CLI command, determine whether a new version of pax-cli is available
        [x] Check remotely via a server we control, for ability to guarantee stability (keep registry of published versions remotely, or check upstream to crates.io, or both)
            [x] build update server
                [x] logic
                [x] dev env
                [x] deployment
        [x] Make CLI main async, "race" nominal actions against update check; give up on update check if a nominal update finishes before update check finishes.  Otherwise, print message after nominal action (incl ctrl-c) if a newer version is available.
        [x] Wrap `Command`s with an async-friendly, interruptible wrapper that handles e.g. ctrl-c
        [x] Seek to make shell scripts compatible with this async mechanism, too
    [x] libdev checks a locally running server (http://localhost:9000) vs. prod checks a live server (https://update.pax.dev)
        - Made it an explicit env option, PAX_UPDATE_SERVER
    [x] reasonably formatted banner
    [x] offline robustness

Split out: Merge the above, tackle the following separately

[ ] Packaging & installation flow
    [ ] create an installer script — can't use `cargo install` due to our need for scripting
        [ ] macOS + *nix
        [ ] Windows
[ ] Test e2e & document pain-points / steps&deps:
    [ ] macos
    [ ] linux
    [ ] windows
[ ] Update monorepo README with instructions to get started with CPA, as well as libdev instructions
[ ] Better starting project - copy of website, router example, layout example, ...?




### On porting to 3D

By porting our 2D, CPU-bottlenecked rendering to 3D, we should relieve a significant compute burden on client devices.
This should be particularly helpful for e.g. iOS Safari, where we seem to be hitting perf limitations with canvas, but
should also raise the ceiling across the board on how much (e.g. how many elements) we can render.

Broadly, to draw 2D vector shapes in 3D we will need to:
 
[ ] Get "hello world" 3d canvas rendering on screen, in browser, via `wgpu`
    [ ] Achieve "hello world", perhaps in a stand-alone dev harness / codebase
[ ] tessellate _fills_ and _strokes_, using a library like `lyon`
    [ ] render triangles to wgpu
    [ ] port our existing logic (can probably do this by introducing a new struct/trait that is API compatible with the current subset of the `rc` that we use, e.g. `save/restore` (clipping stack), `clip`, `bez_path`
[ ] handle clipping, likely with the GPU stencil buffer and our own clipping stack 
    [ ]  manage state of clip stack (push / pop, mapped to our current save / restore situation)
[ ] handle solid & gradient fills, likely with custom shaders
    [ ] Pipeline for running / including shaders
    [ ] Solid color API, incl. alpha
        [ ] Refactor our use of colors throughout
    [ ] Linear
        [ ] Refactor our use of gradients throughout
    [ ] Radial 
        [ ] Refactor our use of gradients throughout
[ ] handle antialiasing, either at the renderer config level or as a screenspace shader
    [ ] Try config level
    [ ] Check out screenspace shader, but likely not a good fit for precision vector rendering
[ ] test rigorously across devices, especially mobile, and polyfill (or fall back to 2D canvas) as necessary


### On mounting components rather than entire apps

Illuminated through the process of shipping our own site with pax

For a given chassis, we can implement component mounts in addition to our existing full-app mounts.

This will make it possible, for example, to mount a Pax component in a React app, or for us to create our own HTML
template (e.g. including Google Analytics snippet) and then to mount the Pax app inside that template.

What will it take to do this?

[x] Chassis
    [x] Introduce a harness-component and rename pax-dev-harness-web to `harness` (or `interface`?)
[x] Compiler
    [x] Build every web project into the same template — include both an index.js (+.d.ts) and an index.html.
        - Statically serving serves the app full-page, via index.html
        - Importing the index.js file will offer an API into manipulating the chassis
            This enables, for example, our GA use-case, or integration via React `ref`s, etc.
[x] Attaching to codebase, design - figure out a smooth workflow for importing a Pax component
    [x] API for init, config
        [x] Consume this in the static index.html, to init on `.mount`
        [x] Expose this (plus .d.ts) for consumers
    [x] Figure out module / package setup
        [x] Node module? Generated JS / TS bindings in some web codebase? Web component?
            - Check out StencilJS as a tool for packaging / compiling web components
            - Maybe conventionally add a `pax_modules` folder to host codebase, version-controlled, where webcomponent bundles are built?  can refer to pax_modules with local fs paths
                - The alternative to this would be publishing pax components to npm
                - Could also symlink via npm, dev-only, but this sets up a hazard for broken teammate editing
            - Probably don't need to use WebComponents at all — just expose a plain ol' JS object with an init command, which accepts a selector or DOM node to mount to
            - Ended up going with UMD as packaged via Webpack
        [-] Regarding React/Vue/etc. components, don't want to use `stenciljs` because it has particular Node major-version reqs, which is stochastic churn for folks trying out Pax
            That said, perhaps we can use StencilJS to build our template, statically swapping in assets instead of requiring stencil to be installed by end-users
            Also, from a look at the docs, stencil's component API should allow us to patch in Pax properties (Props) from host frameworks (e.g. React) 
            Need to vet: can patch a generated stencil dist with new assets and not require stencil end-developer install
            - Decided to come back for this later.
    [x] Package CSS (webpack + TS import)


### On polishing the New User Experience (NUX)

Let's determine what needs to be done via three channels:

1. predictive (based on intuition and experience, what should be improved?)
2. empirical, personal experience (simulating a new user's experience, e.g. on a fresh VM, take notes of what should be improved)
3. empirical, user observation & feedback (observing a new user's experience or based on their account, determine what should be improved)

[ ] Predictive, idealized
    [ ] Installation
        [ ] Install script + one-liner
        [ ] Alias pax-cli as `pax` and ensure presence in PATH
        [ ] Figure out cross-platform pre-built binaries or cargo/rustc-based build flow
    [ ] Starter project, something interesting & instructive (layout tutorial?)
    [ ] Docs — clean up Github readme, website content, and docs.pax.dev (take a pass on the latter to make up-to-date; last publish was one year ago)
    [ ] Using a project
        [ ] Full-app use-case: build & publish static site (for web target, natch)
        [ ] Component use-case: build & inject Pax onto a div (for web target, natch)
    [ ] Authoring experience
        [ ] Helpful error messages
        [ ] Fast build times & feedback loops
        [ ] Auto-complete, go-to-definition, inline error highlighting, syntax highlight (language server + plugin)
        [ ] Hot reloading
        [ ] Robust standard library (e.g. form controls)
[ ] Empirical, personal
    [ ] Want auto-complete or obvious documentation re: the function signatures for event handlers
    [ ] Want a built-in $tick for use in PAXEL
[ ] Empirical, external
    [ ] TODO



### On common properties like `x`, `y`, `width`, and `height`

A.k.a. "Transform API ergo, syntax sugar"

Currently, we special-case a handful of properties in the compiler like width and height. To make this more robust and to offer a declarative API for the "80%-case" of transform declarations (i.e. vs. the 20% case where custom sequencing is desired,)
we can:

[x] introduce a CommonProperties struct, which includes: transform and each individual sugared transform operation (x, y, width, height, scale_x, scale_y, rotate, skew_x, skew_y, anchor_x, anchor_y — or possibly nested versions of these e.g. scale: {x:...y:...})
    [x] Refactor size: get_size to return Option<(Size, Size)>
    [x] Introduce Size#evaluate to DRY evaluation-given-bounds
    [x] Refactor get_clipping_size to match type of get_size
    [x] Default-impl get_size to call self.get_common_properties and to pull out `width` and `height` (also decide based on usage whether to pass `bounds` into `get_size(bounds)`
    [x] Do the same as the above for `get_transform` — note that get_transform is currently called exactly once, so we have leeway to change the interface substantially, easily
    [x] Refactor every impl of RenderNode (primitives + component primitive) to keep some state representing common_properties, and to return an Rc::clone of it when called
        [x] Remove the locally stored `transform` and `size` along the way
    [x] introduce a trait method on dyn InstanceNode to get_common_properties.  likely refactor get_size and get_transform across the board, perhaps to fetch these values, or perhaps retiring them in favor of get_common_properties().size and ....transform [
    [x] update cartridge codegen to accommodate InstantiationArgs#common_properties
[x] in the workhorse rendering loop, combine declarative transforms into a matrix (we choose the sequence to match ergo expectations) and multiply that matrix with a transform property, if specified (a user should be able to specify both a transform matrix and individual properties; again we choose the best order in which to combine these) 
    [x] along the way, retire Transform::align and make Transform::translate accept Size values instead of floats (make it bounds-aware)
    [x] where we special-case certain fields in the compiler when parsing element K/V declarations, now make sure that list of special-case properties meshes with the properties/types of `CommonProperties`.  Perhaps impl CommonProperties to return an ad-hoc, manually maintained "reflection" manifest of its properties (.reflect_on_properties)
[x] update website example to use updated APIs
[x] finish-line
    [x] finish Rotation refactor, ensure we can use an expression e2e (may require special-handling in pratt parser like with Numeric)
        [x] make sure we handle the unit `deg` (and {no unit, `rad`} => `rad`) in codegen, both for property literals and in PAXEL
        [x] for kicks, handle `%`, too
    [-] stretch: support {50% + 35px}, updating Size to support combined values
    [-] e2e test that we can bind an expression to each of our sugared properties
        (Note: didn't manually test all properties, just tested `rotate`)
    [x] QA
        [x] Can't yet bind an expression to `rotate` (compiles but silently fails)
            [x] We need to `compute_properties` — perhaps can build a helper for this inside `impl CommonProperties`
        [-] Can't combine % and px, namely because of `into()` coercion.  Deserves more digging to find a potential mitigation
            Decided to treat as a separate feature
        [-] Can't apply a negative sign to a literal, for other (or the same) `into()` vs. type annotation reasons
            - Are there cases where we know the desired, or some higher level type, and might we be able to call an explicit typecast
              somewhere in codegen?
            Decided to treat as a separate feature
    [x] final cleaning & review



### On a Pax playground, fork of Rust playground

Tasks

[x] Development environment
    [x] Vagrant + idempotent backend provisioning script
    [x] Local dev env + setup scripting
[x] Web chassis
    [x] Support runtime wasm loading (instead of static, bundled wasm as per current setup.)
        This will enable us to introduce path params that override the runtime-loaded wasm file, while falling back to some
        default when not in the sandbox setting.
        [x] In the `mount` method, expose an optional fully qualified path for the wasm file.  We can reasonably assume one file per page session / reload for now,
            so it's OK to do this only at init time in the `mount` method.
        [x] Handle null case for default loading of some fixed-name file, like `web-cartridge.wasm`.  This should keep e.g. our website working and continue to deliver the "index.html that just works" feature.
        [x] Handle bundling / packaging — likely we don't need webpack at all for this; we just need to build the typescript project, and that can be done in libdev
            [x] Refactor webpack / node deps; remove from userland if feasible; figure out libdev build process (bundle built TS => JS; ensure this happens when running pax-example and when publishing to crates.io)
            [x] Handle need to bundle CSS like we currently do with Webpack
                Note: not bundling, but assuming a shared-name file and loading it async at init time
            [x] Ensure that assets are well handled (e.g. images)
        [x] Stepping back: Loading the _wasm_ separately, as opposed to loading the _glue code js_ separately, feels like the wrong tack.  We need to load some sort of js to manage the wasm glue, and that js may evolve over time, which means we'd be introducing version brittleness if we load _strictly_ the wasm on e.g. the playground.
            Thus, we should load both the wasm and the accompanying glue code dynamically, based on playground path/url params.
            In good news, this is still applicable to our "don't foist a bundler on the end-developer" strategy, as wasm-pack outputs both artifacts.  Essentially, we (1) bundle our TS chassis project using esbuild/tsc, then (2) dynamically load the wasm-pack module via glue code based on a passed param (default to a local file; allow overriding for e.g. sandbox loading.)
            In light of this, an assessment of build tool options:
            [-] ESBuild — requires either a node dependency to `npm i` binaries, or managing our own pre-built binaries, or requiring a Go dependency to use the Go API, or managing pre-built Go-based dependencies (e.g. with a Rust / FFI wrapper.)
                    Alt: pre-built binaries are available!  Main drawbacks are (1) need for unix or WSL, and (2) no plugin support.  See https://esbuild.github.io/getting-started/#other-ways-to-install
                Strengths — can inline wasm into a single JS file, simplifying consumption (especially downstream) — note that this may preclude "pre-bundling" as an alternative to foisting a build tool dependency onto the end-user
                Weaknesses — doesn't inline CSS out-of-the-box, complicating consumption downstream (could fix with a hacked script or a plugin)
                Note: we can use ESBuild at libdev time for typescript bundling; see "no userland bundler"
            [-] Vercel Turbopack
                Appears somewhat coupled to next.js?  Also appears unfortunately webpack-like, in bad ways (e.g. the config)
                Built in Rust, which is promising for API consumption — but doesn't appear to have a documented API?
            [-] SWC - bundle feature appears to be alpha and unsupported (creator was bought by Vercel)
            [-] bun - threw an error bundling wasm last time we tried; maybe it will get there
            [x] no userland bundler — the idea is to build a bundle statically at libdev time, including that "built template" in the Pax compiler+CLI, and to splice out the .wasm (assuming an unchanging glue interface)
                for userland builds.  We would still upload both glue + wasm to CDN, so solve the version problem
[ ] UI
    [-] (A) Subtractive approach:
        [ ] Rip out unnecessary features
            [ ] Tools
            [ ] Run > ... menu
            [ ] Debug / Release menu
            [ ] Stable / beta / nightly menu
            [ ] Stable > ... menu
            [ ] Config
            [ ] (?) menu
            [ ] Streamline "Share" to be a single action
        [ ] Add Pax pane alongside Rust pane
            [ ] Pass along with Rust code as a separate param
            [ ] Rehydrate the editors with the correct content based on permalink
            [ ] UI treatment
        [ ] Configure CodeMirror (or monaco! or ace!)
            [x] Decided CodeMirror per replit's published assessment
            [ ] Rust mode
            [ ] Pax mode (try ChatGPT)
    [ ] (B) Additive approach (from scratch) 
        [ ] Simple React or Pax app (or Svelte app?) —  two panes for input, one pane for rendering output, some sort of UI for stdout and stderr
        [ ] Configure and embed CodeMirror (based on replit assessment)
        [ ] In the playground web app, read path params and pass into the pax-chassis-web `mount` method.
[x] Backend
    [x] Manage dependencies, probably same whitelist as RP
    [x] Decide additive or subtractive approach; support simple golden path "pax build --target=web"
    [x] Simple HTTP server + handlers
    [ ] Script copying of "warmed up" directory, executing `pax build --target=web`, uploading .wasm and text files to CDN
[ ] Deployment
    [ ] Configure ELB & ASG
        [ ] Declare with Terraform
        [ ] Script phase-in/phase-out for deployments
    [ ] Configure S3 + Cloudfront with permalink functionality
    [ ] Create "AMI-minting" machine
        [ ] Terraform declaration script
        [ ] idempotent provisioning script (Terraform + shell?) (decide whether to keep one instance kicking around, or whether to re-init from scratch each time)

Sketch addtl. functionality needed to support "edit this website in playground"

[-] Decision: punt on multiple file support for now; we can link to pre-curated published / shared sandbox sessions with single rs/pax file examples, e.g. a modified version of our website
    [-] Multiple files
        [-] Security:
            [-] Blacklist build.rs
            [-] Whitelist available crates
            [-] How to deal with proc macros?  Static analysis?
            [-] Sandboxed containers — allow the user to perform malicious actions, but firewall their container from other users (e.g. through gVisor + Kubernetes or similar)
    [-] UI
        [-] Multiple tabs, maybe file tree view
    [-] Publish (can start with manual)
        [-] Manifest for files
        [-] upload relevant files (lib.rs, website_desktop, website_mobile



### On native iOS builds

We are 90% of the way to supporting iOS, given the shared Swift + CoreGraphics layers with macOS.  Speccing out what's needed to achieve alpha iOS support:

[x] Split out shared logic into SPM packages
    [x] Configure macOS project to consume shared packages; get build working
[x] For proactive cleanliness, break out the cartridge (dylib) into a SPM package, so that consuming it is streamlined and simple across macOS / iOS
    [x] Wrap all the way from .dylib -> SwiftUI View; this includes creating a .Framework
    [x] Consume that exposed View in both macOS & iOS
[x] Refactor (or redo, as needed) macos project to consume new deps
[ ] Compiler work
    [x] Refactor compiler internals to adapt to new cartridge / framework / swift package structure: macOS
        [x] Configure pax-chassis-macos/interface/pax-app-macos to load the swift packages as relative dirs
        [x] Get e2e build working in-place with pax-chassis-macos (no codegen); embed a placeholder cartridge + resources (bouncing logo?)
        [x] After copying everything into .pax, patch (1) resources and (2) the dylib into pax-chassis-common/pax-swift-cartridge, then
        [x] Build the resulting, patched, codegenned macOS project
            [x] Handle architectures: aarch64 and x86_64; 
            [x] Bundle targets into xcframework
                [x] Create default carts for xcframework along the way
            [x] Multithread the dylib builds
            [x] Handle release vs. debug 
        [x] Handle updates to assets; bundle into pax-swift-cartridge Resources
    [X] Extend pax CLI + compiler to support --target=ios, firing up simulator if present on machine
        [x] Refactor macOS-specific deps, e.g. NS* and CVDisplayLink
        [x] set up xcframework manually and achieve hello world build of xcodeproject
        [x] add ios target
        [x] Handle build architectures: aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
        [x] automate copying & lipo of dylibs
        [x] manage xcodebuild shell command + firing up simulator
    [ ] Handle relative paths + inter-dylib deps
        [ ] Manually update xcframework structure
            [ ] use `install_name_tool` to change Rust-generated absolute paths into relative paths
            [ ] get iOS project building with manual changes
        [ ] 
[ ] Make better default cartridge for default bundling (e.g. bouncing Pax logo)
    [ ] Clean xcframework binaries from git history for leaner clones; replace with lighter-weight release builds if straight-forward
[ ] Fix undefined out-of-canvas pixels on iOS (either lock scrolling into bounds with a simple check, or handle canvas clearing ?)

From Apple developer forums:
```
If you want to build a framework that supports iOS and the iOS Simulator, construct an XCFramework with:

One element for iOS containing just the arm64 architecture

Another element for the iOS Simulator containing both arm64 and x86_64 architectures
```


Issue with hard-coded vestigial paths in dylib — our built dylibs include absolute paths pointing to ....../libpaxchassismacos.dylib


```
otool -L PaxCartridge
PaxCartridge (architecture x86_64):
	/Users/zack/code/pax/pax-example/.pax/pkg/pax-chassis-macos/target/x86_64-apple-ios/release/deps/libpaxchassismacos.dylib (compatibility version 0.0.0, current version 0.0.0)
	/System/Library/Frameworks/CoreText.framework/CoreText (compatibility version 1.0.0, current version 1.0.0)
	/System/Library/Frameworks/CoreGraphics.framework/CoreGraphics (compatibility version 64.0.0, current version 1774.0.1)
	/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation (compatibility version 150.0.0, current version 2048.1.101)
	/usr/lib/libiconv.2.dylib (compatibility version 7.0.0, current version 7.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1336.0.0)
	/usr/lib/libobjc.A.dylib (compatibility version 1.0.0, current version 228.0.0)
	/System/Library/Frameworks/Security.framework/Security (compatibility version 1.0.0, current version 61040.2.2)
	/System/Library/Frameworks/Foundation.framework/Foundation (compatibility version 300.0.0, current version 2048.1.101)
PaxCartridge (architecture arm64):
	/Users/zack/code/pax/pax-example/.pax/pkg/pax-chassis-macos/target/aarch64-apple-ios-sim/release/deps/libpaxchassismacos.dylib (compatibility version 0.0.0, current version 0.0.0)
	/System/Library/Frameworks/CoreText.framework/CoreText (compatibility version 1.0.0, current version 1.0.0)
	/System/Library/Frameworks/CoreGraphics.framework/CoreGraphics (compatibility version 64.0.0, current version 1774.0.1)
	/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation (compatibility version 150.0.0, current version 2048.1.101)
	/usr/lib/libiconv.2.dylib (compatibility version 7.0.0, current version 7.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1336.0.0)
	/usr/lib/libobjc.A.dylib (compatibility version 1.0.0, current version 228.0.0)
	/System/Library/Frameworks/Security.framework/Security (compatibility version 1.0.0, current version 61040.2.2)
	/System/Library/Frameworks/Foundation.framework/Foundation (compatibility version 300.0.0, current version 2048.1.101)
```

1. These pax-chassis-macos paths almost certainly come from the "pre-built" PaxCartridges that I made manually to get to iOS hello world.
   These should be replaced with correctly prebuilt packages, from pax-chassis-ios. They should also use @rpath instead of hard-coded dep paths.

2. current best-bet approach to handle the nested dylib deps: create a nested framework (for each arch target, also consider whether lipo is necessary; probably is)
    this allows embedding the dep .dylib alongside the entrypoint .dylib.  Then in the entrypoint .dylib, use `install_name_tool` or similar to 
    update the path to an @rpath/ relative path, using some trial and error to get to a place that xcode respects the provided path.  Something like:

```
MyLibrary.xcframework
├── ios-arm64
│   ├── MyLibrary.framework
│   ├── Dependency1.framework
│   └── Dependency2.framework
└── ...
```

Get this working entirely manually first, then automate in pax-compiler. (validate by `pax clean` and ensuring that builds still run.)




### On robust multi-level slots & scopes

[x] separate runtime scope stack from adoptees stack
    [x] introduce lifecycle methods surrounding property computation, for managing runtime property stack
[x] rename adoptees to slot-children
[x] rename runtime stack to RuntimePropertyScopeStack
[x] document relationship between runtime stack & compile-time (offset) stack
[x] rename `should_flatten` to `is_invisible_to_slot`
    [x] manage lifecycle between calling the above method, which requires recursive properties to have been computed,
    vs computing properties
    [x] note that this means adoptees aren't tracked in a stack at all.
    [x] instead, ensure that the "node containing the node for which the current render_node is a template member" (current active component) can retrieve the slot_children that were passed to it
[x] consider renaming `will_` to `pre_` and `did_` to `post_` (while a departure from React-like conventions, there's precedent in other worlds like .NET, and it's just a better naming scheme)
[x] handle forwarded children in Repeat
[x] handle triggering events in handle_registry given changes in runtime stack
[x] probably introduce `get_properties` on dyn InstanceNode, returning something like an Rc<RefCell<dyn Any>>
[-] Alternatively!  fire handlers at end of properties compute phase?
    Turns out the above is insufficient.  We also rely on property<>render-tangled stack frames
    when gathering `properties` (the on-demand `self`) for invoking event handlers, via `cartridge-render-node-literal.tera`
[-] handle deep cloning or deep property storage
    NOTE: deep cloning shouldn't be needed if we store computed properties on ExpandedNodes


Problem:

In the following template:

```
for i in 0..5 {
    <Stacker cells=10 direction=StackerDirection::Vertical x={(i*50)px} width=50px>
        <Rectangle fill={Fill::Solid(Color::rgb(i * 0.2, 0.2, 0.5))} />
    </Stacker>
}
```

when we are evaluating `i*50px`, we want our stack to have `for`'s `i` on the top frame.
Instead, because the stack frame is getting pushed by compute_properties for Stacker, 
there's already a stack frame for Stacker present when we're trying to compute our `i*50px`.

Possible solution: instead of coupling stack frame management to the compute_properties methods, 
we could explicitly manage stack pushing & popping at the same place we check for `is_component` in
the recursive workhorse.  Along the way, this probably means retiring our new `will_compute_properties` and `did_compute_properties` lifecycle methods,
as we're not yet aware of an explicit application for them.

There's a special case here for `for`.  Within the context of a single component's template, 
we want `for` to be able to push a stack frame, but we don't want to push a stack frame for something like a <Stacker>
inside that template.

So:

[`A` Component root] -> push a stack frame for `self.` for all components in this template
[for loop] -> push a stack frame for `i` or `j`, etc.
[Stacker or other userland component] -> don't do anything in the context of `A`.  We will handle `Stacker`'s evaluation later, with its own template + stack evaluation


So: yes push for a given component root (like we currently do in latest rev. of recursion-workhorse)
Yes push for a `for` loop
No, don't push for other component instances found during 

Possible tool: a parameter passed to ComponentInstance to signal whether it is managed by `for`,
alongside a `dyn InstanceNode` method that the engine can call to push / pop (at properties-compute-recursion time.)
This gives us the ability to special-case `for`-managed ComponentInstances, to explicitly push
to the properties stack, while ignoring components like Stacker



or storing `n` copies of properties per node?  we'd want to store a map of `id_chain => properties`)


we can get_id_chain from rtc.get_id_chain, so this is close at hand
would this mean changing the API for compute_properties and get_properties?
One other option is to "reinstantiate" a node — store the instantiationargs passed to any node and
re-retrieve them as a way of deep-cloning (this would need to be recursive, for child/slot nodes)
-
in service of deep-cloning, consider our APIs for getting nodes:
get_instance_children expects expanded children
ExpandedNode represents expanded nodes
Perhaps we should attach computed properties to ExpandedNodes?  That's already the right shape of container for these properties.


Refactoring node management:

1. compute properties; manage ExpandedNodes and store computed properties
2. render: recurse through ExpandedNodes
How do we recurse through ExpandedNodes?
When we call lifecycle methods e.g. `handle_render`, how do we specify which properties to pass?
Will it be straightforward to move away from the current way of instantiating ExpandedNodes?
   Yes, should be straightforward.  We simply create them in the rendering recursive workhorse; we could straightforwardly
   shunt this into properties computation.  Main challenge is figuring out the shape of pointers between instance nodes and ExpandedNodes 
Think through the relationship between ExpandedNodes, dyn InstanceNode, and property-related lifecycle methods

Perhaps we rotate the manifold currently between `instance node` and `repeat expanded node`
 — `instance node` vs. `properties expanded node` or `properties-render-node` 

We should also reconsider how we store and retrieve ExpandedNodes. Instead of
the current cache, are these nodes findable via `dyn InstanceNode` ?
Do we store them in a separate table and share Rcs?
Make sure to address recursability (through ExpandedNodes) along the way

Consider also how the registry handler + component event handlers route through
ExpandedNodes.



1. We have a strong divide between `instance nodes` and `expanded nodes`.  An instance node is an instantiated version of what is declared in a template.  An expanded node both accounts for repeat (e.g. it is id-ed by `id_chain`) _and_ includes a unique stamp of `properties`, computed in the context of repeat.
2. How do we get `ExpandedNode`s from an instance node?  Currently we have `get_instance_children` — should this be supplanted by `get_expanded_children`, and if so, how does a RenderNode
    become knowledgeable about its ExpandedNode children?

Unpacking:  during properties computation, we can create ExpandedNodes, because we both have knowledge of ancestral repeats for `id_chain`, and have
locally calculated properties that we can store on an ExpandedNode.  Perhaps there needs to be a method on `dyn InstanceNode` for setting ExpandedNode children? (`set_expanded_children` ?)
Another possibility is to mark relationships e.g. through cloned Rcs, allowing ID-based lookups without requiring any setting through `dyn InstanceNode` (with the heavy lift of punching through all trait implementors)
Even so, there needs to be some way for a node to specify its children dynamically.  Basically, most nodes just return their templated children (wrapped as `ExpandedNode`s?)
but Repeat does so as a function of its data source, and Conditional does so as a function of its evaluated boolean state

We could either:
(a) keep a list of ExpandedNodes representing the children of a given node
(b) keep a list of ExpandedNodes representing the _parallel versions_ of a given node (i.e. on the node itself instead of its parent)
(c) traverse ExpandedNodes in their own right, instead of traversing template instance nodes.  This would require tracking child relationships
    on ExpandedNodes, whereas we currently only track parent relationships

Exploring (b), because it's a natural fit with the way we traverse nodes & compute properties, (pass id_chain into compute_properties)
the next question becomes how do we perform tree recursion?  Who is responsible for enumerating the child/inner variants to 
recurse through?

it seems natural that a node should contain pointers to its children.
do we do this on the instance node? do we see-saw down instance => expanded_children, 

what if the expanded node tree exists independently of the instance node tree?

then instance nodes could contain `n` pointers for their `n` rendering children
where rendering recursion proceeds down the expanded node tree once it has started (no returning to `instance_node`)

so our mission becomes:
1. construct the expanded tree and
2. plant pointers to expanded nodes in instance nodes, 
or — what if we just traverse the expanded node tree entirely during rendering?

so 
(1) compute properties and build expanded node tree
(2) for rendering: traverse expanded node tree, calling lifecycle methods with the ExpandedNode-specific stamp of properties
(3) When ray-casting / etc., trigger events such that `self` is the ExpandedNode-specific stamp of properties  

If we separate the instance tree entirely from the expanded tree, we must solve:
    - How do we recurse into subsequent components during compute properties?
        One approach is to keep track of the `NodeType::Component`s that we find along the way, then handle them in order
        How do we stitch together the correct ordering of both nodes & z-index in this case?  z-index is effectively incremented during the post-order traversal of the expanded tree
    - How do we decide the entrypoint for recursing / rendering the expanded tree? (just the global root?  perhaps each component keeps track of its root ExpandedNode and thus we call the root's root to get started?)


Consider that instance nodes (template nodes? `TemplateInstanceNode`s?) represent a singular form
of a possibly expanded thing.  An instance node thus roughly (or exactly) maps to the node that
is authored in a template.

On the other hand, ExpandedNodes represent the fully realized permutations of all such TemplateInstanceNodes, with
properties already computed, ready to render

Currently, Repeat blurs these boundaries, creating `Instance`s for each of its children dynamically.
(This might be coupled to the way that we use components to manage Repeat's locals, e.g. `i`.
 Given that we are already working around this and special-casing it "we want your stack frame but not your component", perhaps
 we should reconsider this design, e.g. maybe Repeat explicitly pushes & pops the properties stack in the process of managing its children, instead of creating Component wrappers)

In an alternate world, instead of creating `n` instance trees underneath Repeat, there is only 1 instance tree
Then, that 1 instance tree is computed `n` times, creating `n` expanded node subtrees, in the process of [computing properties for Repeat?]
(or is this process external to Repeat, called by the properties computation workhorse, retrieving the dynamic expanded subtrees for Repeat
and computing properties on them? e.g. via `get_expanded_children` or `get_expanded_subtrees`)

We do know that Repeat is special.  It needs to somehow signal that it does `n` of things, where most nodes do 1, and Conditional does either 0 or 1.

Also, get_instance_children is already _quite close_ to `get_expanded_children` in practice.  E.g. Slot already handles this correctly, with the main difference that
we should be iterating over ExpandedNodes instead of dyn InstanceNodes (and again, in the current state of things where these are mashed into a single concept, that is essentially what we are doing.)

So what can Repeat do `n` of?
    - It can repeat `n` ExpandedNode subtrees, where the root of each is our ComponentInstance 
    - OR:
      What if `get_instance_children` returns a straight-up clone of the _SINGULAR_ `Component` instance at the root of For's template?
      Abiding by the idea that an instance node is stateless, and that e.g. its rendering is done as a function of a paired, stateful ExpandedNode,
      then all of the subtrees in Repeat's template can be expressed as a single instance tree, just like the structure of the template.

We _could_ have Repeat's `get_instance_children` return that clone `n` times.  The `ExpandedNode` representing the ComponentInstance at 
the root of Repeat's inner template becomes the stateful container of properties (e.g. the spectrum of `i` values in `for i in 0..5`)

The above is promising but still blurs the lines too much between instances and expandednodes.  If we return a singular instance
subtree from repeat, we can still "punch" that stateless instance `n` times with `n` different ExpandedNodes.  

The key is that we need to get `n` from Repeat.  
Maybe this is in the form of the integer value of `n`; maybe it's a Vec of `n` properties, or a vec of `n` ExpandedNodes.
Recall that `compute_properties` must be done in the context of some ExpandedNodes (to provide the stateful properties)

Repeat, for example, will render in the context of some ExpandedNode.  That ExpandedNode contains the stateful aspect of Repeat
(its calculated properties, e.g. the data vec).  Thus that stateful ExpandedNode represents the _current node_.

compute_properties, however, doesn't happen in the context of an ExpandedNode.
Either an ExpandedNode is created in the process of running `compute_properties`
Or some artifact of compute_properties is inspected in order to zip together the expanded tree

If we create ExpandedNodes in the process of running `compute_properties`, then we should return a single ExpandedNode, for this current node.
    The idea would be then to visit certain nodes multiple times, like the above idea of cloning Repeat's internal instance node and returning it multiple times
    for `get_instance_nodes` — but sliced slightly differently.
    Something like `get_dynamic_subtrees`, computed in the context
    of some computed properties (ExpandedNode + properties)

How do we visit an instance node `n` times for something repeated `n` times?
Could `ExpandedNode` have some property describing number of effective children?
Then when we get `ExpandedNode` back from Repeat#compute_properties, we check that value, like `ExpandedNode.NormalizedVecOfProps`
    This vec could contain the iterable data over which Repeat operates — becoming a seed for the `compute_properties` of each encapsulated ComponentInstance

Another possibility... Repeat attaches a "manifold" (in the sense of manifold in irrigation, a pre-assembled modular structure), a pre-assembled
tree of ExpandedNodes

What about something like:
`get_repeat_accounted_instance_children`

where most nodes just return their instance children
but Repeat clones its inner instance node `n` times
    It must also attach a properties stamp to each instance, in order to get values of `i` across
    Thus, again, we're really not cloning the instance_node at all; instead we are applying
    the spectrum of properties stamps `n` times to the one instance node.
    So: perhaps `Repeat` can be coaxed into a returning a set of ExpandedNodes, through something like
    `get_repeat_expanded_nodes(&self) -> Option<Vec<ExpandedNode>>`
    Most nodes return `None`.  Repeat, however, returns a Vec of ExpandedNodes, and when we get this (in the properties
    computation workhorse,) we go ahead and add it to the expanded node tree.
    Problem: we now want to recurse into _instance nodes_ to continue computation.  Does that work, linking from 
    expandednodes into their reference instance nodes, then continuing with compute_properties recursion for each subtree?  maybe it does.


So we manually create ExpandedNodes inside Repeat for its virtual children, as we need to specify the custom properties stamps
created from inside Repeat.

(if node.get_repeat_expanded_nodes().is_some() ...)
    Then we attach those ExpandedNodes to our tree, and continue recursing through `compute_properties_recursive` on the associated 
    


We must call `compute_properties_recursive` on the ComponentInstance(s) created inside Repeat
This will manage runtime properties stack
as well as create our ExpandedNodes

Can we simply call `compute_properties_recursive` `n` times?
What if we pick up `compute_properties_recursive` from _inside_ Repeat's `handle_compute_properties`
    Then we can manually call `compute_properties_recursive` `n` times over the same instance node — we just
    then need to be able to pass in some bit of state....
    Maybe we don't use a Component inside Repeat after all!!  Instead, we manually push & pop from stack, in between 
    iterations of calling `compute_properties_recursive` from inside Repeat.  This solves the problem of "passing in some
    bit of state" in the lines above, and cleans up the "special-casedness" of Repeat's internal Component

This might be circling back to the idea of managing our Repeat folding at the instancenode level, and we let go of
the constraint connecting templatenodes::instancenodes 1:1.  

How do we write back to properties when they're stored on [ephemeral?] ExpandedNodes
We make them not ephemeral.


Repeat has an inner Component
which is stateless
but adjoins to `n` ExpandedNode properties containing properties for `i` and `elem`


Does Repeat store `n` internal ExpandedNodes???
Perhaps calling `Repeat`'s `compute_properties` takes care of calling `compute_properties `


1. compute properties (call `handle_compute_properties`
2. Get an ExpandedNode in return; (and metadata for repeat arity/cardinality?)

2. get 0, 1, or many ExpandedNodes, each with properties attached
    3. Do these represent the parallel versions of this node, or its children / subtrees roots?
4. zip those onto our expanded tree
4.


What if `compute_properties` performs the side-effect of appending ExpandedNodes to the expanded tree?


#### Properties attached to InstanceNodes

Properties have the same definition <> instance dichotomy as nodes.  When a developer authors properties, they're doing so as "definitions".
One set of property definitions may become multiple instances, in the same way that one TemplateNodeDefinition becomes multiple ExpandedNodes.

This comes to bear in how we instantiate nodes.  `InstantiationArgs` has both a `PropertiesCoproduct` and a `CommonProperties` instance inlined.

Underneath the distinction of "Instance Nodes" vs "Expanded Nodes", there are properties attached to instance nodes
(those authored by the user, represented

Maybe we can just straight-up clone these properties?  The only place this starts to gets hairy is with expressions, and even
expressions are compiled exactly the same across multiple ExpandedNodes — only the input tuple changes.  So, a Properties clone
should both point to the same vtable entries (correctly) and give us cloned init values (correctly), which can be overwritten imperatively at runtime (correctly)

Let's store a prototypical clone of the original properties on each InstanceNode, then, and support getting it with something like
`get_instance_properties` + `get_instance_common_properties`.  Each new `ExpandedNode` can clone the Rc<RefCell<>>-wrapped version of each when initializing.




#### Preliminary Conclusions (which might become TODOs)

0. InstanceNodes are separated from ExpandedNodes.  ExpandedNodes are created/modified during `compute_properties_recursive`, and the ExpandedNode tree is traversed for rendering.
1. Every ExpandedNode gets a "permanent home" (in the "beehive" — in a persistent ExpandedNode.)
2. Every dyn InstanceNode keeps a hashmap of the Rcs of its "parallel selves", where keys are id chains and values are Rc<ExpandedNode>
   This allows us to call `get_id_chain` in the context of a given state of `rtc` and use that to retrieve the properties needed for e.g.
   for computing properties (must find existing record if it exists,) as well as the self passed to event handlers and ultimately called via e.g. `self.some_prop.set()`
3. We can still "stitch together" the ExpandedNode tree on the fly, via Rc::clone or Rc::downgrade (thus we have relationships between instance nodes and ExpandedNodes, and between ExpandedNodes and each other.)
4. Repeat only needs to keep a single instance subtree internally
   Ditch ComponentInstance entirely
   Instead, manage runtimepropertiesstack explicitly:
   During handle_compute_properties, 1. push stack, 2. recurse through inner instance root with `compute_properties_recursive`,  
5. When computing properties recursively: when we hit a component,
   (1) compute properties for its slot_children template, then
   (2) recurse and start dealing with _its_ component template frame (with a freshly scoped runtime stack)
    The above order is important because the slot children, which might require expansion via `for`, must be computed before
    they are prospectively used inside the second component in (2)
6. make sure z-indexing remains pre-order


### On computing properties + dirty DAG
Nov 2 2023

When we compute properties:

handle_compute_properties is responsible for returning in `ExpandedNode`.
that `ExpandedNode` either already exists (and should be retrieved, and updated as relevant)
or it doesn't yet exist, in which case it should be created, computed, and registered

we need access to the NodeRegistry here — and we have it: `ptc.engine.node_registry`
this allows us to upsert ExpandedNodes by id_chain
we also need the current id_chain: ptc.get_id_chain(self.instance_id);

once we create/retrieve an ExpandedNode, we need to determine whether each property is dirty
and then re-evaluate its vtable entry if so, storing the calculated value

When do we mark a property no longer dirty? after the entire pass?  We need to make sure we
traverse the DAG — which would require possibly many passes if we don't traverse that order intelligently
(e.g. if the last node traversed marks dirty the first node traversed.)

Perhaps we could traverse the dirty DAG entirely first?  And mark the nodes needing re-computation as dirty
The problem is, even if we do this, but still traverse property computation in-order, we run into an ordering problem,
because dependencies must be evaluated before dependents.
(worth double-checking:  are there some ordering guarantees baked into our domain, i.e. the way ancestors can reach into
descendent properties but not vice-versa, that let us take some kind of shortcut(s) here?)
Also worth noting — we have already had this problem!  It's almost certain that it takes up to `n` frames for properties computation to "settle"
when there are complex chains of dependencies, where those deps run contrary to the order of the tree traversal

Effectively, we need to topologically sort the DAG and evaluate properties in that order.  This suggests some sort of self-encapsulated "calculate myself"
logic on a property, which requires unpacking the PropertiesCoproduct in the context of the containing struct (unsafe_unwrap or similar)
and then computing the vtable value, storing in that unwrapped property, and making sure it gets back into the containing struct.  Currently,
the only reasonably elegant place to do that is in the `impl InstanceNode` block for a given InstanceNode, where we know what type we're dealing with.
If properties kept a pointer to their InstanceNode, then they'd be able to call out to a type-aware handler within that InstanceNode,
One challenge this introduces:  we can elegantly "stitch together" our ExpandedNode tree by visiting our instance nodes in rendering/recursion order.  Perhaps we still need to do this?
This suggests that properties compute becomes two differently shaped passes:
(1) traversal of DAG to compute properties, and (2) traversal of tree to stitch together ExpandedNodes.  
The order of these might best be reversed (2, 1), where newly inserted properties stamps are simply clones of the prototypical properties, marked dirty, and then computed in the DAG pass

This seems to converge on managing all properties in some sort of table, so that we can refer to that table and express edges (ROM-transistor-like in spirit)
One possible approach is to unwrap all properties into a table each tick (either sharing memory, which might get sketch, or unpacking & repacking like `with_properties_unsafe`)
Another possible approach is to wrap PropertyInstances in Rc<RefCell<>> instead of Box<>, which would solve the "unpacking" problem, because Rc<RefCell<>> gives us
exactly such a table.
- This seems feasible at first glance... (but see conclusion in this paragraph.)  We'd need to do some refactoring around `Clone` and `Default` and make sure we appease Rust's `orphan rule` (third party trait impl constraints)
but it seems like if we keep a propertyinstance table, then refactor all PropertiesCoproduct entries to keep pointers to those same properties instances,
instead of the instances themselves, then we
(As an aside... this would drastically normalize the shape of PropertiesCoproduct structs; they would all be effectively lists of smart pointers
instead of heterogeneous blobs of data.  I wonder if it would be feasible to eliminate PropertiesCoproduct entirely by this approach, using
HashMap<String, Rc<RefCell<PropertyInstance>> to manage the properties of a given ExpandedNode ... would we be able to `self.foo.set()`, though?
Indeed, the whole refactor to `Rc<RefCell<PropertyInstance>>` would harsh the ergonomics of userland properties.  Any .set or .get would require
wrangling the Rc<RefCell<>>, which would be unlikely to be a crowd-pleaser.  This seems to be an impasse regarding Rc<RefCell<dyn PropertyInstance>)
Our table could also be gatekept by instance or expanded nodes — in other words, the edges go between ExpandedNode, with a property id passed along as a param
Instead of computing properties in a recursive tree order, we compute them in the order of the property DAG
and instead of computing all properties on each visit, we only compute the one(s) parameterized (by ID, probably)
This requires ExpandedNodes to be able to address their properties by ID.  this could be brute-forced by unwrapping properties and calling `get_property_id` on each until we find a match — there's certainly a path to making this more elegant.

How do we assemble the dirty dag in the first place?  This is probably a compile-time concern — during expression compilation
we know the relationships between properties
(unpack what that means, to know the relationships between properties — can we boil this down to some sort of unique ID per property instance?)
(we probably do need some sort of unique ID per property — we have this with vtable ids, but we need these for literal properties too, because
they can be dependencies in dirty DAG)
We _do_ already have unambiguous relationships between symbols and properties at compile-time — we resolve each symbol such a `self.foo` statically to determine e.g. the stack offset. We could
elegantly register `self.foo` (the property found when we are determining stack offset) as a dependency of the expression in question at this time, and forward that graph to runtime. (probably codegen a HashMap<u64, Vec<u64>> as the "ROM" LUT of this graph.)
Dirty DAG is baked at compile time, and edges cannot be _added_ (because expressions require compilation,) but they can be _removed_
(because you can .set an expression value to a literal.  ways this might bend:
- if we support reverting to original value, which would restore the vtable ID + expression definition
- if we support interpreting PAXEL, which would enable runtime expression definitions and would require runtime manip. of dirty-DAG)
So at compile time, when we compile an expression (because only expressions will have dependencies), when we resolve a symbol like `self.foo`, we
need to establish an edge on the _instance_ (PropertyDefinition) graph
Then, when we expand those definitions into instances (roughly, during the act of copying the "prototypical properties" into ExpandedNodes)
we need to map those instance edges into expansion-aware edges
Double-check this:  is there ever a case where a "parallel version" of a property can be dirty (and require downstream updates) independently
of its other parallel versions?   One case that comes to mind is tactical / addressed changes e.g. to a vec of data, where some Repeat subtree should be updated while the others are not.
Another variant is a Component inside a Repeat, where that Component has a simple instance representation, but has completely independent ExpandedNode + properties instances.  This latter case is clearer,
that those _expanded_ property instances are indeed what should be "DAGged" between, as opposed to the definitions.
It follows, then, that we would need to expand the static instance dag to an expanded dag.

When do we mark properties "clean" again?
Probably after the DAG pass.  Then, when properties may be set imperatively during subsequent lifecycle methods, they are marked `dirty` along the way, and by the next tick
they're accurately considered `dirty` for that DAG pass.

How do we handle built-ins like `$tick`?  Perhaps we handle these as special kinds of dyn PropertyInstance (`PropertyBuiltin`?)
We would need to figure out if they also sit on some kind of synthetic ExpandedNode, or whether we have another way to address these (provided that the properties-update DAG passes through ExpandedNodes as its nodes)
Either we would set $tick on every frame from a special spot in the lifecycle (whereby the act of `set`ting would mark it dirty), or we would

Native patches;
keep a `patch` object on the ExpandedNode (??) the problem with this is patches are polymorphic and ExpandedNodes are currently type-blind.  This would mean doing something like  
PropertiesCoproduct or dyn Any for storing patches
If we can solve this reasonably, though, we can keep a running patch as a register for freshly computed values, punching new values onto it as they're computed
we could also keep a flag `needs_to_send_update_patch`, which we set true any time we punch one of these values, and then we set false again after sending the patch

How far can we get without dirty DAG?  Re-compute every property on every tick, like we do today
And continue to keep a cache of `last_patches`, but move them over to the stateful ExpandedNode#properties ?
(we might also be able to get away without last_patches, and just sending a firehose of `*Update` messages
across native bridges — this would reduce the need to double down on last_patches hacking, and would elegantly
be solved with dirty dag, but it might introduce unacceptable perf drag in the meantime, e.g. especially if either of
DOM or SwiftUI doesn't elegantly handle properties constantly "changing" to the same value )

Decision: remove last_patches for hacked dirty-checking; send native patches greedily; step back into dirty-DAG as the optimization mechanism if we find the flood of native patches incurs too much of a perf hit
We still need a stateful mechanism for storing the "patch-in-progress" as we compute properties.
We could couple this to the computation of a property.  I.e. as we compute a property (as we do in one sweeping pass, pre-dirty-DAG) we populate that computed property to the patch.
As long as we're committing to sending that patch every frame, without checking dirtiness, then we should populate all literal values along the way, too, as they may have been changed during handlers.
Since we're going so far as to keep a patch-in-progress on the ExpandedNode, it would be pretty straight-forward to keep a last-patch, too, to continue our hacked dirty-check...
(this could be a low-hanging alternative to full-blown dirty-DAG if indeed we hit a perf wall with firehose-patches.)

How do we keep our recursion-evaluated _stacks_ (stack frames, clipping & scrolling) intact for our DAG traversal?
Are these static enough that we can copy them onto ExpandedNodes ?
Stack Frames keep an `Rc<RefCell<dyn Any>>`, which is exactly a pointer to the owning-Component's PropertiesCoproduct (etc. for repeat)
The primary question is whether these stacks are _stable_ after an expansion, or whether we may need to perform some sort of surgery.
It seems like it's OK to copy them outright, keeping them immutable for the lifetime of an ExpandedNode

Considerations for ideal property management
Dirty DAG:
[ ] IDs annotated with each property definition
[ ] static property DAG, baked into codegen
[ ] expanded property DAG, id_chain
[ ] mechanism for computing properties atomically, plus addressing mechanism (either per-component `property-id-as-address : re-compute method` "property computers", or per-property addressing / table / RcRefCells
    Consider async: 
        channel property containers, where does data live?
        explore threading + wasm support; can e.g. network requests be non-blocking via unobtrusive threads?  instead of biting off async (userland `async` ergonomics and tooling complexity surface area feel like the points of friction here)


### On tracking ExpandedNode parent-child relationships
Nov 7 2023

(Started as a code comment on a would-be addition to NodeRegistry for tracking ExpandedNode parent/child relationships independently of the node instances (as a relational LUT)

Specifies relationships between expanded nodes — intended to be transient, relying on the persisting
nature of ExpandedNodes and the consistency of `id_chain` to reassociate child relationships after possibly clearing them (e.g. on every tick.)

Clearing them would get us clear & clean mutation handling under if / repeat, without having to perform surgery on existing
relationships
It would also let us short-circuit properties subtrees that don't need to be calculated (vs. having lingering ExpandedNodes
if the child relationships persist.)

Do we really need this, though?  The robust alternative may be to search through the on-ExpandedNode `children_expanded_nodes`
for a particular id_chain to decide whether an ExpandedNode needs to add itself to a parent (e.g. in the case where a Repeat list expands.)
"Perform surgery" each time we recurse_compute_properties, to see whether the ExpandedNode needs to be registered as a child of its parent

One other trade-off is compute: recalculating child relationships each tick adds a CPU burden even for an idle program, vs. performing surgery
reaches a steady low-draw state on idle

With dirty-DAG, we really only need to be handling the _creation_ of ExpandedNodes — expanding / contracting the tree as necessary —
while the _computation of properties_ happens on a separate pass.  In that world... do we even need parent/child relationships?  (yes — this is exactly used by rendering.)
So: expand the tree, keep track of parent-child relationship

In most cases, ExpandedNodes are totally static.  We could cache them aggressively.
_Only_ for Repeat and If are the child ExpandedNodes subject to changing. (or via def. changes, e.g. HMR or runtime mutation API.)
In fact, we might be able to _expand_ ExpandedNodes exactly once, at program init, and then "perform surgery" by API explicitly for the Repeat and If cases.

Thereafter, instead of "expanding tree and computing properties," we just "compute properties."  The pre-dirty-DAG shape of this
would be "just recurse through the tree computing properties." Thereafter, with dirty DAG, we traverse the DAG (when needed) instead
of traversing the tree top-down.
(Maybe we get this already in effect though by getting-or-creating ExpandedNodes as we go.  So there's really no harm in coupling these.)

***Or maybe the entirety of the current `recurse_compute_properties` method is a one-time init function???***

As long as component-side `handle_compute_properties` have the ability to "perform surgery" on parent/child relationships as needed by conditional + repeat,
then Repeat and Conditional can keep the ExpandedNode tree updated via DAG

Note that the surgery is "append-only."  We could later come back for some sort of explicit "garbage collection" as a feature, but as long as we don't require _removing children expanded nodes_ when e.g. Repeat contracts or If turns off,
then we have an elegant solution for unmounting / mounting (mark subtree unmounted, continue to compute properties, but skip rendering entirely.)

Note that currently EXPANSION and PROPERTIES-COMPUTE are tightly coupled.  In a dirty-DAG world we need to decouple these, and we should consider whether to do so also in a pre-dirty-dag world too.

Repeat, for example, needs to affect expandednodes (parent/child relationships) as part of its compute_properties.

In dirty-dag world, we probably need each property to be atomically computable, instead of the current "all properties are computed together in compute_properties" approach.
This also suggests that all properties need to be uniquely identifiable and addressable in a single place (some sort of PropertyInstanceTable)
This still runs up against the ergonomics wall of being able to .get and .set. (presuming the instances are wrapped in Rc<RefCell<>>)
There must be some way to achieve tactical updates of properties in a generic, trait-wrappable way...
Maybe we could address properties globally a la id_chain?  and an instance_node knows how to translate that address into an atomic compute method?  (in the context of a specific expandednode / stateful property container, of course)
intuitive ways to address properties via their containing struct (ExpandedNode or PropertiesCoproduct instance)
- index
- string id
- memory offset??? (`repr(C)`)

Statically we know the property names that we're updating; we manually run through these in the boilerplate of `compute_properties`.  Perhaps, then, there's a
natural place to introduce a static LUT of sorts, for string id => atomic compute_properties function.

```
properties_computers = HashMap::from(vec![
    ("stroke",|properties : &mut Ellipse|{
        handle_vtable_update!(properties.stroke, PropertiesCoproduct, Stroke);
    }),
    ("fill",|properties : &mut Ellipse|{
        handle_vtable_update!(properties.fill, PropertiesCoproduct, Fill);
    }),
])
```

These properties_computers could be derived straight-forwardly from a struct (ultimately as part of derive(Pax))
Main drawback is introduction of string properties IDs as runtime footprint overhead
Dirty DAG, too, would need to address these properties by string ID, introducing duplicate overhead
Instance dag addresses (instance id, property string id)
Expanded dag addresses (id_chain, property string id)

How does dirty-dag manage multiple dirty properties? Either the "lightning bolts" of downstream updates happen immediately, or they're enqueued
It's possible that expression C depends on both properties A and B, and both A and B have changed.  Either we recompute C twice (immediate lightning bolts) or the operations
are enqueued somehow and possibly de-duplicated.  Upon writing this out, it feels like an exotic feature / optimization.  Recomputing C twice, immediately, should be fine.

So: compute properties _once_ at tree expansion time
Then, only recompute properties (a) imperatively, e.g. manually by a developer with a `.set` method, or (b) as an immediate downstream effect of an upstream property `.set` (via DAG)

One more important thing to sanity-check while we're speccing this out:
async + channel-based property setting (as opposed to directly setting on the containing property object itself)
Note that with the channel approach, we would have the ability to indirect between a `.set` method (on the thin channel object) and an unwrapped `.borrow_mut().etc` on an Rc<RefCell<PropertyInstance>> e.g. in a centralized propertyinstance table.
This would afford an alternative to `properties_computers` — without the string lookup / association overhead, allowing simple int nodes on the DAG — while still providing
simple .get/.set ergonomics to the user.

(The above could ostensibly be optimized to use int ids instead of strings, requiring some sort of careful sequencing of application for int IDs => properties)

If we charge forward here without dirty dag:
1. keep properties compute and node expansion coupled; plan to re-run every frame (get-or-create expandednodes provides a hot path caching mechanism, at least)
Figure out a hacked solution to parent / child relationships given a world where we re-run tree expansion every tick (check for presence in parent's child vec before appending; possibly opt. with a hashset alongside children_expanded_nodes for constant lookups)
2. continue to allow "custom management of children" in the context of node expansion (manages_own_propertie_subtree)
3. clone our clipping/scrolling/runtimeproperties stacks onto each expanded node, permanently (the Rc<RefCell<>>s hold exactly the right instances of e.g. properties for stack frame)
4. on each tick, compute_properties for the entire tree from root (clumsy alternative to dag)


### On repeat indices and a prospective `key` property
Nov 8 2023

On the topic of repeat, it's also worth deadend-scoping `key`-like functionality a la React.  Namely, the naive implementation will consider a node unique via its index under the repeat source vec/range.
React offers a provision for the developer to specify the unique identity of a repeated node,

Here's an example of `key` from the React docs:

```
<ul>
  {props.posts.map((post) =>
    <li key={post.id}>
      {post.title}
    </li>
  )}
</ul>
```

and this relevant snippet of doc:

> When children have keys, React uses the key to match children in the original tree with children in the subsequent tree.
from https://legacy.reactjs.org/docs/reconciliation.html#keys

Is this relevant to us?  The primary cases for this are _insertions_ (where subtrees should be left alone if their identity / data is unchanged) and _reorders_ (converging on a case of insertion.)

A naive approach would be to recompute all of Repeat's children / properties / subtrees on a data list change (insertion, deletion, change in foo or bar in `foo..bar`)

What happens to the "identity" of a node if a repeat index changes?  Most notably, this would introduce a "shift" in id_chain — if we're repeating over [a,b,c]
and we insert `d` such that we're iterating over [a,d,b,c]
then `d` maps to the ID chain that used to be associated with `b`.  Let's say the for loop is `for x in my_vec { ... }` we need to ensure that `x` is set to the new value, `d` for the subtree represented by that id_chain.
In a world with expanded dirty DAG, this should be straight-forward enough.
In the naive approach, we do the same for `b` and `c`, because their id_chain vs datum relationships have both shifted.  This certainly incurs more property computation work than
Is there a path to a less naive approach?
- `key` becomes a common property, either a u32 or a String, and could be set anywhere, though is only read in the context of the top-level children of `Repeat`.
- `key` _might_ become a member of the id chain!!  instead of the default int-index Vec<u64>.  Would need to figure out how to consolidate possible string key-ids with those u64s.  
  one possible approach could be to shift u64 => i64, and to associate negative values with some sort of string LUT.  So -1 would map to "foo", which the user passes as key, and -1 gets inserted into the id_chain instead of the repeat index.
  This would be portable across the native bridge; too — id_chain remains globally unique/identifying, but the implementation details of the lookups / negative numbers are encapsulated by Repeat.


### On `if` ... `else if` ... `else`,  if-else ladders
Nov 8 2023
(Deadend-scoping for properties engine refactor)
We can probably update the grammar to match an `if_ladder` as an if statement, (some inner statements), some number of else-if statements, (some inner statements), and zero or one else statements.
We could then model `ConditionalProperties` to have a Vec or linked list of conditional properties (one for each arm) along with a flag describing the kind of arm.  Finally, during properties computation for Conditional,
we can map this data into imperative Rust if/else if/else statements, like we do with Repeat & friends.



#### Second pass, TODOs:

[x] Refactor Instance vs ExpandedNodes
    [x] Rename dyn RenderNode => dyn InstanceNode
    [x] Move properties to ExpandedNodes
        [x] Also move `CommonProperties` into `ExpandedNode`
    [x] Refactor properties lookup: requires awareness of expanded node / id_chain
        [x] Same with `get_common_properties`
        [x] Move `get_properties` and `get_common_properties` to be methods on `ExpandedNode`
    [x] Upsert `ExpandedNode` during `recurse_compute_properties`, along with computed properties "stamp".
        [x] Refactor `handle_compute_properties` to return a `Rc<RefCell<dyn Any>>`.  This is elegantly compatible with upserting (return clone of existing or return new)
            [x] Refactor one handler to figure out quite what this looks like on the handler side
            [x] Refactor all remaining handlers to match
        [x] Continue to store `ExpandedNode`s in registry; rename to `NodeRegistry`; decide whether also to populate pointers to `InstanceNode`s
        [x] Stitch together `ExpandedNode` tree — including relevant `Weak` parent <> child relationships — during recursive property computation
    [x] Refactor everywhere we call `get_properties` — pass an ID chain, possibly move to InstanceRegistry instead of component (otherwise, track ExpandedNode pointers inside instance nodes.)
        Decision: move `get_properties` to ExpandedNode
    [x] Manage wrapping/unwrapping polymorphic properties (PropertiesCoproduct) via `get_properties` and individual `dyn InstanceNode`s
        [x] Decide how to manage shared mutability.  Probably we need to wrap in Rc<RefCell<>>, but 
            (1) this requires special handling with unsafe_unwrap! — especially, this introduces surface area for disappearing properties for any primitive that doesn't "repack" a PropertiesCoproduct after `taking` from the Rc<RefCell<>>.
                this could be guard-railed with some sort of macro that takes, executes a block with the taken PropertiesCoproduct in scope, then repacks after the block
            (2) this also requires special handling around invoking userland event handlers, where `&mut self` _is_ a given properties stamp.  Essentially we will need to 
                `take` from the Rc<RefCell<>>, then `replace` back in after the user has presumably mutated it
        [x] build `with_properties_unsafe` macro
    [x] Manage cloned vs. shared/mutable state with `ptc`
        Because we are sharing a single mutable `ptc`, we must set back to `current_instance_node` after
        recursing, because it will be rewritten on each pass.
        Is there a cleaner way to handle this?
            - Clone `ptc` (this gets sticky around shared NativeMessage queue; could Rc<RefCell<>> that)
            - Pass `this_instance_node` as a separate param, instead of mutating ptc.  Then each call site
             gets its own unique clone in the form of a function arg
            - This would probably extend to passing `this_expanded_node` to e.g. `handle_mount_handlers`, in the same way
        This extends beyond these two nodes — we also need to manage some stack representation of e.g. `current_containing_component` and its slot children.
            Cloning `ptc` is pretty clean, if we bite off an Rc<RefCell<>> for the native message queue + shared state
[x] Unplug most of pax_std to reduce iterative surface area
[x] clipping_ids & scroller_ids in property compute
      [-] 1. might not need them at the *Create stage; might be able to not worry about this
      [x] 2. could make clipping & scrolling containers responsible for their own properties_compute recursion, managing their stacks similarly to components + stack frames
          what would it look like to keep clipping / scroller ids on ptc?
          need to manage pushing / popping pre/post recursion
          need to be able to refer to this during rendering, probably be keeping "expanded" ideas of the clipping / scrolling stack (clones of the vecs of ids?) on each expanded node.
          Can handle tracking clipping IDs (used strictly for native-side) independently of pre/post-render lifecycle methods + rendering clipping stack manip
          [x] Manually manage properties-compute recursion for Scroller + Frame, like we do with Component & friends
      [-] 3. revisit whether mount/unmount should be in the rendering pass instead?
[x] Handled prototypical / instantiation properties
    [x] Store a clone of `InstantiationArgs#properties` (and `#common_properties`) on each `dyn InstanceNode`
    [x] expose appropriate trait methods — access only? or maybe strictly internal, no need for trait methods?
        start with internal; punch through trait methods if that becomes necessary
    [x] Hook into this when creating a new ExpandedNode — initialize ExpandedNodes with a clone of each of properties and commonproperties
[x] Fully split properties-compute from render passes.  Probably start rendering from root of expanded tree.
    [x] Refactor / separate `rtc` as relevant for this too, to help clarify the distinction between `properties compute` vs `rendering` lifecycle methods & relevant data
    [x] Refactor file boundaries along the way
[x] Refactor Repeat & Conditional properties computation
    [x] Refactor each of `Repeat`, `Slot`, and `Conditional` to be stateless (so that stateful expansions with ExpandedNodes actually work)
        [x] Figure out in particular how to store:
            [x] Repeat's cache (+ source expression?)
            [x] Slot's `cached_computed_children` (+ index expression?)
            [x] Conditional's computed state (+ conditional expression?)
        [x] As part of the above, decide whether to refactor Conditional/Repeat/Slot's "special" PropertyInstances into
            PropertiesCoproduct variants.  Major deciding factor should be whether we need `dyn InstanceNode`-level access to prototypical properties (suggesting refactor)
            vs. encapsulating management of prototypical properties => ExpandedNodes so that it's an implementation detail of the respective node.  Given that we need to 
            "accordion" the properties of any node, including control-flow nodes, and that those expanded properties must
            sit on ExpandedNodes as PropertiesCoproduct, we either need to refactor control-flow properties to fit into PropertiesCoproduct (cleaner),
            or special-case control flow properties as Optional fields on ExpandedNodes (similar in shape to InstantiationArgs)
        [x] Punch through the above refactor into codegen, templates, and specs
            [x] In particular, manage control flow + instantiation args; pack into PropertiesCoproduct instead of special flags
            [x] Keep an eye on doing this for Scroller, too
        [x] Refactor internals of control-flow to be stateless + expansion-friendly, patching into the new PropertiesCoproduct variants:
            [x] Repeat
                [x] Remove ComponentInstance from Repeat
                [x] Instead, manage RuntimePropertiesStackFrame manually, as well as recursing into `compute_properties_recursive`
                [x] Think through mounting / unmounting:
                    When a Repeat list contracts, we want to mark any of the culled nodes as marked for unmount.
                    We can leave the ExpandedNodes sitting around and "garbage collect" them later.  Dirty-DAG should be able to address any latent CPU burden here.
            [x] Conditional
                [x] Piecewise-recurse into `compute_properties_recursive`, a la Repeat
                [x] Ensure that we don't render if an ExpandedNode is `marked_for_unmount`.  There still will likely exist an ExpandedNode, so we must be sure not to render these nodes.
                [x] Think through mounting / unmounting:
                    On the falling edge, when a Conditional goes from true to false, we know that there's a subtree of expandednodes, and that they should be unmounted.
                    One way to handle this is by allowing recursion to continue through the dead subtree, with the `marked_for_unmount` flag active.  Since this all happens in the properties-compute pass,
                    this should take effect immediately, this frame.  This seems to handle Conditional well, since there's only one subtree
                    Resetting child expanded nodes: since we are building the ExpandedNode tree from scratch each tick (for now,) we probably should reset all child relationships each tick.
                    Currently the children are tracked by the property `children_expanded_nodes` on ExpandedNode.  This could be externalized into a separate structure (easy since these are already Rcs,
                    and easy to purge in a single spot at the beginning of each tick.)  Something like `HashMap<Vec<u64>, Vec<Weak<RefCell<ExpandedNode<R>>>>>`
            [x] Slot
                [x] Apportion slotted children during properties compute.  This is done by how we stitch ExpandedNodes — grab from flatted pool of current_containing_component#get_slot_children
                    For a given `i` in `slot(i)`, grab the `i`th element of the pool and stitch as the `ExpandedNode` child of this `slot`'s ExpandedNode.  
                [x] Make sure we handle "is_invisible" (née flattening) correctly
            [x] Component - manually manage properties calc recursion + runtime properties stack
            [x] Scroller & Frame - manually manage properties calc recursion + scrolling + clipping stack
[x] Sanity-check id_chain + tree ambiguity + surface area for incorrectly unwrapping (unsafely) properties
    Broken case?
    ```
              foo
            /      \   
         if       repeat
         |          |  
       repeat     bar
         |
        baz
    ``` 
    In the above scenario, you could imagine each of the repeats "shifting" such that a given id_chain is suddenly ...
    No - because the first int in the id_chain is the instance_id, which is married to a specific instance and globally unique
    So the repeat_indices component of the id_chain (indices after the zeroth) could become ambiguous in the tree above
    But the full id_chain remains sound.
[x] Refactor "component template frame" computation order; support recursing mid-frame
    [x] Handle slot children: compute properties first, before recursing into next component template subtree
[x] Refactor `did_` and `will_` naming conventions, drop where unnecessary (e.g. mount/unmount)
[x] Revisit cleanup children — may not be necessary in light of changes to properties compute, instances, and control-flow-internal-recursion of properties computation + node expansion
    [x] Start by ripping it out
    [x] Come back and rebuild if/as needed
[x] Refactor unmounting to be id_chain-specific rather than instance_id specific
    [x] Refactor NodeRegistry
    [x] Refactor lifecycle hooks/call site for unmounting
        [x] Revisit how we `mark_for_unmount`.  We currently special-case this inside Repeat and Conditional.  Rather than require Repeat and Conditional to deal with stateful caches / diffs
            of their own subtrees, we could instead track the entire expanded tree.  If, after finishing properties computation, there's not an expanded node with
            (this could vaguely be considered a garbage collector of sorts.  If an ExpandedNode isn't used any more, we manage unmount.  We might even be able to handle
            this by hooking into `Drop` on `ExpandedNode`!  However, this would be error prone and require very careful management of ExpandedNode instances.  Probably
            better to be explicit on this one, feels like "C++ operator overloading"-style footgun potential.
    [x] and mounting
        [x] decide: properties pass or render pass
            consider that we use mount / unmount to trigger native CRUD ops.  This suggests they should happen before & after properties pass, respectively (otherwise we'll get *Update events at the wrong times)
            Perhaps: remove pre- and post- for properties compute.  Only Component was using this.  Instead, introduce some flag on the InstanceNode trait e.g. `handles_custom_subtree_for_properties_compute`
            Repeat, Component, Conditional, and Slot can return `true`.  And then during Properties compute pass, instead of traversing into these subtrees, the workhorse method allows these nodes to do their own piecewise recursion
            (Note more established term: "indirect recursion" instead of "piecewise recursion.")
            Now we only need a `handle_compute_properties` lifecycle method.  How does that interact with `mount`?
            `mount` requires properties already to have been computed once, with their default values.  This _could_ happen right after upserting, when a prototypical copy is applied.
            This would allow `mount` to be the very first thing to fire.  This would give the benefit of "we can enqueue *Create methods before we enqueue *Update methods"
[x] Refactor `get_rendering_children` => `get_instance_children`, plus ensure there's a means of traversing expanded children
    Perhaps we just assume one ExpandedNode per visit of an InstanceNode (meaning we iterate+recurse through `get_instance_children` in `recurse_compute_properties`).  Then,
    the only places we get many or zero expandednodes is within Conditional + Repeat.
[-] Decide: `ExpandedNode` vs. `RealizedNode` vs `ComputedNode`
    Expanded is pretty clear, even if imperfect vis-a-vis its relationship to InstanceNode
[x] Port sizing / bounds calculations over to the properties pass
    (1) compute native patches is dependent on these calculations
    (2) intuitively, these are property calculations, and rendering could/should be a pure function of these (just like all other properties)
    (3) bounds / size / etc. will be members of dirty-dag, and knowing if they change will be important for evaluation thereof
[x] dyn Any refactor
    Relieves need for PropertiesCoproduct and TypesCoproduct entirely
    Removes unsafe_unwrap, unsafe_wrap
    Risk: need to ensure it's compatible with Rc<RefCell<>> (probably is if done right)
    Simplifies build: no longer need to generate propertiescorproduct or deal with it in dependency graph
    Risk: consider dynamic linking & dyn Any
    [x] Scope out dyn any vs. debuggability / velocity with current thrust
    [x] execute dyn Any refactor
    [x] static code
    [x] codegen
    [x] achieve run
[x] unit tests for unsafe macros
    [x] draft as examples
    [x] vnext as stand-alone tests, create -> mutate -> assert flows



### `tab` and slot children

currently, slot_children's properties are calculated entirely in the context of the component template that owns them
This introduces wrinkles around `bounds` (`TransformAndBounds`, more specifically.) — we want bounds to be calculated in the context of the "rendering" container — the slot that accepts the slot_children.
    1. go back to calculating `tab` on-the-fly during render? instead of pre-computing during properties compute
        This means we unpack what e.g. "100%" or "50%" means lazily, at the time of rendering (this is probably best)
        This should robustly solve the slot transposition / container problem
        Any drawbacks?
    2. as a light alternative to this, introduce a separate traversal of the ExpandedNode tree in between expansion & rendering, something like `compute_rendering_state` or `compute_layout`

There are two kinds of properties computation that we want to happen in the context of the rendering tree
 -- or at least in the _same order_ as that used by rendering the tree.
 (1) z-index
 (2) TransformAndBounds calculation
 Either we calculate these on the fly as we render (easy, but unclean "side-effect free" division between rendering / computation)
   Incurs expected baseline compute cost on every tick
 Or we visit the tree on a separate recursive pass before rendering, performing + caching these values for "side-effect-free rendering"
   In the latter case, we could compute these values only when mutations occur, revisiting a subtree when necessary
 In the place we "stitch together" nodes, e.g. via Slot, we are traversing in the correct order... it may be that
 by allowing recursion to continue through slot, we would compute properties twice, but that they settle correctly?
   Properties probably would not settle correctly in the above case because they would be recomputed in the context
   of the incorrect containing component (the second time) undoing the special-casing we did for computing them in the context of their
   containing template in the first place.
 Perhaps the very cleanest approach is to have a separate `compute_rendering_properties` pass, specifically for z-index and
 TransformAndBounds, which fires after recurse_compute_properties, which visits ExpandedNodes in render order, and which caches these values onto ExpandedNodes.
 This method could be called tactically on sub-trees in a more optimized world, e.g. after an insertion


### ExpandedNodes and StackFrames

TreeFrame ?

We want each node to keep a pointer to its relevant stack frame
That stack frame should be able to traverse to its parent

Since that 'stack' may no longer exist in a sequentially evaluated world, but we still care about
these parent relationships, this structure may be more 'tree' than 'stack.'

Do we need to keep all stackframes in a table somewhere?  Or can these exist more ephemerally?

Table is nice because allocation is explicit (even though Rc does its own decentralization)

1. when pushing to the runtime stack, create an entry in a shared table (write-only? or perhaps keyed by id_chain of the creating entity, such as a Component or a For, such that the frame can later be mutated)
2. Maybe there's an even simpler way?  Instead of keeping a separate stack, allow ExpandedNodes to attach an optional RuntimePropertiesStackFrame to themselves (e.g. both For and Component would do this.)  
   An ExpandedNode can traverse its ancestral tree (linked list) to collect stack frames into a singular stack through which to evaluate an expression
   Back to the originally charted approach:  can that stack be pre-calculated / cached / Rc-cloned during expansion or properties computation?




instance_prototypical_properties_factory: Rc<RefCell<dyn Any>>,
instance_prototypical_common_properties_factory: Rc<RefCell<CommonProperties>>,

    pub prototypical_common_properties_factory: Box<dyn FnMut()->Rc<RefCell<CommonProperties>>>,
    pub prototypical_properties_factory: Box<dyn FnMut()->Rc<RefCell<dyn Any>>>,



### Filling out event handlers

1. create const list of all handler types
2. abstract `dispatch_*` in expandednode
3. abstract `handle_*` in instancenode
4. solve possible mismatch of properties being sent into handler

In the following codegenned example:

```
handler_registry.wheel_handlers = vec![
    |properties, ctx, args|{
        let properties = &mut *properties.as_ref().borrow_mut();
        if let Some(mut synthesized_self) = properties.downcast_mut::<fireworks::pax_reexports::Fireworks>() {
            fireworks::pax_reexports::Fireworks::handle_scroll(&mut synthesized_self,ctx,args);
        } else {panic!()}; //failed to downcast
    },
];
            
```

# Types
ExpandedNode {
    
}
# Tick Expansion Order


### Refreshing TODOs as of Nov 14 2023

[ ] Manual testing - examples as milestones
    [x] Get `one-rect` running correctly + Group
    [x] Get `fireworks` running correctly + event handlers
    [x] Get `color-grid` running correctly
[ ] Native patches
      [x] Figure out to what extent we need to hook back up hacked caching for various dirty-watchers.  Either make these caches stateful inside ExpandedNodes, or power through dirty-DAG
      [x] Decide (and enact) whether we continue to track last_patches, or whether we firehose update methods until dirty dag 
            ^ decided to hook back up last_patches
      [ ] Refactor and hook back up patches
[ ] `pax-std`: plug back in, update remaining primitives, test
[ ] Sizing & layout
    [x] Move `compute_tab` back to render pass, to solve slot transposition + ancestral bounds
    [ ] Revisit None-sizing -- now "whether a node is sized" is an instance-side concern, while "the current computed size" is an ExpandedNode concern
        [ ] Either remove None-sizing entirely — figuring out a better API for Group/etc. (default Size @ 100% might get us there; just create a `<Group>` and it fills its container)
            [ ] The above requires figuring out at least "vacuous ray-cast interception", which we current get around by checking whether size is None
                ^ this could be tackled with `is_invisible_to_ray_casting`, alongside `is_invisible_to_slot` (also decide whether we should negate => `visible`)
        [ ] or introduce a new instance-level distinction for whether `is_sized() -> bool`, for example
[ ] Refactor scroller
    [ ] Make instance node stateless
    [ ] Handle instantiation args => PropertiesCoproduct; remove ScrollerArgs
    [ ] Untangle instantiation args (dyn PropertyInstance) from stateful properties on ScrollerInstance
    [ ] Manage reset / offset transform calculation (formerly: `transform_scroller_reset`.
[ ] Clipping
    [ ] Hook back up computation (e.g. `get_clipping_size`)
    [ ] possibly power through to web chassis, plugging back in e2e clipping
    [ ] Figure out unplugged TransformAndBounds#clipping_bounds, possibly needed for viewport culling
[x] Occlusion
    [x] Hook back up "contiguous layer id" (nee z-index) — probably rename to `LayerIndex` instead of `ZIndex`
[x] Make sure element-level z-indexing is working correctly (incremented on pre-order)
[ ] Events
    [x] Abstract and macro-ize 
    [ ] Add remaining event handlers default impls to `trait InstanceNode`
    [ ] Resolve correct properties type to pass into event handler `synthetic self`
[ ] Dev tooling
    [ ] Debuggability; assess in light of latest .pax/pkg paradigm (or in dynamic linking paradigm)
    [x] Print instance / expanded trees and relevant metadata (can use for both debugging & unit tests)


###  TODOs as of Dec 13 2023 (Samuel Notes)

[x] Repeat cardinality check hack (shore up “native element churning”)
[ ] Slightly less naive canvas_id creation (seems like we shouldn't need an additional O(n) traversal? not a huge deal, but should be low-hanging fruit)
    ^ wrong order
[ ] Test Conditional <- need to restructure how mounted/unmounted is called
[ ] Test Repeat > Stacker > Repeat
[ ] Test other permutations of Conditional, Repeat, Components, Slots, and Primitives
[ ] Clipping
[ ] Robust native elements

Essentials:
[ ] Fix StringBox problem in nested components
[ ] Scroller

Other:
[ ] Improve repeat node handling (currently fully re-creates on each tick)
[ ] Implement slightly less naive version of canvas_id creation (keep track of a single "bounding box sum")
[ ] Create flowchart with order of property computations (for example, native
    patches needs to be sent after canvas_id has been defined, canvas_id will (later
    on) depend on computed tab) to improve stability. maybe introduce new lifecycle hooks such as "after/before properties compute"
[ ] move from recursive functions to a visitor system: https://docs.rs/derive-visitor/latest/derive_visitor/ (if it simplifies things)



### Experience building with Pax on Dec 22 2023

[ ] Haven't tried using Stacker yet (will wait for latest slot refactor to stabilize) — but there are a handful of places I would like to use it

[ ] Image needs to be plugged in (using text as placeholder for now)

[ ] Resizing window too small crashes, looks related to viewport culling math
    (maybe also related to combined % + px)

[ ] occlusion layers seem to be misordered —
    for repro: see `paxcorp` repo at commit `2f41d167`, change any background rectangle in pax-designer/src/controls/mod.pax to 1.0 opacity instead of 0.25

[ ] can't comment last line of a template file:
```
    //Final comment <EOF>
```
The above breaks, but adding an empty Group afterwards allows build to proceed:
```
    //Final comment
    <Group /> <EOF>
```

[x] Wanted to attach event handlers to repeated structs

```
    for panel_spec in panel_specs {
        <Group @click=panel_spec.action> //wishlist!
            <Image path={panel_spec.icon_path} />
        </Group>
    }
```
    this would require at least: handling `.` nesting in the grammar + parser, then adjusting invocation of handlers
    to support invoking functions-in-structs, instead of `SomeStruct::some_handler`
    [x] Way forward for now: instead of making the buttons / actions declarative as data structs,
        we can unroll Pax as manual <Group>, <Image>, etc. (no for loop, a bit more repetition)

[-] stack overflow with compiler when evaluating recursive structs, e.g. a ToolbarPanel with sub_panels: Property<Vec<ToolbarPanel>>
    (note that this probably only matters if we can also store Fn pointers on structs, otherwise there's no use using
    this "spec" pattern which requires recursive structs)


### Notes on hooking up designtime API + state transfer mechanism

pax-designer needs a handle on pax-designtime.
The two most obvious approaches are:
(a) import pax_designtime directly, which will require some build tooling tweaks (because of how we handle pax-* imports e.g. patching Cargo.toml to `.pax/pkg` paths)
    This would also be susceptible to fractured versions between userland project and the imported designtime
(b) import pax_designtime as reexported by chassis via cartridge. 

It seems likely we will need to statically link the userland project
We could theoretically dynamically link (e.g. for desktop builds) but that
won't work in the browser until WASI & Component Model.

This seems to lead to static linking via injecting an entry in pax-designer's Cargo.toml for the userland project, and probably aliasing it so we can refer to it cleanly throughout
the codebase (e.g. aliasing the crate as `userland_project`).
Probably we will want to edit pax-designer's Cargo.toml, injecting a path to the userland project in order to "open that project"

Then, we refer to e.g. `userland_project::designtime` from pax-designer, and can get a handle to the API, ORM, etc.

In terms of where the ORM data actually lives, we should likely attach it to the DesignerState object and ensure it's serializable,
so that we can transfer that state when we swap out a wasm slug

Detail: We may want to use something other than JSON for our state transfer mechanism, e.g. protobufs or flatbuffers.  It's probably worth
the disk footprint overhead for the designer, since it should significantly boost speed of state transfer
when reloading the designer




